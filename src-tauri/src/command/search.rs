use crate::{
    app::AppState,
    error::AppError,
    service::search_service::{SearchResponse, SearchService, SourceTestResult},
};
use tauri::{Manager, State};

#[tauri::command(rename = "search_books")]
pub async fn search_books_cmd(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    query: String,
    source_id: Option<i64>,
) -> Result<SearchResponse, AppError> {
    SearchService::new(state.database()?)
        .search_with_browser(&query, source_id, Some(app))
        .await
}

#[tauri::command(rename = "test_book_source")]
pub async fn test_book_source_cmd(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    source_id: i64,
    query: String,
) -> Result<SourceTestResult, AppError> {
    let browser = app.get_webview_window(&format!("source-auth-{source_id}"));
    SearchService::new(state.database()?)
        .test_with_browser(source_id, &query, browser)
        .await
}
