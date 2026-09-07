//! Single-source probing (validate/test) and the batch validation flow.

use crate::{
    domain::source::BookSearchResult,
    error::AppError,
    infrastructure::http::request::{is_challenge_response, response_error},
    repository::SourceRepository,
    service::source_session::SourceSession,
    source_engine::pipeline::parse_search_response,
    source_engine::url::{decode_text, decode_text_string},
};
use std::{sync::Arc, time::Instant};
use tauri::WebviewWindow;

use super::{
    browser::{browser_body_looks_like_challenge, browser_request, navigate_browser_to_challenge},
    request::{build_search_request, epoch_millis, should_try_browser_fallback},
    types::SourceTestResult,
    SearchService,
};

impl SearchService {
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
        let session = SourceSession::new(
            source.clone(),
            self.sources.clone(),
            15,
            self.settings.proxy_url().await?.as_deref(),
        )?;
        let response = session.send(&request).await?;
        let status = response.status().as_u16();
        let final_url = response.url().to_string();
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
                                let source = self.sources.get(source_id).await?.unwrap_or(source);
                                let parsed = parse_search_response(
                                    &source,
                                    &browser_text,
                                    request.url.as_str(),
                                )?;
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
                (Vec::<BookSearchResult>::new(), false, true)
            } else {
                let results = parse_search_response(&source, &text, &final_url)?;
                (results, false, false)
            }
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
                                let source = self.sources.get(source_id).await?.unwrap_or(source);
                                let parsed = parse_search_response(
                                    &source,
                                    &text,
                                    request.url.as_str(),
                                )?;
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

    pub async fn test_and_record(
        &self,
        source_id: i64,
        query: &str,
        browser: Option<WebviewWindow>,
    ) -> Result<SourceTestResult, AppError> {
        let started = Instant::now();
        let result = self.test_with_browser(source_id, query, browser).await;
        let duration_ms = result
            .as_ref()
            .map(|result| result.duration_ms)
            .unwrap_or_else(|_| started.elapsed().as_millis() as u64);
        self.sources
            .update_probe_stats(source_id, duration_ms as i64, epoch_millis())
            .await?;
        result
    }

    pub async fn test_all(&self, query: &str) -> Result<Vec<SourceTestResult>, AppError> {
        let query = query.trim();
        if query.is_empty() || query.len() > 120 {
            return Err(AppError::InvalidArgument(
                "验证关键词需要为 1 到 120 个字符".into(),
            ));
        }
        let sources = self
            .sources
            .list()
            .await?
            .into_iter()
            .filter(|source| source.enabled)
            .collect::<Vec<_>>();
        let limiter = Arc::new(tokio::sync::Semaphore::new(8));
        let jobs = sources.into_iter().map(|source| {
            let service = self.clone();
            let limiter = limiter.clone();
            let query = query.to_owned();
            async move {
                let started = Instant::now();
                let permit = limiter.acquire_owned().await;
                let result = match permit {
                    Ok(_permit) => service.test_and_record(source.id, &query, None).await,
                    Err(_) => Err(AppError::Source("批量验证调度器不可用".into())),
                };
                result.unwrap_or_else(|error| {
                    SourceTestResult::failed(&source, &error, started.elapsed().as_millis() as u64)
                })
            }
        });
        Ok(futures::future::join_all(jobs).await)
    }
}
