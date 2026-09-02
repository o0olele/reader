//! Concurrent source search orchestration and single-source probing.

mod grouping;

pub use grouping::SearchResultGroup;

use crate::{
    domain::source::{BookSearchResult, BookSource},
    error::AppError,
    infrastructure::http::{
        client::{build_shared_client, build_source_client},
        request::response_error,
        request::send_source_request_with_method,
        url::resolve_url,
    },
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::pipeline::parse_search,
};
use encoding_rs::GBK;
use grouping::group_results;
use serde::Serialize;
use std::{sync::Arc, time::Instant};

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

    pub async fn search(
        &self,
        query: &str,
        source_id: Option<i64>,
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
        let limiter = Arc::new(tokio::sync::Semaphore::new(8));
        let keyword = encode_query(query);
        let jobs = sources.into_iter().map(|source| {
            let shared = shared.clone();
            let proxy = proxy.clone();
            let limiter = limiter.clone();
            let keyword = keyword.clone();
            async move {
                let result =
                    search_one_source(source.clone(), keyword, shared, proxy, limiter).await;
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
                    if auth_required {
                        self.sources.mark_session_expired(id).await?;
                    }
                    failures.push(SourceFailure {
                        source_id: id,
                        source_name: name,
                        reason: error.to_string(),
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

    pub async fn test(&self, source_id: i64, query: &str) -> Result<SourceTestResult, AppError> {
        let started = Instant::now();
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let request = build_search_request(&source, &encode_query(query.trim()))?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let response = send_source_request_with_method(
            &client,
            request.url.as_str(),
            &source,
            request.method,
            request.body,
        )
        .await?;
        let status = response.status().as_u16();
        let (results, auth_required, cloudflare_challenge) = if response.status().is_success() {
            (
                parse_search(&source, &decode_response(response, request.charset).await?)?,
                false,
                false,
            )
        } else {
            let reason = response_error(response, &source.name).await;
            let cloudflare_challenge = reason.contains("需要浏览器执行 JavaScript 验证");
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
        })
    }
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

fn encode_query(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}

// `<searchKey>` is already rewritten to `{{key}}` at import time, and manually
// saved sources are required to carry `{{key}}`/`{key}`. Kept until Step 2's
// rule engine subsumes URL templating.
fn expand_search_url(source: &BookSource, keyword: &str) -> String {
    source
        .search_url
        .replace("{{key}}", keyword)
        .replace("{key}", keyword)
        .replace("<key>", keyword)
        .replace("<searchKey>", keyword)
}

async fn search_one_source(
    source: BookSource,
    keyword: String,
    shared: reqwest::Client,
    proxy: Option<String>,
    limiter: Arc<tokio::sync::Semaphore>,
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
    let response = send_source_request_with_method(
        &client,
        request.url.as_str(),
        &source,
        request.method,
        request.body,
    )
    .await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    parse_search(&source, &decode_response(response, request.charset).await?)
}

struct SearchRequest {
    url: reqwest::Url,
    method: reqwest::Method,
    body: Option<String>,
    charset: Option<String>,
}

fn build_search_request(source: &BookSource, keyword: &str) -> Result<SearchRequest, AppError> {
    let raw = expand_search_url(source, keyword);
    let trimmed = raw.trim();
    let Some(script) = trimmed
        .strip_prefix("<js>")
        .and_then(|value| value.strip_suffix("</js>"))
    else {
        return Ok(SearchRequest {
            url: resolve_url(&source.base_url, &raw, "搜索 URL")?,
            method: reqwest::Method::GET,
            body: None,
            charset: None,
        });
    };
    let (target, options) = script
        .split_once(',')
        .ok_or_else(|| AppError::Parse("搜索 JS 缺少请求选项".into()))?;
    let options = options
        .split_once(';')
        .map(|(value, _)| value)
        .unwrap_or(options)
        .trim();
    let options = options.replace('\'', "\"").replace("undefined", "null");
    let options: serde_json::Value = serde_json::from_str(&options)
        .map_err(|error| AppError::Parse(format!("搜索 JS 请求选项无效: {error}")))?;
    let method = options
        .get("method")
        .and_then(|value| value.as_str())
        .unwrap_or("GET")
        .parse::<reqwest::Method>()
        .map_err(|error| AppError::InvalidArgument(format!("搜索 HTTP method 无效: {error}")))?;
    let body = options
        .get("body")
        .and_then(|value| value.as_str())
        .map(str::to_owned)
        .map(|body| expand_search_url_value(&body, keyword));
    let charset = options
        .get("charset")
        .and_then(|value| value.as_str())
        .map(str::to_ascii_lowercase);
    Ok(SearchRequest {
        url: resolve_url(&source.base_url, target.trim(), "搜索 URL")?,
        method,
        body,
        charset,
    })
}

fn expand_search_url_value(value: &str, keyword: &str) -> String {
    value
        .replace("{{key}}", keyword)
        .replace("{key}", keyword)
        .replace("<searchKey>", keyword)
        .replace("<key>", keyword)
}

async fn decode_response(
    response: reqwest::Response,
    charset: Option<String>,
) -> Result<String, AppError> {
    let bytes = response.bytes().await.map_err(AppError::network)?;
    if charset.as_deref() == Some("gbk") || charset.as_deref() == Some("gb2312") {
        return Ok(GBK.decode(&bytes).0.into_owned());
    }
    String::from_utf8(bytes.to_vec())
        .map_err(|error| AppError::Parse(format!("响应不是 UTF-8: {error}")))
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
            enabled: true,
            raw_rules: Default::default(),
        }
    }

    #[test]
    fn parses_legado_js_search_request_options() {
        let source = source(
            "<js>/modules/article/search.php,{'charset':'gbk','body':'searchkey={{key}}&searchtype=all','method':'POST'};result='';result;</js>",
        );
        let request = build_search_request(&source, "%E6%96%97%E7%A0%B4").unwrap();
        assert_eq!(
            request.url.as_str(),
            "https://www.69shuba.com/modules/article/search.php"
        );
        assert_eq!(request.method, reqwest::Method::POST);
        assert_eq!(
            request.body.as_deref(),
            Some("searchkey=%E6%96%97%E7%A0%B4&searchtype=all")
        );
        assert_eq!(request.charset.as_deref(), Some("gbk"));
    }

    #[test]
    fn keeps_regular_search_urls_as_get_requests() {
        let source = source("search?q={{key}}");
        let request = build_search_request(&source, "%E6%96%97").unwrap();
        assert_eq!(
            request.url.as_str(),
            "https://www.69shuba.com/search?q=%E6%96%97"
        );
        assert_eq!(request.method, reqwest::Method::GET);
        assert!(request.body.is_none());
    }
}
