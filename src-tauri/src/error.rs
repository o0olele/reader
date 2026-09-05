use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("source error: {0}")]
    Source(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::Source(value)
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::Source(value.to_owned())
    }
}

impl AppError {
    pub fn database(error: impl Display) -> Self {
        Self::Database(error.to_string())
    }

    pub fn network(error: impl Display) -> Self {
        Self::Network(error.to_string())
    }

    pub fn parse(error: impl Display) -> Self {
        Self::Parse(error.to_string())
    }

    pub fn io(error: impl Display) -> Self {
        Self::Io(error.to_string())
    }

    /// Whether the remote endpoint rejected the request because credentials
    /// are missing or no longer valid. Keeping this classification on the
    /// error avoids every caller parsing localized error text.
    pub fn requires_authentication(&self) -> bool {
        matches!(self, Self::Network(message) | Self::Source(message)
            if response_requires_authentication(message))
    }

    /// Whether the rendered error contains a Cloudflare/browser challenge.
    /// This is separate from [`Self::requires_authentication`] because a
    /// challenge must not expire a perfectly valid login session.
    pub fn requires_browser_challenge(&self) -> bool {
        matches!(self, Self::Network(message) | Self::Source(message)
            if response_is_browser_challenge(message))
    }
}

/// Classify ordinary HTTP authentication failures without mistaking a
/// Cloudflare interstitial for an expired source session. JS rules surface
/// HTTP failures as `Source` errors, while regular URL requests use
/// `Network`, so the predicate intentionally operates on the rendered text.
fn response_requires_authentication(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    (lower.contains("http 401") || lower.contains("http 403"))
        && !response_is_browser_challenge(message)
}

fn response_is_browser_challenge(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("cloudflare challenge")
        || lower.contains("需要浏览器执行 javascript 验证")
        || lower.contains("just a moment")
        || lower.contains("enable javascript and cookies")
        || lower.contains("_cf_chl_opt")
        || lower.contains("challenge-platform")
}

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn cloudflare_challenges_are_not_marked_as_expired_sessions() {
        let error = AppError::Network(
            "example 返回 HTTP 403: 需要浏览器执行 JavaScript 验证（Cloudflare challenge）".into(),
        );
        assert!(!error.requires_authentication());
    }

    #[test]
    fn ordinary_forbidden_responses_still_require_authentication() {
        assert!(AppError::Network("example 返回 HTTP 403".into()).requires_authentication());
    }

    #[test]
    fn javascript_http_errors_are_classified_as_authentication_failures() {
        assert!(
            AppError::Source("JavaScript 执行失败: network error: HTTP 403 Forbidden".into())
                .requires_authentication()
        );
    }

    #[test]
    fn cloudflare_interstitials_are_not_authentication_failures_even_from_js() {
        let error = AppError::Source(
            "JavaScript 执行失败: network error: HTTP 403 Forbidden: Just a moment...".into(),
        );
        assert!(!error.requires_authentication());
        assert!(error.requires_browser_challenge());
    }

    #[test]
    fn ordinary_forbidden_responses_are_not_browser_challenges() {
        let error = AppError::Network("example 返回 HTTP 403: page not found".into());
        assert!(!error.requires_browser_challenge());
    }
}
