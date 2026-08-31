use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRule {
    pub item: String,
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSource {
    pub id: i64,
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    pub search_rule: SearchRule,
    pub info_rule: InfoRule,
    pub catalog_rule: CatalogRule,
    pub content_selector: String,
    pub next_toc_url_selector: Option<String>,
    pub next_content_url_selector: Option<String>,
    pub header: Option<String>,
    pub login_url: Option<String>,
    pub login_method: String,
    pub login_body: Option<String>,
    pub token_path: Option<String>,
    pub access_token: Option<String>,
    pub session_cookie: Option<String>,
    pub session_expires_at: Option<String>,
    pub sign_script: Option<String>,
    pub proxy_url: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoRule {
    pub title: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub cover: Option<String>,
    pub kind: Option<String>,
    pub latest_chapter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogRule {
    pub item: String,
    pub title: String,
    pub url: String,
    pub next_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceImport {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    pub search_rule: SearchRule,
    pub info_rule: InfoRule,
    pub catalog_rule: CatalogRule,
    pub content_selector: String,
    pub header: Option<String>,
    pub login_url: Option<String>,
    pub login_method: String,
    pub login_body: Option<String>,
    pub token_path: Option<String>,
    pub sign_script: Option<String>,
    pub proxy_url: Option<String>,
    pub next_toc_url_selector: Option<String>,
    pub next_content_url_selector: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSearchResult {
    pub source_id: i64,
    pub source_name: String,
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub cover: Option<String>,
    pub kind: Option<String>,
    pub latest_chapter: Option<String>,
}
