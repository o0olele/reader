use crate::{app::AppState, command_legacy as command, error::AppError};
use tauri::State;

#[tauri::command(rename = "get_app_settings")]
pub async fn get_app_settings_cmd(
    state: State<'_, AppState>,
) -> Result<command::AppSettings, AppError> {
    command::get_app_settings(state).await
}

#[tauri::command(rename = "save_app_settings")]
pub async fn save_app_settings_cmd(
    state: State<'_, AppState>,
    input: command::AppSettingsInput,
) -> Result<command::AppSettings, AppError> {
    command::save_app_settings(state, input).await
}
