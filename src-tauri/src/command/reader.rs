use crate::{
    app::AppState,
    domain::{Chapter, ReadingProgress},
    error::AppError,
    service::reader_service::ReaderService,
};
use tauri::{AppHandle, Emitter, State};

#[tauri::command(rename = "list_chapters")]
pub async fn list_chapters_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    ReaderService::new(state.database()?)
        .list_chapters(book_id)
        .await
}

#[tauri::command(rename = "refresh_catalog")]
pub async fn refresh_catalog_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    let chapters = ReaderService::new(state.database()?)
        .refresh_catalog(book_id)
        .await?;
    app.emit(
        "chapter-updated",
        serde_json::json!({ "book_id": book_id, "count": chapters.len() }),
    )
    .map_err(AppError::database)?;
    Ok(chapters)
}

#[tauri::command(rename = "fetch_online_content")]
pub async fn fetch_online_content_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    chapter_url: String,
    chapter_id: Option<i64>,
) -> Result<String, AppError> {
    ReaderService::new(state.database()?)
        .fetch_online_content(source_id, &chapter_url, chapter_id)
        .await
}

#[tauri::command(rename = "get_reading_progress")]
pub async fn get_reading_progress_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Option<ReadingProgress>, AppError> {
    ReaderService::new(state.database()?)
        .progress(book_id)
        .await
}

#[tauri::command(rename = "save_reading_progress")]
pub async fn save_reading_progress_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
    offset: i64,
) -> Result<(), AppError> {
    ReaderService::new(state.database()?)
        .save_progress(book_id, chapter_id, offset)
        .await
}
