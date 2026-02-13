use crate::commands::capture::AppState;
use crate::storage::repository::Repository;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_all_settings()
        .map_err(|e| format!("获取设置失败: {}", e))
}

#[tauri::command]
pub fn update_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.update_setting(&key, &value)
        .map_err(|e| format!("更新设置失败: {}", e))
}
