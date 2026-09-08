//! Browser-bound requests used for Cloudflare clearance and auth retries.

use crate::{
    domain::source::BookSource, error::AppError, source_engine::url::RequestSpec,
};
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::WebviewWindow;

use super::SearchService;

static NEXT_BROWSER_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

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

impl SearchService {
    pub(super) async fn sync_browser_cookies(
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

#[cfg(test)]
mod tests {
    use super::browser_body_looks_like_challenge;

    #[test]
    fn recognizes_http_200_cloudflare_interstitial_bodies() {
        assert!(browser_body_looks_like_challenge(
            "<html><title>Just a moment...</title><script>window._cf_chl_opt={}</script></html>"
        ));
        assert!(!browser_body_looks_like_challenge(
            "<html><div class='newbox'><li><h3>Book</h3></li></div></html>"
        ));
    }
}
