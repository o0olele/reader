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

impl BookSource {
    /// Builds an unsaved source from an import record.
    ///
    /// `id` is 0 and session credentials are cleared, so the caller must
    /// persist through `upsert` to obtain a real id.
    pub fn from_import(import: &SourceImport) -> Self {
        Self {
            id: 0,
            name: import.name.clone(),
            base_url: import.base_url.clone(),
            search_url: import.search_url.clone(),
            search_rule: import.search_rule.clone(),
            info_rule: import.info_rule.clone(),
            catalog_rule: import.catalog_rule.clone(),
            content_selector: import.content_selector.clone(),
            next_toc_url_selector: import.next_toc_url_selector.clone(),
            next_content_url_selector: import.next_content_url_selector.clone(),
            header: import.header.clone(),
            login_url: import.login_url.clone(),
            login_method: import.login_method.clone(),
            login_body: import.login_body.clone(),
            token_path: import.token_path.clone(),
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: import.sign_script.clone(),
            proxy_url: import.proxy_url.clone(),
            enabled: import.enabled,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_import_starts_unsaved_and_unauthenticated() {
        let import = SourceImport {
            name: "示例".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com?q={{key}}".into(),
            search_rule: SearchRule {
                item: ".item".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a::attr(href)".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a::attr(href)".into(),
                next_url: None,
            },
            content_selector: ".content".into(),
            header: None,
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            sign_script: None,
            proxy_url: None,
            next_toc_url_selector: None,
            next_content_url_selector: None,
            enabled: true,
        };

        let source = BookSource::from_import(&import);

        assert_eq!(source.id, 0);
        assert!(source.access_token.is_none());
        assert!(source.session_cookie.is_none());
        assert!(source.session_expires_at.is_none());
        assert_eq!(source.name, "示例");
        assert!(source.enabled);
    }
}
