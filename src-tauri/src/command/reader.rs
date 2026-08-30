use crate::{
    app::AppState,
    command_legacy as command,
    domain::{Chapter, ReadingProgress},
    error::AppError,
    service::reader_service::ReaderService,
};
use tauri::{AppHandle, State};

#[tauri::command(rename = "list_chapters")]
pub async fn list_chapters_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    command::list_chapters(state, book_id).await
}

#[tauri::command(rename = "refresh_catalog")]
pub async fn refresh_catalog_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    command::refresh_catalog(app, state, book_id).await
}

#[tauri::command(rename = "fetch_online_content")]
pub async fn fetch_online_content_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    chapter_url: String,
    chapter_id: Option<i64>,
) -> Result<String, AppError> {
    command::fetch_online_content(state, source_id, chapter_url, chapter_id).await
}

#[tauri::command(rename = "get_reading_progress")]
pub async fn get_reading_progress_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Option<ReadingProgress>, AppError> {
    ReaderService::new(state.database()?).progress(book_id).await
}

#[tauri::command(rename = "save_reading_progress")]
pub async fn save_reading_progress_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
    offset: i64,
) -> Result<(), AppError> {
    ReaderService::new(state.database()?).save_progress(book_id, chapter_id, offset).await
}
