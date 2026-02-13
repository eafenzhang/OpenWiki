use reqwest::Client;
use std::time::Duration;

const JINA_READER_BASE: &str = "https://r.jina.ai/";
const MAX_CONTENT_LENGTH: usize = 50_000; // ~50KB
const FETCH_TIMEOUT_SECS: u64 = 30;
const MIN_CONTENT_LENGTH: usize = 50;

pub struct UrlReadResult {
    pub content: String,
    pub title: Option<String>,
}

pub struct UrlReader {
    http_client: Client,
}

impl UrlReader {
    pub fn new() -> Self {
        let http_client = match Client::builder()
            .timeout(Duration::from_secs(FETCH_TIMEOUT_SECS))
            .build()
        {
            Ok(client) => {
                log::info!("UrlReader HTTP client created successfully");
                client
            }
            Err(e) => {
                log::error!("Failed to build HTTP client with TLS: {}, using default", e);
                Client::new()
            }
        };
        UrlReader { http_client }
    }

    /// Fetch article content from a URL via Jina Reader API.
    /// Returns markdown content on success.
    pub async fn fetch_content(&self, url: &str) -> Result<UrlReadResult, String> {
        let clean_url = url.trim();
        if clean_url.is_empty() {
            return Err("Empty URL".to_string());
        }
        let jina_url = format!("{}{}", JINA_READER_BASE, clean_url);

        log::info!("Fetching URL content via Jina Reader: {}", clean_url);

        let response = self
            .http_client
            .get(&jina_url)
            .header("X-Return-Format", "markdown")
            .send()
            .await
            .map_err(|e| format!("Jina Reader request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Jina Reader returned status: {}",
                response.status()
            ));
        }

        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read Jina response body: {}", e))?;

        // Check minimum content length
        if body.trim().len() < MIN_CONTENT_LENGTH {
            return Err("Content too short, likely not an article".to_string());
        }

        // Truncate if too long
        let content = if body.len() > MAX_CONTENT_LENGTH {
            let truncated: String = body.chars().take(MAX_CONTENT_LENGTH).collect();
            format!("{}...\n\n[内容已截断]", truncated)
        } else {
            body
        };

        let title = extract_title(&content);

        Ok(UrlReadResult { content, title })
    }
}

/// Extract the first markdown heading as the article title.
fn extract_title(markdown: &str) -> Option<String> {
    for line in markdown.lines().take(10) {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return Some(trimmed.trim_start_matches('#').trim().to_string());
        }
    }
    None
}
