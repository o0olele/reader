use crate::{
    app::AppState,
    error::AppError,
    repository::bookmark::{Bookmark, SqliteBookmarkRepository},
};
use tauri::State;

#[tauri::command(rename = "get_bookmark")]
pub async fn get_bookmark_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
) -> Result<Option<Bookmark>, AppError> {
    SqliteBookmarkRepository::new(state.database()?)
        .get(book_id, chapter_id)
        .await
}

#[tauri::command(rename = "save_bookmark")]
pub async fn save_bookmark_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
    offset: i64,
    mode: String,
) -> Result<(), AppError> {
    SqliteBookmarkRepository::new(state.database()?)
        .save(&Bookmark {
            book_id,
            chapter_id,
            offset,
            mode,
        })
        .await
}

#[tauri::command(rename = "delete_bookmark")]
pub async fn delete_bookmark_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
) -> Result<(), AppError> {
    SqliteBookmarkRepository::new(state.database()?)
        .delete(book_id, chapter_id)
        .await
}
