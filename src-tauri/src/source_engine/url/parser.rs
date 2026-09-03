use super::{
    encoding::{encode_body, encode_query},
    options::{parse_headers, parse_options, value_string, value_usize},
    RequestSpec,
};
use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::url::resolve_url,
    source_engine::rule::{JsContext, JsValue, QuickJsRuntime},
};

pub fn build(
    source: &BookSource,
    raw: &str,
    keyword: Option<&str>,
    label: &str,
) -> Result<RequestSpec, AppError> {
    build_with_base(source, &source.base_url, raw, keyword, label)
}

/// Resolves the URL portion while preserving a trailing Legado option object.
pub fn absolutize(base_url: &str, raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.starts_with("<js>") || trimmed.starts_with("@js:") {
        return trimmed.to_owned();
    }
    let (target, options) = split_target_options(trimmed);
    let resolved = resolve_url(base_url, target.trim(), "规则 URL")
        .map(|url| url.to_string())
        .unwrap_or_else(|_| target.trim().to_owned());
    match options {
        Some(options) => format!("{resolved},{options}"),
        None => resolved,
    }
}

pub fn build_with_base(
    source: &BookSource,
    base_url: &str,
    raw: &str,
    keyword: Option<&str>,
    label: &str,
) -> Result<RequestSpec, AppError> {
    let key = keyword.unwrap_or_default();
    let expanded = replace_templates(source, base_url, raw, key)?;
    let analyzed = evaluate_url_js(source, base_url, &expanded, key)?;
    let (target, options_text) = split_target_options(analyzed.trim());
    let options = options_text
        .map(parse_options)
        .transpose()?
        .unwrap_or_default();
    let method = options
        .get("method")
        .and_then(value_string)
        .unwrap_or_else(|| "GET".into())
        .to_ascii_uppercase()
        .parse()
        .map_err(|error| AppError::InvalidArgument(format!("{label} HTTP method 无效: {error}")))?;
    let charset = options
        .get("charset")
        .and_then(value_string)
        .map(|value| value.to_ascii_lowercase());
    let headers = options
        .get("headers")
        .map(parse_headers)
        .transpose()?
        .unwrap_or_default();
    let body = options
        .get("body")
        .and_then(value_string)
        .map(|value| replace_templates(source, base_url, &value, key))
        .transpose()?
        .map(|value| encode_body(value, &method, charset.as_deref(), &headers))
        .transpose()?;
    let origin = options.get("origin").and_then(value_string);
    let retry = options
        .get("retry")
        .and_then(value_usize)
        .unwrap_or(0)
        .min(10);
    let response_type = options.get("type").and_then(value_string);
    let body_js = options.get("bodyJs").and_then(value_string);
    let mut target = absolute_unencoded(base_url, target.trim(), label)?;
    if let Some(script) = options.get("js").and_then(value_string) {
        target = eval_js(source, base_url, key, &target, Some(&target), &script)?;
    }
    let target = encode_query(target.trim(), &method, charset.as_deref())?;
    let url = resolve_url(base_url, &target, label)?;
    Ok(RequestSpec {
        url,
        method,
        body,
        charset,
        headers,
        origin,
        retry,
        response_type,
        body_js,
    })
}

fn replace_templates(
    source: &BookSource,
    base_url: &str,
    raw: &str,
    key: &str,
) -> Result<String, AppError> {
    let mut output = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(start) = rest.find("{{") {
        output.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            return Err(AppError::Parse("URL 模板缺少 `}}`".into()));
        };
        let script = after[..end].trim();
        output.push_str(&eval_js(source, base_url, key, raw, None, script)?);
        rest = &after[end + 2..];
    }
    output.push_str(rest);
    Ok(output
        .replace("{key}", key)
        .replace("<key>", key)
        .replace("<searchKey>", key))
}

fn evaluate_url_js(
    source: &BookSource,
    base_url: &str,
    raw: &str,
    key: &str,
) -> Result<String, AppError> {
    let trimmed = raw.trim();
    if let Some(script) = trimmed.strip_prefix("@js:") {
        return eval_js(source, base_url, key, raw, None, script);
    }
    if let Some(inner) = trimmed
        .strip_prefix("<js>")
        .and_then(|value| value.strip_suffix("</js>"))
    {
        // Keep compatibility with an existing legado spelling whose first
        // statement is the request specification and whose JS tail only
        // mutates `result`.
        if let Some((request, _)) = inner.split_once(';') {
            let (target, options) = split_target_options(request.trim());
            if options.is_some()
                && (target.starts_with('/')
                    || target.starts_with("http://")
                    || target.starts_with("https://")
                    || target.starts_with("data:"))
            {
                return Ok(request.trim().to_owned());
            }
        }
        return eval_js(source, base_url, key, raw, None, inner);
    }
    let mut result = raw.to_owned();
    let mut rest = raw;
    while let Some(start) = rest.find("<js>") {
        let prefix = rest[..start].trim();
        if !prefix.is_empty() {
            result = prefix.replace("@result", &result);
        }
        let after = &rest[start + 4..];
        let Some(end) = after.find("</js>") else {
            return Err(AppError::Parse("URL JavaScript 缺少 `</js>`".into()));
        };
        result = eval_js(source, base_url, key, &result, None, &after[..end])?;
        rest = &after[end + 5..];
    }
    let tail = rest.trim();
    if !tail.is_empty() && tail != raw.trim() {
        result = tail.replace("@result", &result);
    }
    Ok(result)
}

fn eval_js(
    source: &BookSource,
    base_url: &str,
    key: &str,
    result: &str,
    url: Option<&str>,
    script: &str,
) -> Result<String, AppError> {
    let value = QuickJsRuntime::default().execute_blocking(
        script,
        JsContext {
            result: result.to_owned(),
            url: url.map(str::to_owned),
            key: Some(key.to_owned()),
            base_url: Some(base_url.to_owned()),
            http: Some(source.http_context()),
            ..Default::default()
        },
    )?;
    Ok(js_string(value))
}

fn js_string(value: JsValue) -> String {
    match value {
        JsValue::String(value) => value,
        JsValue::Number(value) if value.fract() == 0.0 => format!("{value:.0}"),
        JsValue::Number(value) => value.to_string(),
        JsValue::Boolean(value) => value.to_string(),
        JsValue::Null => String::new(),
        JsValue::Json(value) => value.to_string(),
    }
}

fn split_target_options(value: &str) -> (&str, Option<&str>) {
    let mut quote = None;
    for (index, ch) in value.char_indices() {
        match ch {
            '\'' | '"' if quote == Some(ch) => quote = None,
            '\'' | '"' if quote.is_none() => quote = Some(ch),
            ',' if quote.is_none() && value[index + 1..].trim_start().starts_with('{') => {
                return (&value[..index], Some(value[index + 1..].trim()));
            }
            _ => {}
        }
    }
    (value, None)
}

fn absolute_unencoded(base_url: &str, target: &str, label: &str) -> Result<String, AppError> {
    let (path, suffix) = target
        .find(['?', '#'])
        .map_or((target, ""), |index| (&target[..index], &target[index..]));
    let path = resolve_url(base_url, path, label)?;
    let mut resolved = path.to_string();
    if !suffix.is_empty() {
        resolved.push_str(suffix);
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{CatalogRule, InfoRule, SearchRule};

    fn source() -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com/books/".into(),
            search_url: String::new(),
            explore_url: None,
            search_rule: SearchRule {
                item: "a".into(),
                title: "a".into(),
                author: None,
                cover: None,
                url: "a".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "GET".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            raw_rules: Default::default(),
        }
    }

    #[test]
    fn parses_options_and_evaluates_url_javascript() {
        let spec = build(
            &source(),
            "path,{'method':'POST','body':'q={{key}}','retry':'2','headers':{'X-A':'b'},'js':'result + \"?from=js\"'}",
            Some("novel"),
            "test",
        ).unwrap();
        assert_eq!(spec.url.as_str(), "https://example.com/books/path?from=js");
        assert_eq!(spec.method, reqwest::Method::POST);
        assert_eq!(spec.body.as_deref(), Some("q=novel"));
        assert_eq!(spec.retry, 2);
        assert_eq!(spec.headers, vec![("X-A".into(), "b".into())]);
    }

    #[test]
    fn evaluates_cover_rule_data_url_template() {
        let spec = build(
            &source(),
            r#"data:;base64,{{java.base64Encode(key)}},{"type":"lyc"}"#,
            Some("cover"),
            "封面 URL",
        )
        .unwrap();
        assert_eq!(spec.url.as_str(), "data:;base64,Y292ZXI=");
        assert_eq!(spec.response_type.as_deref(), Some("lyc"));
    }

    #[test]
    fn pagination_uses_the_current_page_as_base() {
        let spec = build_with_base(
            &source(),
            "https://example.com/books/42/chapter/1",
            "../2,{'charset':'gbk'}",
            None,
            "分页 URL",
        )
        .unwrap();
        assert_eq!(spec.url.as_str(), "https://example.com/books/42/2");
        assert_eq!(spec.charset.as_deref(), Some("gbk"));
    }

    #[test]
    fn absolutize_preserves_request_options() {
        assert_eq!(
            absolutize(
                "https://example.com/books/",
                "42,{'method':'POST','body':'id=42'}"
            ),
            "https://example.com/books/42,{'method':'POST','body':'id=42'}"
        );
    }

    #[test]
    fn exposes_raw_key_to_js_and_encodes_the_request_body() {
        let spec = build(
            &source(),
            r#"search,{"method":"POST","body":"q={{key}}&token={{java.base64Encode(key)}}"}"#,
            Some("剑来"),
            "搜索 URL",
        )
        .unwrap();
        assert_eq!(
            spec.body.as_deref(),
            Some("q=%E5%89%91%E6%9D%A5&token=5YmR5p2l")
        );
    }

    #[test]
    fn encodes_query_with_the_requested_charset() {
        let spec = build(
            &source(),
            r#"search?q={{key}},{"charset":"gbk"}"#,
            Some("斗破"),
            "搜索 URL",
        )
        .unwrap();
        assert_eq!(
            spec.url.as_str(),
            "https://example.com/books/search?q=%B6%B7%C6%C6"
        );
    }

    #[test]
    fn evaluates_javascript_segments_in_order() {
        let spec = build(
            &source(),
            "/books/<js>result + key</js>@result?done=1",
            Some("42"),
            "test",
        )
        .unwrap();
        assert_eq!(spec.url.as_str(), "https://example.com/books/42?done=1");
    }
}
