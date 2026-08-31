use crate::{
    app::AppState, domain::BookshelfGroup, error::AppError,
    service::bookshelf_service::BookshelfService,
};
use tauri::State;

#[tauri::command(rename = "list_groups")]
pub async fn list_groups_cmd(state: State<'_, AppState>) -> Result<Vec<BookshelfGroup>, AppError> {
    BookshelfService::new(state.database()?).list().await
}

#[tauri::command(rename = "create_group")]
pub async fn create_group_cmd(
    state: State<'_, AppState>,
    name: String,
) -> Result<BookshelfGroup, AppError> {
    let name = name.trim();
    if name.is_empty() || name.len() > 40 {
        return Err(AppError::InvalidArgument(
            "分组名称需要为 1 到 40 个字符".into(),
        ));
    }
    BookshelfService::new(state.database()?).create(name).await
}

#[tauri::command(rename = "move_book_to_group")]
pub async fn move_book_to_group_cmd(
    state: State<'_, AppState>,
    book_id: i64,
    group_id: i64,
) -> Result<(), AppError> {
    BookshelfService::new(state.database()?)
        .move_book(book_id, group_id)
        .await
}
