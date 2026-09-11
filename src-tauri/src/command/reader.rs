use crate::{
    app::AppState,
    domain::{Chapter, ReadingProgress, ReadingRecord, ReadingStats},
    error::AppError,
    service::{reader_service::ReaderService, settings_service::SettingsService},
};
use tauri::{AppHandle, Emitter, State};

#[tauri::command(rename = "read_chapter")]
pub async fn read_chapter_cmd(
    state: State<'_, AppState>,
    chapter_id: i64,
) -> Result<Chapter, AppError> {
    ReaderService::new(state.database()?)
        .read_chapter(chapter_id)
        .await
}

/// Warms the chapters around the open one (`legado` `ReadBook.preDownload`).
/// Deliberately fire-and-forget: reading must never wait on a prefetch, and a
/// book without a source simply has nothing to download.
#[tauri::command(rename = "prefetch_chapters")]
pub async fn prefetch_chapters_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
) -> Result<(), AppError> {
    let pool = state.database()?;
    tauri::async_runtime::spawn(async move {
        if let Err(error) = ReaderService::new(pool)
            .prefetch_around(book_id, chapter_id)
            .await
        {
            tracing::warn!(target: "reader", book_id, chapter_id, %error, "chapter prefetch aborted");
        }
    });
    Ok(())
}

/// Stops the window in flight when the reader closes the book.
#[tauri::command(rename = "cancel_prefetch")]
pub async fn cancel_prefetch_cmd(book_id: i64) -> Result<(), AppError> {
    crate::service::reader_service::cancel_prefetch(book_id)
}

#[tauri::command(rename = "get_reader_prefetch_num")]
pub async fn get_reader_prefetch_num_cmd(state: State<'_, AppState>) -> Result<i64, AppError> {
    SettingsService::new(state.database()?)
        .reader_prefetch_num()
        .await
}

#[tauri::command(rename = "set_reader_prefetch_num")]
pub async fn set_reader_prefetch_num_cmd(
    state: State<'_, AppState>,
    chapters: i64,
) -> Result<i64, AppError> {
    let settings = SettingsService::new(state.database()?);
    settings.save_reader_prefetch_num(chapters).await?;
    settings.reader_prefetch_num().await
}

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
    let before = ReaderService::new(state.database()?)
        .list_chapters(book_id)
        .await?;
    let chapters = ReaderService::new(state.database()?)
        .refresh_catalog(book_id)
        .await?;
    let known = before
        .into_iter()
        .map(|chapter| chapter.number)
        .collect::<std::collections::HashSet<_>>();
    let added = chapters
        .iter()
        .filter(|chapter| !known.contains(&chapter.number))
        .count();
    app.emit(
        "chapter-updated",
        serde_json::json!({ "book_id": book_id, "count": chapters.len(), "added": added }),
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
    anchor_index: Option<i64>,
    anchor_ratio: Option<f64>,
) -> Result<(), AppError> {
    let service = ReaderService::new(state.database()?);
    if let (Some(anchor_index), Some(anchor_ratio)) = (anchor_index, anchor_ratio) {
        service
            .save_progress_anchor(book_id, chapter_id, offset, anchor_index, anchor_ratio)
            .await
    } else {
        service.save_progress(book_id, chapter_id, offset).await
    }
}

#[tauri::command(rename = "get_reading_record")]
pub async fn get_reading_record_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Option<ReadingRecord>, AppError> {
    ReaderService::new(state.database()?)
        .reading_record(book_id)
        .await
}

#[tauri::command(rename = "add_reading_time")]
pub async fn add_reading_time_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    duration_seconds: i64,
) -> Result<(), AppError> {
    ReaderService::new(state.database()?)
        .add_reading_time(book_id, duration_seconds)
        .await
}

#[tauri::command(rename = "get_reading_stats")]
pub async fn get_reading_stats_cmd(
    state: State<'_, AppState>,
) -> Result<ReadingStats, AppError> {
    ReaderService::new(state.database()?).reading_stats().await
}

#[tauri::command(rename = "set_reading_goal")]
pub async fn set_reading_goal_cmd(
    state: State<'_, AppState>,
    minutes: i64,
) -> Result<ReadingStats, AppError> {
    ReaderService::new(state.database()?)
        .set_reading_goal(minutes)
        .await
}
