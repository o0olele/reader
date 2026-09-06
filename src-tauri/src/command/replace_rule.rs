use crate::{
    app::AppState,
    domain::replace_rule::ReplaceRule,
    error::AppError,
    service::{content_processor::ProcessedContent, replace_rule_service::ReplaceRuleService},
};
use tauri::State;

#[tauri::command(rename = "list_replace_rules")]
pub async fn list_replace_rules_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<ReplaceRule>, AppError> {
    ReplaceRuleService::new(state.database()?).list().await
}

#[tauri::command(rename = "save_replace_rule")]
pub async fn save_replace_rule_cmd(
    state: State<'_, AppState>,
    rule: ReplaceRule,
) -> Result<ReplaceRule, AppError> {
    ReplaceRuleService::new(state.database()?).save(rule).await
}

#[tauri::command(rename = "delete_replace_rule")]
pub async fn delete_replace_rule_cmd(state: State<'_, AppState>, id: i64) -> Result<(), AppError> {
    ReplaceRuleService::new(state.database()?).delete(id).await
}

#[tauri::command(rename = "preview_replace_rule")]
pub async fn preview_replace_rule_cmd(
    rule: ReplaceRule,
    title: String,
    content: String,
) -> Result<ProcessedContent, AppError> {
    tokio::task::spawn_blocking(move || ReplaceRuleService::preview(rule, &title, &content))
        .await
        .map_err(AppError::parse)?
}
