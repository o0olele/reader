use crate::{
    app::AppState, domain::DownloadTask, error::AppError,
    service::download_service::DownloadService,
};
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
