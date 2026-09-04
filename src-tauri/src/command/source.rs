use crate::{
    app::AppState,
    domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule},
    error::AppError,
    service::source_service::{
        SourceImportReport, SourceLoginInput, SourceLoginResult, SourceService, SourceSessionStatus,
    },
};
use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::infrastructure::http::request::user_agent_override;

#[tauri::command(rename = "debug_source_stage")]
pub async fn debug_source_stage_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    source_id: i64,
    stage: crate::service::source_debug_service::SourceDebugStage,
    input: String,
) -> Result<crate::service::source_debug_service::SourceDebugResult, AppError> {
    let _ = app.emit(
        "source-test-progress",
        serde_json::json!({"source_id": source_id, "stage": stage, "state": "started"}),
    );
    let result = crate::service::source_debug_service::SourceDebugService::new(state.database()?)
        .run_with_browser(
            source_id,
            stage,
            &input,
            app.get_webview_window(&format!("source-auth-{source_id}")),
        )
        .await?;
    let _ = app.emit(
        "source-test-progress",
        serde_json::json!({"source_id": source_id, "stage": result.stage, "state": "completed"}),
    );
    Ok(result)
}

#[tauri::command(rename = "update_book_source_rules")]
pub async fn update_book_source_rules_cmd(
    state: State<'_, AppState>,
    source_id: i64,
    raw_rules: RawSourceRules,
) -> Result<(), AppError> {
    crate::service::source_debug_service::SourceDebugService::new(state.database()?)
        .update_rules(source_id, raw_rules)
        .await
}

#[tauri::command(rename = "list_book_sources")]
pub async fn list_book_sources_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<BookSource>, AppError> {
    SourceService::new(state.database()?).list().await
}

#[derive(Deserialize)]
pub struct BookSourceInput {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    #[serde(default)]
    pub explore_url: Option<String>,
    pub search_rule: SearchRule,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub info_rule: InfoRule,
    #[serde(default = "default_catalog_rule")]
    pub catalog_rule: CatalogRule,
    #[serde(default = "default_content_selector")]
    pub content_selector: String,
    #[serde(default)]
    pub next_toc_url_selector: Option<String>,
    #[serde(default)]
    pub next_content_url_selector: Option<String>,
    #[serde(default)]
    pub header: Option<String>,
    #[serde(default)]
    pub login_url: Option<String>,
    #[serde(default = "default_login_method")]
    pub login_method: String,
    #[serde(default)]
    pub login_body: Option<String>,
    #[serde(default)]
    pub token_path: Option<String>,
    #[serde(default)]
    pub sign_script: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub concurrent_rate: Option<String>,
}

fn default_login_method() -> String {
    "POST".into()
}
fn default_catalog_rule() -> CatalogRule {
    CatalogRule {
        item: "a".into(),
        title: "a".into(),
        url: "a::attr(href)".into(),
        next_url: None,
    }
}
fn default_content_selector() -> String {
    "body".into()
}
#[tauri::command(rename = "save_book_source")]
pub async fn save_book_source_cmd(
    state: State<'_, AppState>,
    input: BookSourceInput,
) -> Result<BookSource, AppError> {
    let name = input.name.trim();
    let base_url = input.base_url.trim();
    let search_url = input.search_url.trim();
    if name.is_empty() || name.len() > 80 {
        return Err(AppError::InvalidArgument(
            "书源名称需要为 1 到 80 个字符".into(),
        ));
    }
    reqwest::Url::parse(base_url)
        .map_err(|_| AppError::InvalidArgument("书源基础 URL 无效".into()))?;
    if !search_url.contains("{{key}}") && !search_url.contains("{key}") {
        return Err(AppError::InvalidArgument(
            "搜索 URL 必须包含 {{key}} 占位符".into(),
        ));
    }
    for selector in [
        &input.search_rule.item,
        &input.search_rule.title,
        &input.search_rule.url,
    ] {
        scraper::Selector::parse(selector)
            .map_err(|_| AppError::Parse(format!("无效 CSS 选择器: {selector}")))?;
    }
    let source = BookSource {
        id: 0,
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        search_url: search_url.to_owned(),
        explore_url: input.explore_url.filter(|value| !value.trim().is_empty()),
        search_rule: input.search_rule,
        info_rule: input.info_rule,
        catalog_rule: input.catalog_rule,
        content_selector: input.content_selector,
        next_toc_url_selector: input.next_toc_url_selector,
        next_content_url_selector: input.next_content_url_selector,
        header: input.header,
        login_url: input.login_url,
        login_method: input.login_method,
        login_body: input.login_body,
        token_path: input.token_path,
        access_token: None,
        session_cookie: None,
        session_expires_at: None,
        sign_script: input.sign_script,
        proxy_url: input.proxy_url,
        concurrent_rate: input.concurrent_rate,
        enabled: input.enabled.unwrap_or(true),
        // Saving by hand takes the source's rules over. Carrying legado rules
        // across from an earlier import would silently outrank the selectors
        // the user just typed, since the engine prefers them.
        raw_rules: RawSourceRules::default(),
    };
    let id = SourceService::new(state.database()?)
        .upsert(&source)
        .await?;
    Ok(BookSource { id, ..source })
}
#[tauri::command(rename = "import_book_sources_json")]
pub async fn import_book_sources_json_cmd(
    state: State<'_, AppState>,
    input: String,
) -> Result<SourceImportReport, AppError> {
    SourceService::new(state.database()?)
        .import_json(&input)
        .await
}
#[tauri::command(rename = "import_book_sources_url")]
pub async fn import_book_sources_url_cmd(
    state: State<'_, AppState>,
    url: String,
) -> Result<SourceImportReport, AppError> {
    SourceService::new(state.database()?).import_url(&url).await
}
#[tauri::command(rename = "login_book_source")]
pub async fn login_book_source_cmd(
    state: State<'_, AppState>,
    input: SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    SourceService::new(state.database()?).login(input).await
}
#[tauri::command(rename = "clear_book_source_session")]
pub async fn clear_book_source_session_cmd(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    SourceService::new(state.database()?)
        .clear_session(source_id)
        .await
}

#[tauri::command(rename = "get_book_source_session_status")]
pub async fn get_book_source_session_status_cmd(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<SourceSessionStatus, AppError> {
    SourceService::new(state.database()?)
        .session_status(source_id)
        .await
}

#[tauri::command(rename = "refresh_book_source_session")]
pub async fn refresh_book_source_session_cmd(
    state: State<'_, AppState>,
    input: SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    SourceService::new(state.database()?)
        .refresh_session(input)
        .await
}

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

fn browser_auth_url(source: &crate::domain::source::BookSource) -> Result<reqwest::Url, AppError> {
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
    source: &crate::domain::source::BookSource,
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
mod browser_tests {
    use super::browser_proxy_url;
    use crate::domain::source::{BookSource, CatalogRule, InfoRule, SearchRule};

    fn source(proxy_url: Option<&str>) -> BookSource {
        BookSource {
            id: 1,
            name: "proxy source".into(),
            base_url: "https://example.com".into(),
            search_url: "search?q={{key}}".into(),
            explore_url: None,
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
