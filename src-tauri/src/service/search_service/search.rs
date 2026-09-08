//! Concurrent source search orchestration.

use crate::{
    domain::source::{BookSearchResult, BookSource},
    error::AppError,
    infrastructure::http::request::{is_challenge_response, response_error},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    service::source_session::SourceSession,
    source_engine::pipeline::parse_search_response,
    source_engine::url::{decode_text, decode_text_string},
};
use std::sync::Arc;
use tauri::{AppHandle, Manager, WebviewWindow};

use super::{
    browser::{browser_body_looks_like_challenge, browser_request, navigate_browser_to_challenge},
    grouping::group_results,
    request::{build_search_request, should_try_browser_fallback},
    types::{SearchResponse, SourceFailure},
    SearchService,
};

const DEFAULT_SEARCH_CONCURRENCY: usize = 32;
const MAX_SEARCH_CONCURRENCY: usize = 64;

fn search_concurrency() -> usize {
    search_concurrency_from_env(std::env::var("READER_SEARCH_CONCURRENCY").ok().as_deref())
}

fn search_concurrency_from_env(value: Option<&str>) -> usize {
    value
        .and_then(|value| value.trim().parse::<usize>().ok())
        .map(|value| value.clamp(1, MAX_SEARCH_CONCURRENCY))
        .unwrap_or(DEFAULT_SEARCH_CONCURRENCY)
}

impl SearchService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    pub async fn search_with_browser(
        &self,
        query: &str,
        source_id: Option<i64>,
        app: Option<AppHandle>,
    ) -> Result<SearchResponse, AppError> {
        let query = query.trim();
        tracing::info!(target: "source", query = %query, source_id = ?source_id, "starting source search");
        if query.is_empty() || query.len() > 120 {
            return Err(AppError::InvalidArgument(
                "搜索关键词需要为 1 到 120 个字符".into(),
            ));
        }
        let sources = self
            .sources
            .list()
            .await?
            .into_iter()
            .filter(|s| s.enabled && source_id.is_none_or(|id| id == s.id))
            .collect::<Vec<_>>();
        if sources.is_empty() {
            return Err(AppError::Source("没有启用的书源，请先添加书源".into()));
        }
        let searched_sources = sources.len();
        let proxy = self.settings.proxy_url().await?;
        let concurrency = search_concurrency();
        tracing::info!(
            target: "source",
            concurrency,
            searched_sources,
            "source search concurrency configured"
        );
        let limiter = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let keyword = query.to_owned();
        let jobs = sources.into_iter().map(|source| {
            let service = self.clone();
            let proxy = proxy.clone();
            let limiter = limiter.clone();
            let keyword = keyword.clone();
            let browser = app
                .as_ref()
                .and_then(|app| app.get_webview_window(&format!("source-auth-{}", source.id)));
            async move {
                let result = service
                    .search_one_source(source.clone(), keyword, proxy, limiter, browser)
                    .await;
                (source.id, source.name, result)
            }
        });
        let mut results = Vec::new();
        let mut failures = Vec::new();
        for (id, name, result) in futures::future::join_all(jobs).await {
            match result {
                Ok(found) => results.extend(found),
                Err(error) => {
                    tracing::warn!(target: "source", source = %name, error = %error, "source search failed");
                    let auth_required = error.requires_authentication();
                    let reason = if error.requires_browser_challenge() {
                        format!(
                            "{} 需要浏览器执行 JavaScript 验证（Cloudflare challenge），HTTP 客户端无法直接通过",
                            name
                        )
                    } else {
                        error.to_string()
                    };
                    if auth_required {
                        self.sources.mark_session_expired(id).await?;
                    }
                    failures.push(SourceFailure {
                        source_id: id,
                        source_name: name,
                        reason,
                        auth_required,
                    });
                }
            }
        }
        let groups = group_results(results);
        tracing::info!(target: "source", groups = groups.len(), failures = failures.len(), searched_sources, "source search finished");
        Ok(SearchResponse {
            groups,
            failures,
            searched_sources,
        })
    }

    async fn search_one_source(
        &self,
        source: BookSource,
        keyword: String,
        proxy: Option<String>,
        limiter: Arc<tokio::sync::Semaphore>,
        browser: Option<WebviewWindow>,
    ) -> Result<Vec<BookSearchResult>, AppError> {
        let session =
            SourceSession::new(source.clone(), self.sources.clone(), 15, proxy.as_deref())?;
        let _permit = limiter
            .acquire_owned()
            .await
            .map_err(|_| AppError::Source("搜索并发限制器不可用".into()))?;
        let request = build_search_request(&source, &keyword)?;
        let response = session.send(&request).await?;
        let response_headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                Some((name.as_str().to_owned(), value.to_str().ok()?.to_owned()))
            })
            .collect::<Vec<_>>();
        if !response.status().is_success() {
            let status = response.status();
            let reason = response_error(response, &source.name).await;
            // A source can require a normal login without presenting a
            // Cloudflare interstitial. If its authenticated WebView is open,
            // give that session one chance before reporting 401/403. This is
            // especially important for imported legado sources whose login
            // flow is browser-only and therefore has no login_url/token_path
            // that the reqwest client can replay on its own.
            if should_try_browser_fallback(status, &reason) {
                if let Some(browser) = browser.as_ref() {
                    if let Some((status, body)) = browser_request(browser, &request).await? {
                        if (200..400).contains(&status) {
                            let text = decode_text_string(body, &request, &source)?;
                            if !browser_body_looks_like_challenge(&text) {
                                self.sync_browser_cookies(&source, browser, &request.url)
                                    .await?;
                                return parse_search_response(
                                    &source,
                                    &text,
                                    request.url.as_str(),
                                );
                            }
                        }
                    }
                    if reason.contains("需要浏览器执行 JavaScript 验证") {
                        let _ = navigate_browser_to_challenge(browser, &request);
                    }
                }
            }
            return Err(AppError::Network(reason));
        }
        let final_url = response.url().to_string();
        let text = decode_text(response, &request, &source).await?;
        let cloudflare_challenge = is_challenge_response(
            response_headers
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str())),
            &text,
        ) || browser_body_looks_like_challenge(&text);
        if cloudflare_challenge {
            if let Some(browser) = browser.as_ref() {
                if let Some((status, body)) = browser_request(browser, &request).await? {
                    if (200..400).contains(&status) {
                        let browser_text = decode_text_string(body, &request, &source)?;
                        if !browser_body_looks_like_challenge(&browser_text) {
                            self.sync_browser_cookies(&source, browser, &request.url)
                                .await?;
                            return parse_search_response(
                                &source,
                                &browser_text,
                                request.url.as_str(),
                            );
                        }
                    }
                }
                let _ = navigate_browser_to_challenge(browser, &request);
            }
            return Err(AppError::Network(format!(
                "{} 需要浏览器执行 JavaScript 验证（Cloudflare challenge），HTTP 客户端无法直接通过",
                source.name
            )));
        }
        parse_search_response(&source, &text, &final_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{CatalogRule, InfoRule, SearchRule};

    fn source(search_url: &str) -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://www.69shuba.com/".into(),
            search_url: search_url.into(),
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
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
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
    fn parses_legado_js_search_request_options() {
        let source = source(
            "<js>/modules/article/search.php,{'charset':'gbk','body':'searchkey={{key}}&searchtype=all','method':'POST'};result='';result;</js>",
        );
        let request = build_search_request(&source, "斗破").unwrap();
        assert_eq!(
            request.url.as_str(),
            "https://www.69shuba.com/modules/article/search.php"
        );
        assert_eq!(request.method, reqwest::Method::POST);
        assert_eq!(
            request.body.as_deref(),
            Some("searchkey=%B6%B7%C6%C6&searchtype=all")
        );
        assert_eq!(request.charset.as_deref(), Some("gbk"));
    }

    #[test]
    fn keeps_regular_search_urls_as_get_requests() {
        let source = source("search?q={{key}}");
        let request = build_search_request(&source, "斗").unwrap();
        assert_eq!(
            request.url.as_str(),
            "https://www.69shuba.com/search?q=%E6%96%97"
        );
        assert_eq!(request.method, reqwest::Method::GET);
        assert!(request.body.is_none());
    }

    #[test]
    fn search_concurrency_defaults_to_thirty_two() {
        assert_eq!(
            search_concurrency_from_env(None),
            DEFAULT_SEARCH_CONCURRENCY
        );
        assert_eq!(
            search_concurrency_from_env(Some("not-a-number")),
            DEFAULT_SEARCH_CONCURRENCY
        );
    }

    #[test]
    fn search_concurrency_is_clamped_to_safe_bounds() {
        assert_eq!(search_concurrency_from_env(Some("0")), 1);
        assert_eq!(search_concurrency_from_env(Some("1")), 1);
        assert_eq!(search_concurrency_from_env(Some("64")), 64);
        assert_eq!(
            search_concurrency_from_env(Some("999")),
            MAX_SEARCH_CONCURRENCY
        );
        assert_eq!(search_concurrency_from_env(Some(" 16 ")), 16);
    }

    #[test]
    fn browser_fallback_is_limited_to_authentication_responses() {
        assert!(should_try_browser_fallback(
            reqwest::StatusCode::UNAUTHORIZED,
            "source 返回 HTTP 401"
        ));
        assert!(should_try_browser_fallback(
            reqwest::StatusCode::FORBIDDEN,
            "source 返回 HTTP 403"
        ));
        assert!(should_try_browser_fallback(
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            "source 需要浏览器执行 JavaScript 验证（Cloudflare challenge）"
        ));
        assert!(!should_try_browser_fallback(
            reqwest::StatusCode::NOT_FOUND,
            "source 返回 HTTP 404"
        ));
        assert!(!should_try_browser_fallback(
            reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "source 返回 HTTP 451"
        ));
    }
}
