use crate::{app::AppState, command_legacy as command, domain::Book, error::AppError};
use tauri::State;

#[tauri::command(rename = "import_txt_book")]
pub async fn import_txt_book_cmd(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    command::import_txt_book(state, filename, bytes).await
}

#[tauri::command(rename = "import_epub_book")]
pub async fn import_epub_book_cmd(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    command::import_epub_book(state, filename, bytes).await
}

#[tauri::command(rename = "list_books")]
pub async fn list_books_cmd(state: State<'_, AppState>) -> Result<Vec<Book>, AppError> {
    command::list_books(state).await
}

#[tauri::command(rename = "delete_book")]
pub async fn delete_book_cmd(state: State<'_, AppState>, book_id: i64) -> Result<(), AppError> {
    command::delete_book(state, book_id).await
}

#[tauri::command(rename = "add_online_book")]
pub async fn add_online_book_cmd(
    state: State<'_, AppState>,
    result: crate::source::BookSearchResult,
) -> Result<Book, AppError> {
    command::add_online_book(state, result).await
}
