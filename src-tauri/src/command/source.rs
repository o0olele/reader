//! Thin Tauri IPC adapters for book-source management and debugging.

#[path = "source/browser.rs"]
pub mod browser;
#[path = "source/input.rs"]
pub mod input;

use crate::{
    app::AppState,
    domain::source::{BookSource, RawSourceRules},
    error::AppError,
    service::source_export_service::{SourceExportResult, SourceExportService},
    service::source_service::{
        SourceImportReport, SourceLoginInput, SourceLoginResult, SourceService, SourceSessionStatus,
    },
};
use tauri::{AppHandle, Emitter, Manager, State};

#[tauri::command(rename = "debug_source_stage")]
pub async fn debug_source_stage_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    source_id: i64,
    stage: crate::service::source_debug_service::SourceDebugStage,
    input: String,
) -> Result<crate::service::source_debug_service::SourceDebugResult, AppError> {
    let _ = app.emit(
        "source-test-progress",
        serde_json::json!({"source_id": source_id, "stage": stage, "state": "started"}),
    );
    let result = crate::service::source_debug_service::SourceDebugService::new(state.database()?)
        .run_with_browser(
            source_id,
            stage,
            &input,
            app.get_webview_window(&format!("source-auth-{source_id}")),
        )
        .await?;
    let _ = app.emit(
        "source-test-progress",
        serde_json::json!({"source_id": source_id, "stage": result.stage, "state": "completed"}),
    );
    Ok(result)
}

#[tauri::command(rename = "update_book_source_rules")]
pub async fn update_book_source_rules_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    raw_rules: RawSourceRules,
) -> Result<(), AppError> {
    crate::service::source_debug_service::SourceDebugService::new(state.database()?)
        .update_rules(source_id, raw_rules)
        .await
}

#[tauri::command(rename = "export_source_fixture")]
pub async fn export_source_fixture_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    html: String,
    out_dir: String,
) -> Result<String, AppError> {
    if out_dir.trim().is_empty() {
        return Err(AppError::InvalidArgument("fixture 目录不能为空".into()));
    }
    crate::service::source_debug_service::SourceDebugService::new(state.database()?)
        .export_fixture(source_id, &html, std::path::Path::new(&out_dir))
        .await
}

#[tauri::command(rename = "list_book_sources")]
pub async fn list_book_sources_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<BookSource>, AppError> {
    SourceService::new(state.database()?).list().await
}

#[tauri::command(rename = "set_book_source_enabled")]
pub async fn set_book_source_enabled_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    enabled: bool,
) -> Result<(), AppError> {
    SourceService::new(state.database()?)
        .set_enabled(source_id, enabled)
        .await
}

#[tauri::command(rename = "update_book_source_management")]
pub async fn update_book_source_management_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    source_group: Option<String>,
    custom_order: i64,
    weight: i64,
    enabled_explore: bool,
) -> Result<(), AppError> {
    SourceService::new(state.database()?)
        .update_management(
            source_id,
            source_group,
            custom_order,
            weight,
            enabled_explore,
        )
        .await
}

#[tauri::command(rename = "export_book_sources")]
pub async fn export_book_sources_cmd(
    state: State<'_, AppState>,
    target_path: String,
) -> Result<SourceExportResult, AppError> {
    if target_path.trim().is_empty() {
        return Err(AppError::InvalidArgument("导出路径不能为空".into()));
    }
    SourceExportService::new(state.database()?)
        .export_legado(std::path::Path::new(&target_path))
        .await
}

#[tauri::command(rename = "import_book_sources_json")]
pub async fn import_book_sources_json_cmd(
    state: State<'_, AppState>,
    input: String,
) -> Result<SourceImportReport, AppError> {
    SourceService::new(state.database()?)
        .import_json(&input)
        .await
}

#[tauri::command(rename = "import_book_sources_url")]
pub async fn import_book_sources_url_cmd(
    state: State<'_, AppState>,
    url: String,
) -> Result<SourceImportReport, AppError> {
    SourceService::new(state.database()?).import_url(&url).await
}

#[tauri::command(rename = "login_book_source")]
pub async fn login_book_source_cmd(
    state: State<'_, AppState>,
    input: SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    SourceService::new(state.database()?).login(input).await
}

#[tauri::command(rename = "clear_book_source_session")]
pub async fn clear_book_source_session_cmd(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    SourceService::new(state.database()?)
        .clear_session(source_id)
        .await
}

#[tauri::command(rename = "get_book_source_session_status")]
pub async fn get_book_source_session_status_cmd(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<SourceSessionStatus, AppError> {
    SourceService::new(state.database()?)
        .session_status(source_id)
        .await
}

#[tauri::command(rename = "refresh_book_source_session")]
pub async fn refresh_book_source_session_cmd(
    state: State<'_, AppState>,
    input: SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    SourceService::new(state.database()?)
        .refresh_session(input)
        .await
}
