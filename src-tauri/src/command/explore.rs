use crate::{
    app::AppState,
    domain::source::BookSearchResult,
    error::AppError,
    service::explore_service::{ExploreCategory, ExploreService},
};
use tauri::State;

#[tauri::command(rename = "list_explore_categories")]
pub async fn list_explore_categories_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<ExploreCategory>, AppError> {
    ExploreService::new(state.database()?).categories().await
}

#[tauri::command(rename = "explore_books")]
pub async fn explore_books_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    url: String,
) -> Result<Vec<BookSearchResult>, AppError> {
    ExploreService::new(state.database()?)
        .books(source_id, &url)
        .await
}
