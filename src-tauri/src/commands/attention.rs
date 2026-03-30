use crate::ai::attention_analyzer::{self, AnalysisProvider};
use crate::commands::capture::AppState;
use crate::storage::models::AttentionInsight;
use crate::storage::repository::Repository;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarStatus {
    pub status: String,
    pub insight: Option<AttentionInsight>,
    pub has_new_content: bool,
}

/// Get the current attention radar status and insight.
#[tauri::command]
pub fn get_attention_insights(
    state: State<'_, AppState>,
) -> Result<RadarStatus, String> {
    let repo = Repository::new(state.db.clone());

    // 1. Check if API key is configured
    let api_key = repo
        .get_setting("ai_api_key")
        .map_err(|e| format!("读取设置失败: {}", e))?
        .unwrap_or_default();

    if api_key.is_empty() {
        return Ok(RadarStatus {
            status: "no_api_key".to_string(),
            insight: None,
            has_new_content: false,
        });
    }

    // 2. Check if we have enough content (at least 1 item in the last 14 days)
    let content_check = repo
        .get_recent_content_for_analysis(14, 1)
        .map_err(|e| format!("检查内容失败: {}", e))?;

    if content_check.is_empty() {
        return Ok(RadarStatus {
            status: "not_enough_content".to_string(),
            insight: None,
            has_new_content: false,
        });
    }

    // 3. Get current insight
    let insight = repo
        .get_current_insight()
        .map_err(|e| format!("获取洞察失败: {}", e))?;

    match insight {
        None => Ok(RadarStatus {
            status: "empty".to_string(),
            insight: None,
            has_new_content: true,
        }),
        Some(insight) => {
            // Check if currently analyzing
            if insight.status == "analyzing" {
                return Ok(RadarStatus {
                    status: "analyzing".to_string(),
                    insight: Some(insight),
                    has_new_content: false,
                });
            }

            // Check if there's an error
            if insight.status == "error" {
                return Ok(RadarStatus {
                    status: "error".to_string(),
                    insight: Some(insight),
                    has_new_content: true,
                });
            }

            // Check if new content has arrived since the last analysis
            let has_new = repo
                .has_new_content_since(&insight.analyzed_at)
                .map_err(|e| format!("检查新内容失败: {}", e))?;

            let status = if has_new { "stale" } else { "fresh" };

            Ok(RadarStatus {
                status: status.to_string(),
                insight: Some(insight),
                has_new_content: has_new,
            })
        }
    }
}

/// Trigger a new attention analysis in the background.
#[tauri::command]
pub async fn trigger_attention_analysis(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use tauri::Emitter;

    let db = state.db.clone();
    let repo = Repository::new(db.clone());

    // 1. Check if already analyzing
    let current = repo
        .get_current_insight()
        .map_err(|e| format!("检查状态失败: {}", e))?;

    if let Some(ref insight) = current {
        if insight.status == "analyzing" {
            return Ok(()); // Already in progress, skip
        }
    }

    // 2. Read AI settings
    let api_key = repo
        .get_setting("ai_api_key")
        .map_err(|e| format!("读取 API Key 失败: {}", e))?
        .unwrap_or_default();

    if api_key.is_empty() {
        return Err("请先在设置中配置 AI API Key".to_string());
    }

    let provider_str = repo
        .get_setting("ai_provider")
        .map_err(|e| format!("读取 AI 提供商失败: {}", e))?
        .unwrap_or_else(|| "anthropic".to_string());

    let model = repo
        .get_setting("ai_model")
        .map_err(|e| format!("读取 AI 模型失败: {}", e))?
        .unwrap_or_else(|| "claude-sonnet-4-20250514".to_string());

    // 3. Get content for analysis (14 days, max 200)
    let items = repo
        .get_recent_content_for_analysis(14, 200)
        .map_err(|e| format!("获取内容失败: {}", e))?;

    if items.is_empty() {
        return Err("没有足够的内容进行分析".to_string());
    }

    let item_count = items.len();

    // Build id_map: index -> content_id
    let id_map: std::collections::HashMap<usize, String> = items
        .iter()
        .enumerate()
        .map(|(i, (id, _, _, _))| (i, id.clone()))
        .collect();

    // 4. Create "analyzing" record
    let now = chrono::Utc::now();
    let window_end = now.to_rfc3339();
    let window_start = (now - chrono::TimeDelta::days(14)).to_rfc3339();

    let insight_id = repo
        .save_attention_insight(
            None,
            "analyzing",
            None,
            &window_start,
            &window_end,
            item_count as i32,
            &model,
        )
        .map_err(|e| format!("创建分析记录失败: {}", e))?;

    // 5. Build prompt
    let (system_prompt, user_message) = attention_analyzer::build_prompt(&items);
    let provider = AnalysisProvider::from_str(&provider_str);

    // 6. Spawn background task
    tauri::async_runtime::spawn(async move {
        let repo = Repository::new(db.clone());

        match attention_analyzer::call_analysis_api(
            &provider,
            &api_key,
            &model,
            &system_prompt,
            &user_message,
        )
        .await
        {
            Ok(raw_response) => {
                // Validate and parse the response
                match attention_analyzer::validate_analysis(&raw_response, item_count) {
                    Ok(analysis) => {
                        // Build response JSON with analysis and id_map
                        let response = serde_json::json!({
                            "analysis": analysis,
                            "id_map": id_map,
                        });
                        let json_str = response.to_string();

                        if let Err(e) = repo.update_insight_status(
                            insight_id,
                            "complete",
                            Some(&json_str),
                            None,
                        ) {
                            log::error!("保存分析结果失败: {}", e);
                            let _ = repo.update_insight_status(
                                insight_id,
                                "error",
                                None,
                                Some(&format!("保存失败: {}", e)),
                            );
                            let _ = app.emit("attention-analysis-complete", "error");
                            return;
                        }

                        log::info!("注意力分析完成，共分析 {} 条内容", item_count);
                        let _ = app.emit("attention-analysis-complete", "complete");
                    }
                    Err(e) => {
                        log::error!("分析结果验证失败: {}", e);
                        let _ = repo.update_insight_status(
                            insight_id,
                            "error",
                            None,
                            Some(&e),
                        );
                        let _ = app.emit("attention-analysis-complete", "error");
                    }
                }
            }
            Err(e) => {
                log::error!("AI API 调用失败: {}", e);
                let _ = repo.update_insight_status(
                    insight_id,
                    "error",
                    None,
                    Some(&e),
                );
                let _ = app.emit("attention-analysis-complete", "error");
            }
        }
    });

    Ok(())
}
