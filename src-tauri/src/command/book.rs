use crate::{
    app::AppState, domain::source::BookSearchResult, domain::Book, error::AppError,
    service::book_service::BookService,
};
use tauri::State;

#[tauri::command(rename = "import_txt_book")]
pub async fn import_txt_book_cmd(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    BookService::new(state.database()?)
        .import_txt(&filename, &bytes)
        .await
}

#[tauri::command(rename = "import_epub_book")]
pub async fn import_epub_book_cmd(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    BookService::new(state.database()?)
        .import_epub(&filename, bytes)
        .await
}

#[tauri::command(rename = "list_books")]
pub async fn list_books_cmd(state: State<'_, AppState>) -> Result<Vec<Book>, AppError> {
    BookService::new(state.database()?).list().await
}

#[tauri::command(rename = "delete_book")]
pub async fn delete_book_cmd(state: State<'_, AppState>, book_id: i64) -> Result<(), AppError> {
    BookService::new(state.database()?).delete(book_id).await
}

#[tauri::command(rename = "add_online_book")]
pub async fn add_online_book_cmd(
    state: State<'_, AppState>,
    result: BookSearchResult,
) -> Result<Book, AppError> {
    BookService::new(state.database()?)
        .add_online(&result)
        .await
}
