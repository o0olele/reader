//! Hand-typed book source input model and save command.

use crate::{
    app::AppState,
    domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule},
    error::AppError,
    service::source_service::SourceService,
};
use serde::Deserialize;
use tauri::State;

#[derive(Deserialize)]
pub struct BookSourceInput {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    #[serde(default)]
    pub explore_url: Option<String>,
    #[serde(default)]
    pub book_url_pattern: Option<String>,
    #[serde(default = "default_enabled_cookie_jar")]
    pub enabled_cookie_jar: bool,
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
    #[serde(default)]
    pub source_group: Option<String>,
    #[serde(default)]
    pub custom_order: i64,
    #[serde(default)]
    pub weight: i64,
    #[serde(default = "default_enabled_explore")]
    pub enabled_explore: bool,
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
fn default_enabled_explore() -> bool {
    true
}
fn default_enabled_cookie_jar() -> bool {
    true
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
        book_url_pattern: input
            .book_url_pattern
            .filter(|value| !value.trim().is_empty()),
        enabled_cookie_jar: input.enabled_cookie_jar,
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
        source_group: input.source_group,
        custom_order: input.custom_order,
        weight: input.weight,
        enabled_explore: input.enabled_explore,
        respond_time: None,
        last_update_time: None,
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
