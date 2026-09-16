//! Source import conversion and authentication context.
use super::{time::parse_expiry, BookSource, SourceImport};
use crate::source_engine::rule::JsHttpContext;

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
            explore_url: import.explore_url.clone(),
            book_url_pattern: import.book_url_pattern.clone(),
            enabled_cookie_jar: import.enabled_cookie_jar,
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
            source_group: import.source_group.clone(),
            custom_order: import.custom_order,
            weight: import.weight,
            enabled_explore: import.enabled_explore,
            respond_time: import.respond_time,
            last_update_time: import.last_update_time,
            raw_rules: import.raw_rules.clone(),
        }
    }
}
