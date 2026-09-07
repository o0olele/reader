//! Cookie merge, expiry extraction, and browser-session persistence.

use crate::{error::AppError, service::source_service::SourceService};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::time::{SystemTime, UNIX_EPOCH};

impl SourceService {
    /// Persists cookies collected from the source's embedded browser window.
    /// Existing token credentials are retained because browser challenges often
    /// supplement, rather than replace, an API token.
    pub async fn save_browser_cookies(
        &self,
        source_id: i64,
        cookies: &str,
    ) -> Result<crate::service::source_service::SourceLoginResult, AppError> {
        let source = self.get(source_id).await?;
        let cookies = cookies.trim();
        if cookies.is_empty() {
            return Err(AppError::InvalidArgument(
                "浏览器中没有可保存的 Cookie".into(),
            ));
        }
        if cloudflare_challenge_unsolved(cookies) {
            return Err(AppError::InvalidArgument(
                "Cloudflare 验证尚未完成：浏览器只返回了 __cf_bm，没有 cf_clearance。请在认证窗口里等待验证通过（页面显示出书源内容）后再读取会话".into(),
            ));
        }
        let merged_cookies = merge_cookies(source.session_cookie.as_deref(), Some(cookies))
            .ok_or_else(|| AppError::InvalidArgument("浏览器中没有可保存的有效 Cookie".into()))?;
        let session_expires_at = (!source.session_expired())
            .then(|| source.session_expires_at.clone())
            .flatten();
        self.sources
            .update_session(
                source_id,
                source.access_token.as_deref(),
                Some(merged_cookies.as_str()),
                // A 401/403 marks the previous protocol session expired. A
                // successful browser challenge supersedes that marker; keep
                // it would make request builders silently omit these cookies.
                session_expires_at.as_deref(),
            )
            .await?;
        Ok(crate::service::source_service::SourceLoginResult {
            source_id,
            authenticated: true,
            has_token: source.access_token.is_some(),
            has_cookie: true,
            session_expires_at,
        })
    }
}

pub(super) fn session_expiry(response: &str, token: Option<&str>) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(response) {
        if let Some(expiry) = value.get("expires_at").and_then(expiry_value) {
            return Some(expiry);
        }
        if let Some(seconds) = value.get("expires_in").and_then(|v| v.as_u64()) {
            return Some((now_epoch().saturating_add(seconds)).to_string());
        }
    }
    let token = token?;
    let payload = token.split('.').nth(1)?;
    let bytes = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    value.get("exp").and_then(expiry_value)
}

fn expiry_value(value: &serde_json::Value) -> Option<String> {
    value
        .as_u64()
        .map(|value| value.to_string())
        .or_else(|| value.as_str().map(str::to_owned))
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default()
}

pub(super) fn cookie_max_age(cookie: &str) -> Option<u64> {
    cookie.split(';').find_map(|attribute| {
        let (name, value) = attribute.trim().split_once('=')?;
        name.eq_ignore_ascii_case("max-age")
            .then(|| value.trim().parse::<i64>().ok())
            .flatten()
            .map(|seconds| seconds.max(0) as u64)
    })
}

/// Whether a cookie set proves the user is still stuck on the challenge page.
///
/// Cloudflare hands `__cf_bm` (and the `cf_chl_*` scratch cookies) to every
/// visitor, including one who has only just loaded "Just a moment…". Solving
/// the challenge is what mints `cf_clearance`. Accepting the former as proof
/// is what made the source report "已认证" while every later request still got
/// bounced — the failure this whole flow exists to fix, reported as a success.
///
/// Sources with no Cloudflare marker at all are ordinary logins and pass
/// through untouched.
fn cloudflare_challenge_unsolved(cookies: &str) -> bool {
    let mut saw_marker = false;
    let mut has_clearance = false;
    for (name, value) in cookies
        .split(';')
        .filter_map(|pair| pair.trim().split_once('='))
    {
        let name = name.trim();
        if name.eq_ignore_ascii_case("cf_clearance") {
            // An empty or deleted clearance does not prove verification.
            saw_marker = true;
            has_clearance = !value.trim().is_empty();
        }
        saw_marker |= name.eq_ignore_ascii_case("__cf_bm")
            || name.to_ascii_lowercase().starts_with("cf_chl")
            || name.to_ascii_lowercase().starts_with("_cf_chl");
    }
    saw_marker && !has_clearance
}

pub(super) fn merge_cookies(values: Option<&str>, extra: Option<&str>) -> Option<String> {
    let mut merged = std::collections::BTreeMap::new();
    for raw in values.into_iter().chain(extra) {
        for pair in raw.split(';') {
            let Some((name, value)) = pair.trim().split_once('=') else {
                continue;
            };
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            merged.insert(name.to_owned(), value.trim().to_owned());
        }
    }
    (!merged.is_empty()).then(|| {
        merged
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

#[cfg(test)]
mod tests {
    use super::{cloudflare_challenge_unsolved, cookie_max_age, merge_cookies, session_expiry};

    #[test]
    fn clearance_beside_the_challenge_markers_is_accepted() {
        assert!(!cloudflare_challenge_unsolved(
            "__cf_bm=abc; cf_clearance=ok"
        ));
    }

    #[test]
    fn empty_clearance_is_not_treated_as_verified() {
        assert!(cloudflare_challenge_unsolved("__cf_bm=abc; cf_clearance="));
        assert!(cloudflare_challenge_unsolved("cf_clearance="));
    }

    /// Sources behind an ordinary login have no Cloudflare cookies at all and
    /// must not be caught by the gate.
    #[test]
    fn non_cloudflare_cookies_pass_through() {
        assert!(!cloudflare_challenge_unsolved("PHPSESSID=abc; remember=1"));
    }

    #[test]
    fn extracts_expiry_from_jwt_payload() {
        let token = "eyJhbGciOiJub25lIn0.eyJleHAiOjQyMDB9.signature";
        assert_eq!(session_expiry("{}", Some(token)).as_deref(), Some("4200"));
    }

    #[test]
    fn extracts_cookie_max_age_for_session_expiry() {
        assert_eq!(cookie_max_age("sid=1; Max-Age=3600; HttpOnly"), Some(3600));
        assert_eq!(cookie_max_age("sid=1; max-age=0"), Some(0));
        assert_eq!(cookie_max_age("sid=1; Path=/"), None);
    }

    #[test]
    fn merges_browser_cookies_without_dropping_existing_login_state() {
        assert_eq!(
            merge_cookies(
                Some("sid=old; theme=dark"),
                Some("cf_clearance=ok; sid=new")
            )
            .as_deref(),
            Some("cf_clearance=ok; sid=new; theme=dark")
        );
    }

    #[test]
    fn ignores_malformed_cookie_fragments() {
        assert_eq!(
            merge_cookies(Some("invalid; sid=1"), Some(" ")).as_deref(),
            Some("sid=1")
        );
    }
}
