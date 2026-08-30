use crate::command_legacy::SourceTestResult;
use crate::{app::AppState, command_legacy as command, error::AppError, source::BookSearchResult};
use tauri::State;

#[tauri::command(rename = "search_books")]
pub async fn search_books_cmd(
    state: State<'_, AppState>,
    query: String,
    source_id: Option<i64>,
) -> Result<Vec<BookSearchResult>, AppError> {
    command::search_books(state, query, source_id).await
}

#[tauri::command(rename = "test_book_source")]
pub async fn test_book_source_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    query: String,
) -> Result<SourceTestResult, AppError> {
    command::test_book_source(state, source_id, query).await
}
