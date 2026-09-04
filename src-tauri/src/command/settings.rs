use crate::{
    app::AppState,
    error::AppError,
    infrastructure::http::request::{
        set_detected_user_agent, set_user_agent, user_agent, DEFAULT_USER_AGENT,
    },
    service::settings_service::SettingsService,
};
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command(rename = "get_app_settings")]
pub async fn get_app_settings_cmd(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let settings = SettingsService::new(state.database()?);
    Ok(AppSettings {
        proxy_url: settings.proxy_url().await?,
        user_agent: settings.user_agent().await?,
        detected_user_agent: settings.webview_user_agent().await?,
        effective_user_agent: user_agent(),
        default_user_agent: DEFAULT_USER_AGENT,
    })
}

/// Records what the main window reports as its own `navigator.userAgent`, so
/// every HTTP request can impersonate the very webview that will be solving
/// Cloudflare challenges. Called once by the frontend on boot; a no-op if the
/// value has not changed.
#[tauri::command(rename = "report_webview_user_agent")]
pub async fn report_webview_user_agent_cmd(
    state: State<'_, AppState>,
    user_agent: String,
) -> Result<(), AppError> {
    let reported = user_agent.trim();
    // A UA is a header value; anything that cannot be one is not from a real
    // webview and must not reach the request builders.
    if reported.is_empty() || reqwest::header::HeaderValue::from_str(reported).is_err() {
        return Err(AppError::InvalidArgument("浏览器 User-Agent 无效".into()));
    }
    let settings = SettingsService::new(state.database()?);
    if settings.webview_user_agent().await?.as_deref() != Some(reported) {
        settings.save_webview_user_agent(reported).await?;
        tracing::info!(target: "network", user_agent = %reported, "adopted webview user agent");
    }
    set_detected_user_agent(Some(reported));
    Ok(())
}

#[derive(Serialize)]
pub struct AppSettings {
    pub proxy_url: Option<String>,
    /// The explicit user override. `None` means the detected value is in use.
    pub user_agent: Option<String>,
    /// What the embedded webview reported about itself on last boot.
    pub detected_user_agent: Option<String>,
    /// What requests actually go out with, after applying the precedence
    /// `user_agent` → `detected_user_agent` → `default_user_agent`.
    pub effective_user_agent: String,
    /// Surfaced so the settings UI can show the last-resort fallback.
    pub default_user_agent: &'static str,
}

#[derive(Deserialize)]
pub struct AppSettingsInput {
    pub proxy_url: Option<String>,
    pub user_agent: Option<String>,
}

#[tauri::command(rename = "save_app_settings")]
pub async fn save_app_settings_cmd(
    state: State<'_, AppState>,
    input: AppSettingsInput,
) -> Result<AppSettings, AppError> {
    let proxy = input.proxy_url.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    });
    if let Some(value) = proxy.as_deref() {
        reqwest::Proxy::all(value)
            .map_err(|error| AppError::InvalidArgument(format!("代理 URL 无效: {error}")))?;
    }
    let agent = input.user_agent.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    });
    if let Some(value) = agent.as_deref() {
        reqwest::header::HeaderValue::from_str(value)
            .map_err(|_| AppError::InvalidArgument("User-Agent 含有非法字符".into()))?;
    }
    let settings = SettingsService::new(state.database()?);
    settings.save_proxy_url(proxy.as_deref()).await?;
    settings.save_user_agent(agent.as_deref()).await?;
    *state
        .global_proxy
        .lock()
        .map_err(|_| AppError::Database("代理状态锁不可用".into()))? = proxy.clone();
    set_user_agent(agent.as_deref());
    Ok(AppSettings {
        proxy_url: proxy,
        user_agent: agent,
        detected_user_agent: settings.webview_user_agent().await?,
        effective_user_agent: user_agent(),
        default_user_agent: DEFAULT_USER_AGENT,
    })
}
