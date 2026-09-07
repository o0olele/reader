use crate::{
    app::AppState,
    domain::DownloadTask,
    error::AppError,
    service::{
        cache_service::{CacheService, CacheStats},
        download_service::DownloadService,
        export_service::{ExportResult, ExportService},
    },
};
use std::path::Path;
use tauri::State;

#[tauri::command(rename = "list_download_tasks")]
pub async fn list_download_tasks_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<DownloadTask>, AppError> {
    DownloadService::new(state.database()?).list().await
}
#[tauri::command(rename = "start_download")]
pub async fn start_download_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_ids: Option<Vec<i64>>,
) -> Result<i64, AppError> {
    let pool = state.database()?;
    let id = DownloadService::new(pool.clone())
        .create(book_id, chapter_ids)
        .await?;
    DownloadService::spawn(pool, id);
    Ok(id)
}
#[tauri::command(rename = "pause_download")]
pub async fn pause_download_cmd(state: State<'_, AppState>, task_id: i64) -> Result<(), AppError> {
    DownloadService::new(state.database()?).pause(task_id).await
}
#[tauri::command(rename = "resume_download")]
pub async fn resume_download_cmd(state: State<'_, AppState>, task_id: i64) -> Result<(), AppError> {
    let pool = state.database()?;
    let service = DownloadService::new(pool.clone());
    service.resume(task_id).await?;
    DownloadService::spawn(pool, task_id);
    Ok(())
}
#[tauri::command(rename = "cancel_download")]
pub async fn cancel_download_cmd(state: State<'_, AppState>, task_id: i64) -> Result<(), AppError> {
    DownloadService::new(state.database()?)
        .cancel(task_id)
        .await
}

#[tauri::command(rename = "export_book")]
pub async fn export_book_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    format: String,
    target_path: String,
) -> Result<ExportResult, AppError> {
    if target_path.trim().is_empty() {
        return Err(AppError::InvalidArgument("导出路径不能为空".into()));
    }
    ExportService::new(state.database()?)
        .export(book_id, &format, Path::new(&target_path))
        .await
}

#[tauri::command(rename = "get_cache_stats")]
pub async fn get_cache_stats_cmd(state: State<'_, AppState>) -> Result<CacheStats, AppError> {
    CacheService::new(state.database()?).stats().await
}

#[tauri::command(rename = "set_cache_quota")]
pub async fn set_cache_quota_cmd(
    state: State<'_, AppState>,
    megabytes: i64,
) -> Result<CacheStats, AppError> {
    CacheService::new(state.database()?)
        .set_quota(megabytes)
        .await
}

#[tauri::command(rename = "clear_chapter_cache")]
pub async fn clear_chapter_cache_cmd(state: State<'_, AppState>) -> Result<CacheStats, AppError> {
    CacheService::new(state.database()?).clear().await
}
