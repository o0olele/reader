use crate::source_engine::rule::JsHttpContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRule {
    pub item: String,
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub url: String,
}

/// The legado rule objects exactly as imported, before the CSS projection.
///
/// Each field holds the raw JSON of one `rule*` object (or a bare string, which
/// legado allows for `ruleContent`). The rule engine prefers these; the flat
/// selector columns on [`BookSource`] remain as a fallback.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawSourceRules {
    pub search: Option<String>,
    pub book_info: Option<String>,
    pub toc: Option<String>,
    pub content: Option<String>,
}

impl RawSourceRules {
    pub fn is_empty(&self) -> bool {
        self.search.is_none()
            && self.book_info.is_none()
            && self.toc.is_none()
            && self.content.is_none()
    }
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
    pub concurrent_rate: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub raw_rules: RawSourceRules,
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
    pub concurrent_rate: Option<String>,
    pub next_toc_url_selector: Option<String>,
    pub next_content_url_selector: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub raw_rules: RawSourceRules,
}

impl BookSource {
    pub fn session_expired(&self) -> bool {
        let Some(raw) = self.session_expires_at.as_deref() else {
            return false;
        };
        let expiry = parse_expiry(raw);
        expiry.is_some_and(|value| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or(u64::MAX);
            value <= now
        })
    }

    pub fn session_state(&self) -> &'static str {
        if self.access_token.is_none() && self.session_cookie.is_none() {
            "anonymous"
        } else if self.session_expired() {
            "expired"
        } else {
            "authenticated"
        }
    }

    pub fn http_context(&self) -> JsHttpContext {
        JsHttpContext {
            base_url: self.base_url.clone(),
            headers: self.header.clone(),
            access_token: self.access_token.clone(),
            session_cookie: self.session_cookie.clone(),
            session_expired: self.session_expired(),
            sign_script: self.sign_script.clone(),
        }
    }

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
            concurrent_rate: import.concurrent_rate.clone(),
            enabled: import.enabled,
            raw_rules: import.raw_rules.clone(),
        }
    }
}

fn parse_expiry(raw: &str) -> Option<u64> {
    if let Ok(value) = raw.trim().parse::<u64>() {
        return Some(value);
    }
    let value = raw.trim().strip_suffix('Z').unwrap_or(raw.trim());
    let (date, time) = value.split_once('T')?;
    let mut date_parts = date.split('-');
    let year = date_parts.next()?.parse::<i64>().ok()?;
    let month = date_parts.next()?.parse::<i64>().ok()?;
    let day = date_parts.next()?.parse::<i64>().ok()?;
    let mut time_parts = time.split(':');
    let hour = time_parts.next()?.parse::<i64>().ok()?;
    let minute = time_parts.next()?.parse::<i64>().ok()?;
    let second = time_parts.next()?.split('.').next()?.parse::<i64>().ok()?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || !(0..=23).contains(&hour)
        || !(0..=59).contains(&minute)
        || !(0..=59).contains(&second)
    {
        return None;
    }
    Some((days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second) as u64)
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = (if year >= 0 { year } else { year - 399 }) / 400;
    let yoe = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
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
            concurrent_rate: None,
            next_toc_url_selector: None,
            next_content_url_selector: None,
            enabled: true,
            raw_rules: RawSourceRules::default(),
        };

        let source = BookSource::from_import(&import);

        assert_eq!(source.id, 0);
        assert!(source.access_token.is_none());
        assert!(source.session_cookie.is_none());
        assert!(source.session_expires_at.is_none());
        assert_eq!(source.name, "示例");
        assert!(source.enabled);
    }

    #[test]
    fn session_state_distinguishes_anonymous_and_expired_credentials() {
        let mut source = BookSource::from_import(&SourceImport {
            name: "state".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com?q={{key}}".into(),
            search_rule: SearchRule {
                item: "a".into(),
                title: "a".into(),
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
            header: None,
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            next_toc_url_selector: None,
            next_content_url_selector: None,
            enabled: true,
            raw_rules: RawSourceRules::default(),
        });
        assert_eq!(source.session_state(), "anonymous");
        source.access_token = Some("token".into());
        source.session_expires_at = Some("0".into());
        assert_eq!(source.session_state(), "expired");
        source.session_expires_at = Some("1970-01-01T00:00:00Z".into());
        assert_eq!(source.session_state(), "expired");
    }
}
