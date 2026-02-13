use crate::ai::client::AiClient;
use crate::ai::preference_engine;
use crate::ai::prompts;
use crate::storage::database::Database;
use crate::storage::models::{ReportSection, WeeklyReport};
use crate::storage::repository::Repository;
use chrono::{Datelike, TimeDelta, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

/// JSON structure expected from the AI response
#[derive(Debug, Deserialize)]
struct AiReportJson {
    summary: String,
    sections: Vec<AiSectionJson>,
}

#[derive(Debug, Deserialize)]
struct AiSectionJson {
    title: String,
    body: String,
    section_type: Option<String>,
    relevance_score: Option<f64>,
    content_ids: Option<Vec<String>>,
}

/// Main entry point: generate a weekly report using the AI pipeline.
///
/// Steps:
/// 1. Query content from the past 7 days
/// 2. Build content summaries (truncate long texts to 500 chars)
/// 3. Get user preferences
/// 4. Build prompt from templates
/// 5. Call AI API
/// 6. Parse response JSON into WeeklyReport + ReportSections
/// 7. Save to database
/// 8. Return complete report
pub async fn generate_weekly_report(
    db: Arc<Database>,
    api_key: &str,
    provider: &str,
    model: &str,
) -> Result<WeeklyReport, String> {
    log::info!(
        "开始生成周报, provider={}, model={}",
        provider,
        model
    );

    // Step 1: Calculate the date range for the past 7 days
    let now = Utc::now();
    let week_end = now.to_rfc3339();
    let week_start_dt = now - TimeDelta::days(7);
    let week_start = week_start_dt.to_rfc3339();

    // Shorter date strings for the report record (YYYY-MM-DD)
    let week_start_date = week_start_dt.format("%Y-%m-%d").to_string();
    let week_end_date = now.format("%Y-%m-%d").to_string();

    // Step 2: Query all content from the past 7 days
    let repo = Repository::new(db.clone());
    let contents = repo
        .get_content_for_week(&week_start, &week_end)
        .map_err(|e| format!("查询本周内容失败: {}", e))?;

    if contents.is_empty() {
        return Err("本周没有保存任何内容".to_string());
    }

    let content_count = contents.len() as i32;
    log::info!("本周共有 {} 条内容", content_count);

    // Step 3: Build content summaries, truncating long text
    // URL content with fetched articles gets 1000 chars; regular content gets 500
    let mut content_summaries = String::new();
    for item in &contents {
        let is_fetched_url = item.content_type.as_str() == "url"
            && item.source_url.is_some()
            && item.raw_text.as_deref() != item.source_url.as_deref();
        let max_chars: usize = if is_fetched_url { 1000 } else { 500 };

        let text_preview = match &item.raw_text {
            Some(text) if !text.is_empty() => {
                if text.chars().count() > max_chars {
                    let truncated: String = text.chars().take(max_chars).collect();
                    format!("{}...", truncated)
                } else {
                    text.clone()
                }
            }
            _ => "[图片内容]".to_string(),
        };

        let line = if is_fetched_url {
            let url = item.source_url.as_deref().unwrap_or("");
            format!(
                "- [ID: {}] [url] 来自「{}」({}): [原文: {}]\n  摘要: {}",
                item.id, item.source_app, item.captured_at, url, text_preview
            )
        } else {
            prompts::format_content_item(
                &item.id,
                item.content_type.as_str(),
                &item.source_app,
                &item.captured_at,
                &text_preview,
            )
        };
        content_summaries.push_str(&line);
        content_summaries.push('\n');
    }

    // Step 4: Get user preferences summary
    let preference_summary = preference_engine::get_preference_summary(db.clone());

    // Step 5: Build the prompt
    let system_prompt = prompts::weekly_report_system_prompt();
    let user_message = prompts::weekly_report_user_message(&content_summaries, &preference_summary);

    // Step 6: Call the AI API
    let client = AiClient::new(
        api_key.to_string(),
        provider.to_string(),
        model.to_string(),
    );

    let ai_response = client
        .send_message(&system_prompt, &user_message)
        .await
        .map_err(|e| format!("AI 生成失败: {}", e))?;

    log::info!("AI 响应已收到, 解析中...");

    // Step 7: Parse the JSON response
    let response_text = ai_response.text.trim().to_string();

    // Strip potential markdown code block markers
    let json_text = strip_markdown_code_block(&response_text);

    let ai_report: AiReportJson = serde_json::from_str(&json_text).map_err(|e| {
        log::error!(
            "解析 AI 返回的 JSON 失败: {}\nResponse: {}",
            e,
            &response_text
        );
        format!("解析周报 JSON 失败: {}", e)
    })?;

    // Build the WeeklyReport and ReportSections
    let report_id = uuid::Uuid::new_v4().to_string();
    let generated_at = Utc::now().to_rfc3339();

    // Compute activity stats from content items (not from AI)
    let mut source_counts: HashMap<String, usize> = HashMap::new();
    let mut daily_counts = [0i32; 7];
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for item in &contents {
        *source_counts
            .entry(item.source_app.clone())
            .or_insert(0) += 1;
        *type_counts
            .entry(item.content_type.as_str().to_string())
            .or_insert(0) += 1;
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&item.captured_at) {
            let weekday = dt.weekday().num_days_from_monday() as usize;
            if weekday < 7 {
                daily_counts[weekday] += 1;
            }
        }
    }
    let mut top_sources: Vec<_> = source_counts.into_iter().collect();
    top_sources.sort_by(|a, b| b.1.cmp(&a.1));
    let top_sources_json: Vec<_> = top_sources
        .into_iter()
        .take(3)
        .map(|(app, count)| serde_json::json!({"app": app, "count": count}))
        .collect();

    let report_json = serde_json::json!({
        "stats": {
            "total_items": contents.len(),
            "topics_count": ai_report.sections.len(),
            "top_sources": top_sources_json,
            "daily_counts": daily_counts,
            "type_counts": {
                "text": type_counts.get("text").unwrap_or(&0),
                "url": type_counts.get("url").unwrap_or(&0),
                "image": type_counts.get("image").unwrap_or(&0),
            },
        },
        "raw_response": response_text,
    });

    // Sort sections by relevance_score descending before assigning sort_order
    let mut indexed_sections: Vec<(usize, &AiSectionJson)> =
        ai_report.sections.iter().enumerate().collect();
    indexed_sections.sort_by(|a, b| {
        let score_a = a.1.relevance_score.unwrap_or(0.5);
        let score_b = b.1.relevance_score.unwrap_or(0.5);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut sections = Vec::new();
    for (sort_idx, (_, ai_section)) in indexed_sections.iter().enumerate() {
        let section = ReportSection {
            id: uuid::Uuid::new_v4().to_string(),
            report_id: report_id.clone(),
            section_type: ai_section
                .section_type
                .clone()
                .unwrap_or_else(|| "topic".to_string()),
            title: ai_section.title.clone(),
            body: ai_section.body.clone(),
            relevance_score: ai_section.relevance_score,
            sort_order: sort_idx as i32,
            content_ids: ai_section.content_ids.clone().unwrap_or_default(),
        };
        sections.push(section);
    }

    let report = WeeklyReport {
        id: report_id,
        week_start: week_start_date,
        week_end: week_end_date,
        summary_text: ai_report.summary.clone(),
        report_json,
        content_count,
        model_used: model.to_string(),
        tokens_used: ai_response.tokens_used,
        generated_at,
        sections,
    };

    // Step 8: Save the report and sections to the database
    repo.save_report(&report)
        .map_err(|e| format!("保存周报失败: {}", e))?;

    log::info!("周报生成完成, ID: {}", report.id);

    // Step 9: Return the complete report
    Ok(report)
}

/// Strip markdown code block markers (```json ... ```) from a response string.
fn strip_markdown_code_block(text: &str) -> String {
    let trimmed = text.trim();

    // Check for ```json or ``` prefix
    let without_prefix = if trimmed.starts_with("```json") {
        trimmed.strip_prefix("```json").unwrap_or(trimmed).trim_start()
    } else if trimmed.starts_with("```") {
        trimmed.strip_prefix("```").unwrap_or(trimmed).trim_start()
    } else {
        trimmed
    };

    // Strip trailing ```
    let result = if without_prefix.ends_with("```") {
        without_prefix
            .strip_suffix("```")
            .unwrap_or(without_prefix)
            .trim_end()
    } else {
        without_prefix
    };

    result.to_string()
}
