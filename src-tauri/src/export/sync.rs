use crate::storage::repository::Repository;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct ExportSyncer {
    export_dir: PathBuf,
    last_sync: Arc<Mutex<Option<Instant>>>,
}

impl ExportSyncer {
    pub fn new(export_dir: PathBuf) -> Self {
        ExportSyncer {
            export_dir,
            last_sync: Arc::new(Mutex::new(None)),
        }
    }

    /// Called after content is saved. Debounces (300ms) then re-exports that day's markdown.
    pub fn on_content_saved(&self, date: &str, repo: &Repository) {
        // Check if auto-sync is enabled
        let auto_sync = repo
            .get_setting("export_auto_sync")
            .ok()
            .flatten()
            .unwrap_or_default();
        if auto_sync != "true" {
            return;
        }

        // Debounce: skip if last sync was < 300ms ago
        {
            let mut last = self.last_sync.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(t) = *last {
                if t.elapsed().as_millis() < 300 {
                    return;
                }
            }
            *last = Some(Instant::now());
        }

        // Export the day's markdown
        if let Err(e) = super::markdown::export_day(date, repo, &self.export_dir) {
            log::warn!("Auto-sync export failed for {}: {}", date, e);
        }
    }
}
