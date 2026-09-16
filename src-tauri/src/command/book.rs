use crate::{
    app::AppState,
    domain::source::{BookSearchResult, ChapterRef, SourceBookPreview},
    domain::Book,
    error::AppError,
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
        .import_txt(&filename, bytes)
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

#[tauri::command(rename = "fetch_book_info")]
pub async fn fetch_book_info_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Book, AppError> {
    BookService::new(state.database()?)
        .fetch_info(book_id)
        .await
}

/// Reads a candidate source for an already-shelved book without switching to
/// it: 换源's 加载详情 / 加载目录, and the check that runs before 换源 commits.
#[tauri::command(rename = "preview_book_source")]
pub async fn preview_book_source_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    result: BookSearchResult,
    with_info: bool,
    with_toc: bool,
) -> Result<SourceBookPreview, AppError> {
    BookService::new(state.database()?)
        .preview_source(book_id, &result, with_info, with_toc)
        .await
}

/// Switches a shelved book to another source. `chapters` is the catalog a
/// previous `preview_book_source` returned; omit it to have the backend fetch
/// the 目录 itself. The switch only happens once a catalog is in hand.
#[tauri::command(rename = "switch_book_source")]
pub async fn switch_book_source_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    result: BookSearchResult,
    chapters: Option<Vec<ChapterRef>>,
) -> Result<Book, AppError> {
    BookService::new(state.database()?)
        .switch_source(book_id, &result, chapters)
        .await
}
