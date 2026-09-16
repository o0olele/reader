//! Source-specific headers and signature evaluation.
use crate::{
    domain::source::BookSource,
    error::AppError,
    source_engine::rule::{JsContext, JsValue, QuickJsRuntime},
};
use sha2::{Digest, Sha256};

pub(super) fn add_custom_header(
    request: reqwest::RequestBuilder,
    name: &str,
    value: &str,
) -> reqwest::RequestBuilder {
    let cleaned_name = trim_header_token(name);
    let Ok(name) = reqwest::header::HeaderName::from_bytes(cleaned_name.as_bytes()) else {
        tracing::warn!(target: "network", header = %name, "ignoring invalid source header name");
        return request;
    };
    let cleaned_value = trim_header_token(value);
    let Ok(value) = reqwest::header::HeaderValue::from_str(&cleaned_value) else {
        tracing::warn!(target: "network", header = %name, "ignoring invalid source header value");
        return request;
    };
    request.header(name, value)
}

pub(super) fn resolve_source_headers(
    raw: &str,
    source: &BookSource,
    url: &str,
) -> Result<Vec<(String, String)>, AppError> {
    let raw = raw.trim();
    let evaluated = if let Some(script) = raw.strip_prefix("@js:") {
        Some(QuickJsRuntime::default().execute_blocking(
            script,
            JsContext {
                url: Some(url.to_owned()),
                base_url: Some(source.base_url.clone()),
                http: Some(source.http_context()),
                ..Default::default()
            },
        )?)
    } else if raw.starts_with('{') && serde_json::from_str::<serde_json::Value>(raw).is_err() {
        QuickJsRuntime::default()
            .execute_blocking(raw, JsContext::default())
            .ok()
    } else {
        None
    };
    if let Some(value) = evaluated {
        return Ok(match value {
            JsValue::Json(value) => header_pairs_from_value(value),
            JsValue::String(value) => parse_header_text(&value),
            JsValue::Null => Vec::new(),
            other => parse_header_text(&js_header_string(other)),
        });
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        return Ok(header_pairs_from_value(value));
    }
    Ok(parse_header_text(raw))
}

fn header_pairs_from_value(value: serde_json::Value) -> Vec<(String, String)> {
    value
        .as_object()
        .into_iter()
        .flat_map(|object| object.iter())
        .map(|(name, value)| {
            (
                name.clone(),
                value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| value.to_string()),
            )
        })
        .collect()
}

fn parse_header_text(raw: &str) -> Vec<(String, String)> {
    raw.trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(&['\n', '&', ','][..])
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            let name = trim_header_token(name);
            (!name.is_empty()).then(|| (name, trim_header_token(value)))
        })
        .collect()
}

fn trim_header_token(value: &str) -> String {
    value
        .trim()
        .trim_matches(|character| matches!(character, '\'' | '"' | '`' | ','))
        .trim()
        .to_owned()
}

fn js_header_string(value: JsValue) -> String {
    match value {
        JsValue::String(value) => value,
        JsValue::Number(value) => value.to_string(),
        JsValue::Boolean(value) => value.to_string(),
        JsValue::Null => String::new(),
        JsValue::Json(value) => value.to_string(),
    }
}

pub fn evaluate_sign_script(script: &str, url: &str) -> Option<String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();
    let expression = script
        .replace("{{url}}", url)
        .replace("{{timestamp}}", &timestamp);
    let inner = expression
        .trim()
        .trim_start_matches("return")
        .trim()
        .trim_end_matches(';')
        .trim()
        .strip_prefix("sha256(")?
        .strip_suffix(')')?;
    let mut hasher = Sha256::new();
    hasher.update(inner.as_bytes());
    Some(format!("{:x}", hasher.finalize()))
}
