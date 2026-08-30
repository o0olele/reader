use crate::command_legacy::{SourceImportReport, SourceLoginResult};
use crate::{app::AppState, command_legacy as command, error::AppError, source::BookSource};
use tauri::State;

#[tauri::command(rename = "list_book_sources")]
pub async fn list_book_sources_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<BookSource>, AppError> {
    command::list_book_sources(state).await
}
#[tauri::command(rename = "save_book_source")]
pub async fn save_book_source_cmd(
    state: State<'_, AppState>,
    input: command::BookSourceInput,
) -> Result<BookSource, AppError> {
    command::save_book_source(state, input).await
}
#[tauri::command(rename = "import_book_sources_json")]
pub async fn import_book_sources_json_cmd(
    state: State<'_, AppState>,
    input: String,
) -> Result<SourceImportReport, AppError> {
    command::import_book_sources_json(state, input).await
}
#[tauri::command(rename = "import_book_sources_url")]
pub async fn import_book_sources_url_cmd(
    state: State<'_, AppState>,
    url: String,
) -> Result<SourceImportReport, AppError> {
    command::import_book_sources_url(state, url).await
}
#[tauri::command(rename = "login_book_source")]
pub async fn login_book_source_cmd(
    state: State<'_, AppState>,
    input: command::SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    command::login_book_source(state, input).await
}
#[tauri::command(rename = "clear_book_source_session")]
pub async fn clear_book_source_session_cmd(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    command::clear_book_source_session(state, source_id).await
}
