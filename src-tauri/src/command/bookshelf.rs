use crate::{app::AppState, command_legacy as command, domain::BookshelfGroup, error::AppError};
use tauri::State;

#[tauri::command(rename = "list_groups")]
pub async fn list_groups_cmd(state: State<'_, AppState>) -> Result<Vec<BookshelfGroup>, AppError> {
    command::list_groups(state).await
}

#[tauri::command(rename = "create_group")]
pub async fn create_group_cmd(
    state: State<'_, AppState>,
    name: String,
) -> Result<BookshelfGroup, AppError> {
    command::create_group(state, name).await
}

#[tauri::command(rename = "move_book_to_group")]
pub async fn move_book_to_group_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    group_id: i64,
) -> Result<(), AppError> {
    command::move_book_to_group(state, book_id, group_id).await
}
