use crate::{
    app::AppState,
    error::AppError,
    service::source_service::{SearchResponse, SourceService, SourceTestResult},
};
use tauri::State;

#[tauri::command(rename = "search_books")]
pub async fn search_books_cmd(
    state: State<'_, AppState>,
    query: String,
    source_id: Option<i64>,
) -> Result<SearchResponse, AppError> {
    SourceService::new(state.database()?)
        .search(&query, source_id)
        .await
}

#[tauri::command(rename = "test_book_source")]
pub async fn test_book_source_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    query: String,
) -> Result<SourceTestResult, AppError> {
    SourceService::new(state.database()?)
        .test(source_id, &query)
        .await
}
