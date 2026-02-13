use super::clipboard::ClipboardWatcher;
use super::screenshot::ScreenshotWatcher;
use super::sensitive_filter::contains_sensitive_data;
use super::url_reader::UrlReader;
use crate::commands::capture::{save_content_auto, AppState};
use crate::storage::database::Database;
use crate::storage::models::CaptureEvent;
use crate::storage::repository::Repository;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{AppHandle, Emitter, Listener, Manager};

/// Time window in milliseconds for deduplication.
/// If two capture events (e.g., screenshot file + clipboard image) arrive within
/// this window, only the first one is forwarded to the frontend.
const DEDUP_WINDOW_MS: u128 = 3000;

/// Tracks recent events to deduplicate overlapping captures.
struct DeduplicationState {
    /// Maps a dedup key to the time it was last emitted.
    recent_events: HashMap<String, Instant>,
}

impl DeduplicationState {
    fn new() -> Self {
        DeduplicationState {
            recent_events: HashMap::new(),
        }
    }

    /// Check if an event with the given keys should be emitted.
    /// Returns true if NONE of the keys have been seen within the dedup window.
    /// If any key matches a recently-seen key, the event is suppressed.
    fn should_emit(&mut self, keys: &[String]) -> bool {
        let now = Instant::now();

        // Clean up old entries to prevent unbounded growth
        self.recent_events
            .retain(|_, time| now.duration_since(*time).as_millis() < DEDUP_WINDOW_MS * 5);

        // Check if any key was recently seen
        for key in keys {
            if let Some(last_time) = self.recent_events.get(key) {
                if now.duration_since(*last_time).as_millis() < DEDUP_WINDOW_MS {
                    log::info!("Dedup: suppressing duplicate event for key: {}", key);
                    return false;
                }
            }
        }

        // Record all keys as seen
        for key in keys {
            self.recent_events.insert(key.clone(), now);
        }
        true
    }
}

/// Compute deduplication keys from an event's content.
/// Returns a list of keys. An event is considered a duplicate if ANY of its keys
/// match a recently seen key. This allows cross-source deduplication:
/// e.g., a clipboard image event (with dimensions) and a screenshot file event
/// (with a file path) both produce a dimension-based key, so the second is suppressed.
fn compute_dedup_keys(event: &serde_json::Value) -> Vec<String> {
    let content_type = event
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    match content_type {
        "image" => {
            let mut keys = Vec::new();

            // Key based on image path (if available)
            if let Some(path) = event.get("image_path").and_then(|v| v.as_str()) {
                if !path.is_empty() {
                    keys.push(format!("img:path:{}", path));
                }
            }

            // Key based on image dimensions (for cross-source dedup between
            // clipboard images and screenshot files of the same capture)
            let w = event.get("image_width").and_then(|v| v.as_u64()).unwrap_or(0);
            let h = event.get("image_height").and_then(|v| v.as_u64()).unwrap_or(0);
            if w > 0 && h > 0 {
                keys.push(format!("img:dims:{}x{}", w, h));
            }

            // Fallback: generic image key if no other keys produced
            if keys.is_empty() {
                keys.push("img:unknown".to_string());
            }
            keys
        }
        "text" => {
            if let Some(text) = event.get("raw_text").and_then(|v| v.as_str()) {
                // Use first 64 chars as dedup key for text
                let key_text: String = text.chars().take(64).collect();
                vec![format!("text:{}", key_text)]
            } else {
                vec!["text:empty".to_string()]
            }
        }
        _ => vec![format!("other:{}", content_type)],
    }
}

/// Async function to fetch URL content via Jina Reader.
/// Runs as a standalone async task so it has proper async runtime context.
async fn fetch_url_content(content_id: String, url: String, db: Arc<Database>, app: AppHandle) {
    log::info!(
        "Starting URL fetch task for {} (url={})",
        content_id,
        url
    );

    let reader = UrlReader::new();
    match reader.fetch_content(&url).await {
        Ok(result) => {
            let repo = Repository::new(db);
            if let Err(e) = repo.update_content_for_url(
                &content_id,
                &result.content,
                &url,
            ) {
                log::error!("Failed to update URL content: {}", e);
            } else {
                log::info!(
                    "URL content fetched for {}: {} chars (title={:?})",
                    content_id,
                    result.content.len(),
                    result.title
                );
                let _ = app.emit(
                    "content:url-fetched",
                    serde_json::json!({
                        "id": content_id,
                        "title": result.title,
                        "content_length": result.content.len(),
                    }),
                );
            }
        }
        Err(e) => {
            log::error!(
                "Failed to fetch URL content for {} (url={}): {}",
                content_id,
                url,
                e
            );
        }
    }
}

/// Parse JSON data into CaptureEvent and auto-save to database.
/// This is a module-level function (not nested) for better async task spawning.
fn handle_auto_save(app: &AppHandle, data: serde_json::Value) {
    let db = {
        let state = app.state::<AppState>();
        state.db.clone()
    };

    // Check if sensitive data filtering is enabled
    let repo = Repository::new(db.clone());
    let sensitive_filter_enabled = repo
        .get_setting("sensitive_filter_enabled")
        .ok()
        .flatten()
        .map(|v| v == "true")
        .unwrap_or(false);

    // If filter is enabled, check text content for sensitive data
    if sensitive_filter_enabled {
        if let Some(text) = data.get("raw_text").and_then(|v| v.as_str()) {
            if contains_sensitive_data(text) {
                log::info!(
                    "Sensitive data detected, skipping capture (source_app={}, preview={}...)",
                    data.get("source_app").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    &text.chars().take(20).collect::<String>()
                );
                return;
            }
        }
    }

    let event = CaptureEvent {
        content_type: data
            .get("content_type")
            .and_then(|v| v.as_str())
            .unwrap_or("text")
            .to_string(),
        preview: data
            .get("preview")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        source_app: data
            .get("source_app")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        raw_text: data
            .get("raw_text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        image_path: data
            .get("image_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    };

    match save_content_auto(&db, event) {
        Ok(content) => {
            log::info!(
                "Auto-saved: {} (type={}, source_url={:?})",
                content.id,
                content.content_type.as_str(),
                content.source_url
            );

            // For URL content, spawn background fetch via Jina Reader
            if content.content_type.as_str() == "url" {
                if let Some(url) = content.source_url {
                    // Check if URL reading is enabled (default: true)
                    let repo = Repository::new(db.clone());
                    let url_reading_enabled = repo
                        .get_setting("url_reading_enabled")
                        .ok()
                        .flatten()
                        .map(|v| v != "false")
                        .unwrap_or(true);

                    if url_reading_enabled {
                        log::info!(
                            "Spawning URL fetch task for {} (url={})",
                            content.id,
                            url
                        );
                        let content_id = content.id.clone();
                        let app_clone = app.clone();
                        tauri::async_runtime::spawn(fetch_url_content(
                            content_id,
                            url,
                            db.clone(),
                            app_clone,
                        ));
                    } else {
                        log::info!("URL reading disabled, skipping fetch for {}", content.id);
                    }
                } else {
                    log::warn!(
                        "URL content {} has no source_url, cannot fetch",
                        content.id
                    );
                }
            }
        }
        Err(e) => {
            if e.contains("Duplicate content") {
                log::debug!("Skipped duplicate content");
            } else {
                log::error!("Failed to auto-save content: {}", e);
            }
        }
    }
}

pub struct CaptureDetector {
    clipboard_watcher: ClipboardWatcher,
    screenshot_watcher: ScreenshotWatcher,
}

impl CaptureDetector {
    pub fn new() -> Self {
        CaptureDetector {
            clipboard_watcher: ClipboardWatcher::new(),
            screenshot_watcher: ScreenshotWatcher::new(),
        }
    }

    pub fn start(&self, app: AppHandle) {
        log::info!("Starting capture detector with auto-save...");

        let dedup_state = Arc::new(Mutex::new(DeduplicationState::new()));

        // Listen to clipboard events, dedup, then auto-save
        let app_for_clipboard = app.clone();
        let dedup_for_clipboard = dedup_state.clone();
        app.listen("capture:clipboard", move |event| {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                let keys = compute_dedup_keys(&data);
                let should_save = {
                    let mut state = dedup_for_clipboard.lock().unwrap_or_else(|e| e.into_inner());
                    state.should_emit(&keys)
                };

                if should_save {
                    handle_auto_save(&app_for_clipboard, data);
                }
            }
        });

        // Listen to screenshot events, dedup, then auto-save
        let app_for_screenshot = app.clone();
        let dedup_for_screenshot = dedup_state;
        app.listen("capture:screenshot", move |event| {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                let keys = compute_dedup_keys(&data);
                let should_save = {
                    let mut state = dedup_for_screenshot.lock().unwrap_or_else(|e| e.into_inner());
                    state.should_emit(&keys)
                };

                if should_save {
                    handle_auto_save(&app_for_screenshot, data);
                }
            }
        });

        // Start the underlying watchers (they emit to capture:clipboard / capture:screenshot)
        self.clipboard_watcher.start(app.clone());
        self.screenshot_watcher.start(app);

        log::info!("Capture detector started with auto-save enabled");
    }

    pub fn stop(&self) {
        log::info!("Stopping capture detector...");
        self.clipboard_watcher.stop();
        self.screenshot_watcher.stop();
        log::info!("Capture detector stopped");
    }
}
