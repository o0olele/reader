use super::{request_options::JsHttpRequestOptions, JsHttpContext};
use crate::{
    error::AppError,
    infrastructure::http::request::{evaluate_sign_script, user_agent},
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

pub(super) struct JsHttpSession {
    pub(super) client: reqwest::blocking::Client,
    pub(super) context: JsHttpContext,
    pub(super) response: Arc<Mutex<Option<JsHttpResponse>>>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct JsHttpResponse {
    pub(super) status: u16,
    pub(super) headers: HashMap<String, String>,
}

pub(super) fn blocking_http_request_with_options(
    session: &JsHttpSession,
    raw_url: &str,
    options: JsHttpRequestOptions,
) -> Result<String, AppError> {
    let context = &session.context;
    let url = reqwest::Url::parse(raw_url)
        .or_else(|_| reqwest::Url::parse(&context.base_url).and_then(|base| base.join(raw_url)))
        .map_err(AppError::network)?;
    let method = reqwest::Method::from_bytes(options.method.as_bytes())
        .map_err(|error| AppError::InvalidArgument(format!("HTTP method 无效: {error}")))?;
    // Set per request rather than on the cached client: the client is a
    // `OnceLock` and would otherwise freeze whatever UA was configured at the
    // first `java.ajax` call. Source-level and call-level headers below can
    // still override it.
    let mut request = session
        .client
        .request(method, url.clone())
        .header(reqwest::header::USER_AGENT, user_agent());
    if let Some(timeout_ms) = options.timeout_ms.filter(|value| *value > 0) {
        request = request.timeout(Duration::from_millis(timeout_ms.min(120_000)));
    }
    if !context.session_expired {
        if let Some(token) = context.access_token.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(cookie) = context.session_cookie.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::COOKIE, cookie);
        }
    } else {
        tracing::debug!(target: "network", "JS source session expired; omitting credentials");
    }
    if let Some(script) = context.sign_script.as_deref() {
        if let Some(signature) = evaluate_sign_script(script, url.as_str()) {
            request = request.header("x-signature", signature);
        }
    }
    if let Some(raw) = context.headers.as_deref() {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, JsonValue>>(raw) {
            for (name, value) in headers {
                request = request.header(
                    name,
                    value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string()),
                );
            }
        }
    }
    for (name, value) in options.headers {
        request = request.header(name, value);
    }
    if let Some(body) = options.body {
        request = request.body(body);
    }
    let response = request.send().map_err(AppError::network)?;
    let status = response.status();
    let headers = response
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            Some((
                name.as_str().to_ascii_lowercase(),
                value.to_str().ok()?.to_owned(),
            ))
        })
        .collect::<HashMap<_, _>>();
    if let Ok(mut previous) = session.response.lock() {
        *previous = Some(JsHttpResponse {
            status: status.as_u16(),
            headers,
        });
    }
    let text = response.text().map_err(AppError::network)?;
    if !status.is_success() {
        return Err(AppError::Network(format!("HTTP {status}: {text}")));
    }
    Ok(text)
}

pub(super) fn build_js_http_session(context: JsHttpContext) -> Result<JsHttpSession, AppError> {
    static CLIENT: OnceLock<Result<reqwest::blocking::Client, String>> = OnceLock::new();
    let client = CLIENT
        .get_or_init(|| {
            reqwest::blocking::Client::builder()
                .cookie_store(true)
                .timeout(Duration::from_secs(15))
                .build()
                .map_err(|error| error.to_string())
        })
        .as_ref()
        .map_err(|error| AppError::Network(error.clone()))?
        .clone();
    Ok(JsHttpSession {
        client,
        context,
        response: Arc::new(Mutex::new(None)),
    })
}
