//! Public result types for the search service.

use crate::{domain::source::BookSource, error::AppError};
use serde::Serialize;

use super::SearchResultGroup;

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

impl SourceTestResult {
    pub(super) fn failed(source: &BookSource, error: &AppError, duration_ms: u64) -> Self {
        Self {
            source_id: source.id,
            source_name: source.name.clone(),
            status: 0,
            result_count: 0,
            auth_required: error.requires_authentication(),
            cloudflare_challenge: error.requires_browser_challenge(),
            session_state: "error".into(),
            request_url: source.search_url.clone(),
            duration_ms,
            has_token: source.access_token.is_some() && !source.session_expired(),
            has_cookie: source.session_cookie.is_some() && !source.session_expired(),
            user_agent: crate::infrastructure::http::request::user_agent(),
        }
    }
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
