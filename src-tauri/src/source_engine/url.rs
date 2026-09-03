//! Shared Legado-style URL/request option parsing.
//!
//! This module is deliberately transport-agnostic: it turns a source URL (including
//! `,{'method':'POST',...}` or `<js>...</js>`) into a normalized request spec.
use crate::{domain::source::BookSource, error::AppError, infrastructure::http::url::resolve_url};

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub url: reqwest::Url,
    pub method: reqwest::Method,
    pub body: Option<String>,
    pub charset: Option<String>,
    pub headers: Vec<(String, String)>,
    pub origin: Option<String>,
    pub retry: usize,
    pub response_type: Option<String>,
}

pub fn build(source: &BookSource, raw: &str, keyword: Option<&str>, label: &str) -> Result<RequestSpec, AppError> {
    let expanded = expand(raw, keyword.unwrap_or(""));
    let trimmed = expanded.trim();
    let (target, options_text) = if let Some(script) = trimmed.strip_prefix("<js>").and_then(|v| v.strip_suffix("</js>")) {
        split_target_options(script.trim(), label)?
    } else {
        split_target_options(trimmed, label)?
    };
    let options = options_text.map(parse_options).transpose()?.unwrap_or_default();
    let method = options.get("method").and_then(value_string).unwrap_or_else(|| "GET".into())
        .parse().map_err(|e| AppError::InvalidArgument(format!("{label} HTTP method 无效: {e}")))?;
    let body = options.get("body").and_then(value_string).map(|v| expand(&v, keyword.unwrap_or("")));
    let charset = options.get("charset").and_then(value_string).map(|v| v.to_ascii_lowercase());
    let headers = options.get("headers").map(parse_headers).transpose()?.unwrap_or_default();
    let origin = options.get("origin").and_then(value_string);
    let retry = options.get("retry").and_then(|v| v.as_u64()).unwrap_or(0).min(10) as usize;
    let response_type = options.get("type").and_then(value_string);
    Ok(RequestSpec { url: resolve_url(&source.base_url, target.trim(), label)?, method, body, charset, headers, origin, retry, response_type })
}

fn split_target_options(value: &str, label: &str) -> Result<(&str, Option<&str>), AppError> {
    let mut quote = None;
    let mut depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '\'' | '"' if quote == Some(ch) => quote = None,
            '\'' | '"' if quote.is_none() => quote = Some(ch),
            '{' | '[' if quote.is_none() => depth += 1,
            '}' | ']' if quote.is_none() => depth = depth.saturating_sub(1),
            ',' if quote.is_none() && depth == 0 => return Ok((&value[..index], Some(value[index + 1..].trim()))),
            _ => {}
        }
    }
    if value.starts_with("<js>") { return Err(AppError::Parse(format!("{label} JS 缺少请求选项"))); }
    Ok((value, None))
}

fn parse_options(raw: &str) -> Result<serde_json::Map<String, serde_json::Value>, AppError> {
    let raw = raw.split_once(';').map(|(v, _)| v).unwrap_or(raw).trim();
    let normalized = raw.replace('\'', "\"").replace("undefined", "null");
    serde_json::from_str(&normalized).map_err(|e| AppError::Parse(format!("请求选项无效: {e}")))
}

fn parse_headers(value: &serde_json::Value) -> Result<Vec<(String, String)>, AppError> {
    let Some(object) = value.as_object() else { return Err(AppError::Parse("请求 headers 必须是对象".into())); };
    Ok(object.iter().map(|(k, v)| (k.clone(), value_string(v).unwrap_or_else(|| v.to_string()))).collect())
}

fn value_string(value: &serde_json::Value) -> Option<String> { value.as_str().map(str::to_owned) }
fn expand(value: &str, keyword: &str) -> String {
    value.replace("{{key}}", keyword).replace("{key}", keyword).replace("<key>", keyword).replace("<searchKey>", keyword)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{BookSource, CatalogRule, InfoRule, SearchRule};
    fn source() -> BookSource { BookSource { id: 1, name: "t".into(), base_url: "https://example.com/".into(), search_url: "".into(), search_rule: SearchRule { item: "a".into(), title: "a".into(), author: None, cover: None, url: "a".into() }, info_rule: InfoRule::default(), catalog_rule: CatalogRule { item: "a".into(), title: "a".into(), url: "a".into(), next_url: None }, content_selector: "body".into(), next_toc_url_selector: None, next_content_url_selector: None, header: None, login_url: None, login_method: "GET".into(), login_body: None, token_path: None, access_token: None, session_cookie: None, session_expires_at: None, sign_script: None, proxy_url: None, enabled: true, raw_rules: Default::default() } }
    #[test] fn parses_options_and_expands_body() { let spec = build(&source(), "path,{'method':'POST','body':'q={{key}}','retry':2,'headers':{'X-A':'b'}}", Some("%E4%B8%AD"), "test").unwrap(); assert_eq!(spec.method, reqwest::Method::POST); assert_eq!(spec.body.as_deref(), Some("q=%E4%B8%AD")); assert_eq!(spec.retry, 2); assert_eq!(spec.headers[0], ("X-A".into(), "b".into())); }
    #[test] fn parses_js_wrapper() { let spec = build(&source(), "<js>/api,{'charset':'gbk'};result;</js>", None, "test").unwrap(); assert_eq!(spec.charset.as_deref(), Some("gbk")); }
}
