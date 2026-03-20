use crate::commands::capture::AppState;
use crate::export::markdown;
use crate::storage::models::CapturedContent;
use crate::storage::repository::Repository;
use std::path::PathBuf;
use tauri::State;

/// Get the default export directory (~/Documents/Xiaoyun/).
fn default_export_dir() -> PathBuf {
    dirs::document_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join("Documents"))
        .join("Xiaoyun")
}

/// Resolve the export directory from settings or use the default.
fn resolve_export_dir(repo: &Repository) -> PathBuf {
    match repo.get_setting("export_dir") {
        Ok(Some(dir)) if !dir.is_empty() => PathBuf::from(dir),
        _ => default_export_dir(),
    }
}

#[tauri::command]
pub async fn search_content(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<CapturedContent>, String> {
    let repo = Repository::new(state.db.clone());
    repo.search_content(&query, 50).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_dates_with_content(
    state: State<'_, AppState>,
) -> Result<Vec<(String, i64)>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_dates_with_content().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_content_for_date(
    date: String,
    state: State<'_, AppState>,
) -> Result<Vec<CapturedContent>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_content_for_date(&date).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_day_markdown(
    date: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let repo = Repository::new(state.db.clone());
    let export_dir = resolve_export_dir(&repo);

    let path = markdown::export_day(&date, &repo, &export_dir).map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_all_markdown(state: State<'_, AppState>) -> Result<usize, String> {
    let repo = Repository::new(state.db.clone());
    let export_dir = resolve_export_dir(&repo);

    markdown::export_all(&repo, &export_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_date_range_markdown(
    start: String,
    end: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let repo = Repository::new(state.db.clone());
    let export_dir = resolve_export_dir(&repo);

    markdown::export_date_range(&start, &end, &repo, &export_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_export_dir(state: State<'_, AppState>) -> Result<String, String> {
    let repo = Repository::new(state.db.clone());
    let dir = resolve_export_dir(&repo);
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn set_export_dir(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.update_setting("export_dir", &path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_export_dir(state: State<'_, AppState>) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    let dir = resolve_export_dir(&repo);

    // Ensure directory exists before opening
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    std::process::Command::new("open")
        .arg(dir.to_string_lossy().to_string())
        .spawn()
        .map_err(|e| format!("Failed to open directory: {}", e))?;

    Ok(())
}
