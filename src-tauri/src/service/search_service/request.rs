//! Request construction and auth-retry classification shared by search and probe.

use crate::{domain::source::BookSource, error::AppError, source_engine::url::RequestSpec};

pub(super) type SearchRequest = RequestSpec;

pub(super) fn build_search_request(
    source: &BookSource,
    keyword: &str,
) -> Result<SearchRequest, AppError> {
    crate::source_engine::url::build(source, &source.search_url, Some(keyword), "搜索 URL")
}

/// Retry an HTTP authentication failure in the source's authenticated
/// WebView. Cloudflare responses are included because their clearance cookie
/// is also browser-bound; other statuses (404/451/429) are not auth retries.
pub(super) fn should_try_browser_fallback(status: reqwest::StatusCode, reason: &str) -> bool {
    status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::FORBIDDEN
        || reason.contains("需要浏览器执行 JavaScript 验证")
}

pub(super) fn epoch_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or_default()
}
