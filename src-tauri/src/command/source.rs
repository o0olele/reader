use crate::{
    app::AppState,
    domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule},
    error::AppError,
    service::source_service::{
        SourceImportReport, SourceLoginInput, SourceLoginResult, SourceService,
    },
};
use serde::Deserialize;
use tauri::State;

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
