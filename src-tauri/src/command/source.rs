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
        .run(source_id, stage, &input)
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
    let raw_url = source
        .login_url
        .as_deref()
        .unwrap_or(&source.base_url)
        .trim();
    let url = reqwest::Url::parse(raw_url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(raw_url)))
        .map_err(|_| AppError::InvalidArgument("浏览器认证 URL 无效".into()))?;
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
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(url))
        .title(format!("浏览器认证 - {}", source.name))
        .inner_size(1000.0, 760.0)
        .min_inner_size(640.0, 480.0)
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
    let raw_url = source.login_url.as_deref().unwrap_or(&source.base_url);
    let url = reqwest::Url::parse(raw_url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(raw_url)))
        .map_err(|_| AppError::InvalidArgument("浏览器认证 URL 无效".into()))?;
    let label = format!("source-auth-{source_id}");
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| AppError::Source("请先打开浏览器认证窗口".into()))?;
    let base_url = reqwest::Url::parse(&source.base_url)
        .map_err(|_| AppError::InvalidArgument("书源基础 URL 无效".into()))?;
    // Cloudflare often sets its clearance cookie on the origin root while the
    // login page is nested under a path. Read both URL scopes and keep the
    // latest value for each cookie name.
    let mut cookies = std::collections::BTreeMap::new();
    for cookie in window
        .cookies_for_url(base_url)
        .map_err(|error| AppError::Source(format!("读取浏览器 Cookie 失败: {error}")))?
        .into_iter()
        .chain(
            window
                .cookies_for_url(url)
                .map_err(|error| AppError::Source(format!("读取浏览器 Cookie 失败: {error}")))?,
        )
    {
        cookies.insert(cookie.name().to_owned(), cookie.value().to_owned());
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
