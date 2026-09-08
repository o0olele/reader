//! Browser-auth window commands and helpers.

use crate::{
    app::AppState,
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::request::user_agent_override,
    service::source_service::{SourceLoginResult, SourceService},
};
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};

#[tauri::command(rename = "open_book_source_browser")]
pub async fn open_book_source_browser_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    let source = SourceService::new(state.database()?).get(source_id).await?;
    let url = browser_auth_url(&source)?;
    let global_proxy = state
        .global_proxy
        .lock()
        .map_err(|_| AppError::Database("代理状态锁不可用".into()))?
        .clone();
    let proxy = browser_proxy_url(&source, global_proxy.as_deref())?;
    let profile_dir = app
        .path()
        .app_data_dir()
        .map_err(AppError::io)?
        .join("webview-profiles")
        .join(format!("source-{source_id}"));
    std::fs::create_dir_all(&profile_dir).map_err(AppError::io)?;
    let label = format!("source-auth-{source_id}");
    if let Some(window) = app.get_webview_window(&label) {
        window
            .show()
            .map_err(|error| AppError::Source(error.to_string()))?;
        window
            .set_focus()
            .map_err(|error| AppError::Source(error.to_string()))?;
        return Ok(());
    }
    let mut builder = WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(url))
        .title(format!("浏览器认证 - {}", source.name))
        .inner_size(1000.0, 760.0)
        .min_inner_size(640.0, 480.0)
        // WebView2 environment options such as `--proxy-server` are fixed for
        // a user-data directory. The main window has already initialized the
        // default profile without this per-source proxy; attempting to reuse
        // it with different options fails with ERROR_INVALID_STATE
        // (0x8007139F). A stable source profile avoids that conflict and keeps
        // browser cookies across authentication-window reopenings.
        .data_directory(profile_dir);
    // Only override when the user asked for a specific UA. Left alone, the
    // window presents its native identity — the same one the frontend reported
    // and that reqwest now replays — so its UA and its client hints agree.
    // Forcing a UA changes the header but not the hints the webview emits, so
    // it reintroduces exactly the mismatch that invalidates `cf_clearance`.
    if let Some(configured) = user_agent_override() {
        builder = builder.user_agent(&configured);
    }
    // The challenge and the follow-up reqwest request must leave through the
    // same proxy. Cloudflare can bind clearance to the observed client IP, so
    // solving it in a direct WebView and replaying the cookie through the
    // configured proxy produces a valid-looking cookie that is still rejected.
    if let Some(proxy) = proxy {
        builder = builder.proxy_url(proxy);
    }
    builder
        .build()
        .map_err(|error| AppError::Source(format!("打开浏览器认证窗口失败: {error}")))?;
    Ok(())
}

#[tauri::command(rename = "save_book_source_browser_session")]
pub async fn save_book_source_browser_session_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<SourceLoginResult, AppError> {
    let source = SourceService::new(state.database()?).get(source_id).await?;
    let url = browser_auth_url(&source)?;
    let label = format!("source-auth-{source_id}");
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| AppError::Source("请先打开浏览器认证窗口".into()))?;
    let base_url = reqwest::Url::parse(&source.base_url)
        .map_err(|_| AppError::InvalidArgument("书源基础 URL 无效".into()))?;
    // Cloudflare often sets its clearance cookie on the origin root while the
    // login page is nested under a path, and a challenge can bounce the window
    // to a host neither URL covers. Read every scope we know about and keep the
    // latest value for each cookie name.
    let mut scopes = vec![base_url, url];
    if let Ok(current) = window.url() {
        scopes.push(current);
    }
    let mut cookies = std::collections::BTreeMap::new();
    for scope in scopes {
        let found = window
            .cookies_for_url(scope)
            .map_err(|error| AppError::Source(format!("读取浏览器 Cookie 失败: {error}")))?;
        for cookie in found {
            cookies.insert(cookie.name().to_owned(), cookie.value().to_owned());
        }
    }
    let cookies = cookies
        .into_iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("; ");
    SourceService::new(state.database()?)
        .save_browser_cookies(source_id, &cookies)
        .await
}

fn browser_auth_url(source: &BookSource) -> Result<reqwest::Url, AppError> {
    let raw_url = source
        .login_url
        .as_deref()
        .unwrap_or(&source.base_url)
        .trim();
    // legado permits loginUrl to be a JS rule. A browser window cannot execute
    // that request rule, so use the source origin and let the user navigate.
    let raw_url =
        if raw_url.starts_with("@js:") || raw_url.starts_with("<js>") || raw_url.contains("{{") {
            source.base_url.trim()
        } else {
            raw_url
        };
    let url = reqwest::Url::parse(raw_url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(raw_url)))
        .map_err(|_| AppError::InvalidArgument("浏览器认证 URL 无效".into()))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(AppError::InvalidArgument(
            "浏览器认证 URL 必须使用 HTTP 或 HTTPS".into(),
        ));
    }
    Ok(url)
}

fn browser_proxy_url(
    source: &BookSource,
    global_proxy: Option<&str>,
) -> Result<Option<reqwest::Url>, AppError> {
    let Some(raw) = source
        .proxy_url
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or(global_proxy.filter(|value| !value.trim().is_empty()))
    else {
        return Ok(None);
    };
    let proxy = reqwest::Url::parse(raw.trim())
        .map_err(|error| AppError::InvalidArgument(format!("浏览器代理 URL 无效: {error}")))?;
    if !matches!(proxy.scheme(), "http" | "socks5") {
        return Err(AppError::InvalidArgument(
            "浏览器代理仅支持 http:// 或 socks5://".into(),
        ));
    }
    Ok(Some(proxy))
}

#[cfg(test)]
mod tests {
    use super::browser_proxy_url;
    use crate::domain::source::{BookSource, CatalogRule, InfoRule, SearchRule};

    fn source(proxy_url: Option<&str>) -> BookSource {
        BookSource {
            id: 1,
            name: "proxy source".into(),
            base_url: "https://example.com".into(),
            search_url: "search?q={{key}}".into(),
            explore_url: None,
            book_url_pattern: None,
            enabled_cookie_jar: true,
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "GET".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: proxy_url.map(str::to_owned),
            concurrent_rate: None,
            enabled: true,
            source_group: None,
            custom_order: 0,
            weight: 0,
            enabled_explore: true,
            respond_time: None,
            last_update_time: None,
            raw_rules: Default::default(),
        }
    }

    #[test]
    fn browser_uses_global_proxy_when_source_has_none() {
        let proxy = browser_proxy_url(&source(None), Some("http://127.0.0.1:7890"))
            .unwrap()
            .unwrap();
        assert_eq!(proxy.as_str(), "http://127.0.0.1:7890/");
    }

    #[test]
    fn source_proxy_overrides_global_proxy() {
        let proxy = browser_proxy_url(
            &source(Some("socks5://127.0.0.1:1080")),
            Some("http://127.0.0.1:7890"),
        )
        .unwrap()
        .unwrap();
        assert_eq!(proxy.as_str(), "socks5://127.0.0.1:1080");
    }
}
