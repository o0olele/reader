//! Concurrent source search orchestration and single-source probing.

mod grouping;

pub use grouping::SearchResultGroup;

use crate::{
    domain::source::{BookSearchResult, BookSource},
    error::AppError,
    infrastructure::http::{
        client::{build_shared_client, build_source_client},
        request::{is_challenge_response, response_error},
    },
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::pipeline::parse_search,
    source_engine::url::{
        build as build_url_request, decode_text, decode_text_string, send as send_url_request,
        RequestSpec,
    },
};
use grouping::group_results;
use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};
use tauri::{AppHandle, Manager, WebviewWindow};

static NEXT_BROWSER_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

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

#[derive(Clone)]
pub struct SearchService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
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
        let shared = build_shared_client(15)?;
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
            let shared = shared.clone();
            let proxy = proxy.clone();
            let limiter = limiter.clone();
            let keyword = keyword.clone();
            let browser = app
                .as_ref()
                .and_then(|app| app.get_webview_window(&format!("source-auth-{}", source.id)));
            async move {
                let result = service
                    .search_one_source(source.clone(), keyword, shared, proxy, limiter, browser)
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

    pub async fn test_with_browser(
        &self,
        source_id: i64,
        query: &str,
        browser: Option<WebviewWindow>,
    ) -> Result<SourceTestResult, AppError> {
        let started = Instant::now();
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let request = build_search_request(&source, query.trim())?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let response = send_url_request(&client, &source, &request).await?;
        let status = response.status().as_u16();
        let response_headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                Some((name.as_str().to_owned(), value.to_str().ok()?.to_owned()))
            })
            .collect::<Vec<_>>();
        let header_challenge = is_challenge_response(
            response_headers
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str())),
            "",
        );
        let (results, auth_required, cloudflare_challenge) = if response.status().is_success() {
            let text = decode_text(response, &request, &source).await?;
            let cloudflare_challenge = header_challenge
                || is_challenge_response(
                    response_headers
                        .iter()
                        .map(|(name, value)| (name.as_str(), value.as_str())),
                    &text,
                )
                || browser_body_looks_like_challenge(&text);
            if cloudflare_challenge {
                if let Some(browser) = browser.as_ref() {
                    if let Some((browser_status, browser_body)) =
                        browser_request(browser, &request).await?
                    {
                        if (200..400).contains(&browser_status) {
                            let browser_text = decode_text_string(browser_body, &request, &source)?;
                            if !browser_body_looks_like_challenge(&browser_text) {
                                self.sync_browser_cookies(&source, browser, &request.url)
                                    .await?;
                                let parsed = parse_search(&source, &browser_text)?;
                                let source_name = source.name.clone();
                                let session_state = source.session_state().to_owned();
                                let duration_ms = started.elapsed().as_millis() as u64;
                                return Ok(SourceTestResult {
                                    source_id,
                                    source_name,
                                    status: browser_status,
                                    result_count: parsed.len(),
                                    auth_required: false,
                                    cloudflare_challenge: false,
                                    session_state,
                                    request_url: request.url.to_string(),
                                    duration_ms,
                                    has_token: !source.session_expired()
                                        && source.access_token.is_some(),
                                    has_cookie: !source.session_expired()
                                        && source.session_cookie.is_some(),
                                    user_agent: crate::infrastructure::http::request::user_agent(),
                                });
                            }
                        }
                    }
                    let _ = navigate_browser_to_challenge(browser, &request);
                }
            }
            (Vec::<BookSearchResult>::new(), false, cloudflare_challenge)
        } else {
            let status_code = response.status();
            let reason = response_error(response, &source.name).await;
            let cloudflare_challenge = reason.contains("需要浏览器执行 JavaScript 验证");
            if should_try_browser_fallback(status_code, &reason) {
                if let Some(browser) = browser.as_ref() {
                    if let Some((browser_status, browser_body)) =
                        browser_request(browser, &request).await?
                    {
                        if (200..400).contains(&browser_status) {
                            let text = decode_text_string(browser_body, &request, &source)?;
                            if !browser_body_looks_like_challenge(&text) {
                                self.sync_browser_cookies(&source, browser, &request.url)
                                    .await?;
                                let parsed = parse_search(&source, &text)?;
                                let source_name = source.name.clone();
                                let session_state = source.session_state().to_owned();
                                let duration_ms = started.elapsed().as_millis() as u64;
                                return Ok(SourceTestResult {
                                    source_id,
                                    source_name,
                                    status: browser_status,
                                    result_count: parsed.len(),
                                    auth_required: false,
                                    cloudflare_challenge: false,
                                    session_state,
                                    request_url: request.url.to_string(),
                                    duration_ms,
                                    has_token: !source.session_expired()
                                        && source.access_token.is_some(),
                                    has_cookie: !source.session_expired()
                                        && source.session_cookie.is_some(),
                                    user_agent: crate::infrastructure::http::request::user_agent(),
                                });
                            }
                        }
                    }
                    if cloudflare_challenge {
                        let _ = navigate_browser_to_challenge(browser, &request);
                    }
                }
            }
            let auth_required = matches!(status, 401) || (status == 403 && !cloudflare_challenge);
            if auth_required {
                self.sources.mark_session_expired(source_id).await?;
            }
            tracing::warn!(target: "network", source = %source.name, status, auth_required, cloudflare_challenge, "source probe returned an error");
            (Vec::new(), auth_required, cloudflare_challenge)
        };
        let source_name = source.name.clone();
        let session_state = if auth_required {
            "expired".to_owned()
        } else {
            source.session_state().to_owned()
        };
        let duration_ms = started.elapsed().as_millis() as u64;
        tracing::info!(target: "network", source = %source_name, status, duration_ms, result_count = results.len(), "source probe finished");
        Ok(SourceTestResult {
            source_id,
            source_name,
            status,
            result_count: results.len(),
            auth_required,
            cloudflare_challenge,
            session_state,
            request_url: request.url.to_string(),
            duration_ms,
            has_token: !source.session_expired() && source.access_token.is_some(),
            has_cookie: !source.session_expired() && source.session_cookie.is_some(),
            user_agent: crate::infrastructure::http::request::user_agent(),
        })
    }

    async fn sync_browser_cookies(
        &self,
        source: &BookSource,
        browser: &WebviewWindow,
        url: &reqwest::Url,
    ) -> Result<(), AppError> {
        let found = browser
            .cookies_for_url(url.clone())
            .map_err(|error| AppError::Source(format!("读取浏览器 Cookie 失败: {error}")))?;
        let mut merged = std::collections::BTreeMap::new();
        if let Some(existing) = source.session_cookie.as_deref() {
            for pair in existing.split(';') {
                if let Some((name, value)) = pair.trim().split_once('=') {
                    if !name.trim().is_empty() {
                        merged.insert(name.trim().to_owned(), value.trim().to_owned());
                    }
                }
            }
        }
        for cookie in found {
            merged.insert(cookie.name().to_owned(), cookie.value().to_owned());
        }
        if merged.is_empty() {
            return Ok(());
        }
        let header = merged
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");
        let expiry = (!source.session_expired())
            .then_some(source.session_expires_at.as_deref())
            .flatten();
        self.sources
            .update_session(
                source.id,
                source.access_token.as_deref(),
                Some(&header),
                expiry,
            )
            .await
    }
}

#[derive(Debug, serde::Deserialize)]
struct BrowserResponse {
    status: u16,
    #[serde(default)]
    body: String,
}

/// Runs a source request inside the authenticated WebView so Cloudflare's
/// JavaScript/runtime-bound clearance remains valid for the request.
pub(crate) async fn browser_request(
    window: &WebviewWindow,
    spec: &RequestSpec,
) -> Result<Option<(u16, String)>, AppError> {
    let request_id = NEXT_BROWSER_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
    let result_key = format!("__readerDesktopBrowserRequest_{request_id}");
    let result_key = serde_json::to_string(&result_key).map_err(AppError::parse)?;
    let url = serde_json::to_string(spec.url.as_str()).map_err(AppError::parse)?;
    let method = serde_json::to_string(spec.method.as_str()).map_err(AppError::parse)?;
    let body =
        serde_json::to_string(spec.body.as_deref().unwrap_or("")).map_err(AppError::parse)?;
    let mut headers = serde_json::Map::new();
    for (name, value) in &spec.headers {
        headers.insert(name.clone(), serde_json::Value::String(value.clone()));
    }
    if spec.body.is_some()
        && !headers
            .keys()
            .any(|name| name.eq_ignore_ascii_case("content-type"))
    {
        headers.insert(
            "Content-Type".into(),
            serde_json::Value::String("application/x-www-form-urlencoded".into()),
        );
    }
    let headers = serde_json::to_string(&headers).map_err(AppError::parse)?;
    let script = format!(
        r#"(() => {{ try {{
            const resultKey = {result_key};
            window[resultKey] = {{done: false, status: 0, body: ""}};
            const xhr = new XMLHttpRequest();
            xhr.open({method}, {url}, true);
            xhr.withCredentials = true;
            xhr.timeout = 15000;
            const headers = {headers};
            for (const [name, value] of Object.entries(headers)) xhr.setRequestHeader(name, value);
            const finish = (status, body) => {{
                const current = window[resultKey];
                if (!current || current.done) return;
                window[resultKey] = {{done: true, status, body: String(body || "")}};
            }};
            xhr.onload = () => finish(xhr.status, xhr.responseText);
            xhr.onerror = () => finish(0, "浏览器请求失败");
            xhr.ontimeout = () => finish(0, "浏览器请求超时");
            xhr.send({body});
            return "started";
        }} catch (error) {{
            window[{result_key}] = {{done: true, status: 0, body: String(error)}};
            return "started";
        }} }})()"#,
        method = method,
        url = url,
        body = body,
        headers = headers,
        result_key = result_key,
    );
    eval_browser_script(window, script).await?;

    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(20);
    let poll_script = format!(
        r#"(() => {{
            const value = window[{result_key}];
            if (!value || !value.done) return "";
            delete window[{result_key}];
            return JSON.stringify({{status: value.status, body: value.body}});
        }})()"#,
        result_key = result_key,
    );
    let result: BrowserResponse = loop {
        if tokio::time::Instant::now() >= deadline {
            return Err(AppError::Source(
                "浏览器请求超时，请确认认证窗口仍然打开".into(),
            ));
        }
        let raw = eval_browser_script(window, poll_script.clone()).await?;
        let decoded = serde_json::from_str::<String>(&raw).unwrap_or(raw);
        if !decoded.is_empty() {
            break serde_json::from_str(&decoded)
                .map_err(|error| AppError::Source(format!("浏览器响应解析失败: {error}")))?;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    };
    if result.status == 0 {
        return Ok(None);
    }
    Ok(Some((result.status, result.body)))
}

async fn eval_browser_script(window: &WebviewWindow, script: String) -> Result<String, AppError> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let sender = std::sync::Mutex::new(Some(sender));
    window
        .eval_with_callback(script, move |value| {
            if let Ok(mut sender) = sender.lock() {
                if let Some(sender) = sender.take() {
                    let _ = sender.send(value);
                }
            }
        })
        .map_err(|error| AppError::Source(format!("浏览器请求执行失败: {error}")))?;
    tokio::time::timeout(std::time::Duration::from_secs(5), receiver)
        .await
        .map_err(|_| AppError::Source("浏览器脚本回调超时，请确认认证窗口仍然打开".into()))?
        .map_err(|_| AppError::Source("浏览器脚本回调已失效".into()))
}

/// Put the protected URL in front of the user when the in-page XHR itself is
/// still challenged. This gives Cloudflare a real top-level navigation where
/// its interstitial can run, instead of leaving the user on an already-passed
/// landing page with no way to solve the new challenge.
pub(crate) fn navigate_browser_to_challenge(
    window: &WebviewWindow,
    spec: &RequestSpec,
) -> Result<(), AppError> {
    window
        .navigate(spec.url.clone())
        .map_err(|error| AppError::Source(format!("打开浏览器验证页面失败: {error}")))
}

pub(crate) fn browser_body_looks_like_challenge(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    lower.contains("_cf_chl_opt")
        || lower.contains("enable javascript and cookies")
        || lower.contains("just a moment")
        || lower.contains("cf-chl-")
        || lower.contains("challenge-platform")
        || (lower.contains("cloudflare") && lower.contains("verify you are human"))
}

#[derive(Debug, Serialize)]
pub struct SourceTestResult {
    pub source_id: i64,
    pub source_name: String,
    pub status: u16,
    pub result_count: usize,
    pub auth_required: bool,
    pub cloudflare_challenge: bool,
    pub session_state: String,
    pub request_url: String,
    pub duration_ms: u64,
    pub has_token: bool,
    pub has_cookie: bool,
    pub user_agent: String,
}
#[derive(Debug, Serialize)]
pub struct SourceFailure {
    pub source_id: i64,
    pub source_name: String,
    pub reason: String,
    pub auth_required: bool,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub groups: Vec<SearchResultGroup>,
    pub failures: Vec<SourceFailure>,
    pub searched_sources: usize,
}

impl SearchService {
    async fn search_one_source(
        &self,
        source: BookSource,
        keyword: String,
        shared: reqwest::Client,
        proxy: Option<String>,
        limiter: Arc<tokio::sync::Semaphore>,
        browser: Option<WebviewWindow>,
    ) -> Result<Vec<BookSearchResult>, AppError> {
        let own = source
            .proxy_url
            .as_deref()
            .is_some_and(|v| !v.trim().is_empty())
            || proxy.is_some();
        let client = if own {
            build_source_client(&source, 15, proxy.as_deref())?
        } else {
            shared
        };
        let _permit = limiter
            .acquire_owned()
            .await
            .map_err(|_| AppError::Source("搜索并发限制器不可用".into()))?;
        let request = build_search_request(&source, &keyword)?;
        let response = send_url_request(&client, &source, &request).await?;
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
                                return parse_search(&source, &text);
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
                            return parse_search(&source, &browser_text);
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
        parse_search(&source, &text)
    }
}

/// Retry an HTTP authentication failure in the source's authenticated
/// WebView. Cloudflare responses are included because their clearance cookie
/// is also browser-bound; other statuses (404/451/429) are not auth retries.
fn should_try_browser_fallback(status: reqwest::StatusCode, reason: &str) -> bool {
    status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::FORBIDDEN
        || reason.contains("需要浏览器执行 JavaScript 验证")
}

type SearchRequest = RequestSpec;

fn build_search_request(source: &BookSource, keyword: &str) -> Result<SearchRequest, AppError> {
    build_url_request(source, &source.search_url, Some(keyword), "搜索 URL")
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
    fn recognizes_http_200_cloudflare_interstitial_bodies() {
        assert!(browser_body_looks_like_challenge(
            "<html><title>Just a moment...</title><script>window._cf_chl_opt={}</script></html>"
        ));
        assert!(!browser_body_looks_like_challenge(
            "<html><div class='newbox'><li><h3>Book</h3></li></div></html>"
        ));
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
