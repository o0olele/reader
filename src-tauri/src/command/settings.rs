use crate::{app::AppState, error::AppError, service::settings_service::SettingsService};
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command(rename = "get_app_settings")]
pub async fn get_app_settings_cmd(
    state: State<'_, AppState>,
) -> Result<AppSettings, AppError> {
    Ok(AppSettings { proxy_url: SettingsService::new(state.database()?).proxy_url().await? })
}

#[derive(Serialize)]
pub struct AppSettings { pub proxy_url: Option<String> }

#[derive(Deserialize)]
pub struct AppSettingsInput { pub proxy_url: Option<String> }

#[tauri::command(rename = "save_app_settings")]
pub async fn save_app_settings_cmd(
    state: State<'_, AppState>,
    input: AppSettingsInput,
) -> Result<AppSettings, AppError> {
    let proxy = input.proxy_url.and_then(|value| { let value = value.trim().to_owned(); (!value.is_empty()).then_some(value) });
    if let Some(value) = proxy.as_deref() { reqwest::Proxy::all(value).map_err(|error| AppError::InvalidArgument(format!("代理 URL 无效: {error}")))?; }
    SettingsService::new(state.database()?).save_proxy_url(proxy.as_deref()).await?;
    *state.global_proxy.lock().map_err(|_| AppError::Database("代理状态锁不可用".into()))? = proxy.clone();
    Ok(AppSettings { proxy_url: proxy })
}
