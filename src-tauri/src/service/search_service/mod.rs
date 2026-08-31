//! Concurrent source search orchestration and single-source probing.

mod grouping;

pub use grouping::SearchResultGroup;

use crate::{
    domain::source::{BookSearchResult, BookSource},
    error::AppError,
    infrastructure::http::{
        client::{build_shared_client, build_source_client},
        request::{response_error, send_source_request},
        url::resolve_url,
    },
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::selector::parse_search,
};
use grouping::group_results;
use serde::Serialize;
use std::sync::Arc;

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
                    failures.push(SourceFailure {
                        source_id: id,
                        source_name: name,
                        reason: error.to_string(),
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
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let url = resolve_url(
            &source.base_url,
            &expand_search_url(&source, &encode_query(query.trim())),
            "搜索 URL",
        )?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let response = send_source_request(&client, url.as_str(), &source).await?;
        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let results = parse_search(&source, &response.text().await.map_err(AppError::network)?)
            .map_err(AppError::parse)?;
        Ok(SourceTestResult {
            source_id,
            source_name: source.name,
            status,
            result_count: results.len(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SourceTestResult {
    pub source_id: i64,
    pub source_name: String,
    pub status: u16,
    pub result_count: usize,
}
#[derive(Debug, Serialize)]
pub struct SourceFailure {
    pub source_id: i64,
    pub source_name: String,
    pub reason: String,
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
    let url = resolve_url(
        &source.base_url,
        &expand_search_url(&source, &keyword),
        "搜索 URL",
    )?;
    let response = send_source_request(&client, url.as_str(), &source).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    parse_search(&source, &response.text().await.map_err(AppError::network)?)
        .map_err(AppError::parse)
}
