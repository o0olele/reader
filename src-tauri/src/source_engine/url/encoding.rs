use crate::error::AppError;
use encoding_rs::Encoding;

pub(super) fn encode_body(
    body: String,
    method: &reqwest::Method,
    charset: Option<&str>,
    headers: &[(String, String)],
) -> Result<String, AppError> {
    let trimmed = body.trim_start();
    let has_content_type = headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("content-type"));
    if *method != reqwest::Method::POST
        || has_content_type
        || trimmed.starts_with('{')
        || trimmed.starts_with('[')
        || trimmed.starts_with('<')
    {
        return Ok(body);
    }
    encode_fields(&body, charset)
}

pub(super) fn encode_query(
    target: &str,
    method: &reqwest::Method,
    charset: Option<&str>,
) -> Result<String, AppError> {
    if !matches!(*method, reqwest::Method::GET | reqwest::Method::HEAD)
        || charset.is_none_or(|value| matches!(value, "utf-8" | "utf8"))
    {
        return Ok(target.to_owned());
    }
    let Some((path, query)) = target.split_once('?') else {
        return Ok(target.to_owned());
    };
    let (query, fragment) = query
        .split_once('#')
        .map_or((query, None), |(query, fragment)| (query, Some(fragment)));
    let encoded = encode_fields(query, charset)?;
    Ok(match fragment {
        Some(fragment) => format!("{path}?{encoded}#{fragment}"),
        None => format!("{path}?{encoded}"),
    })
}

fn encode_fields(value: &str, charset: Option<&str>) -> Result<String, AppError> {
    let encoding = match charset {
        Some("escape") => None,
        Some(label) => Some(
            Encoding::for_label(label.as_bytes())
                .ok_or_else(|| AppError::Parse(format!("不支持的请求字符集: {label}")))?,
        ),
        None => Some(encoding_rs::UTF_8),
    };
    Ok(value
        .split('&')
        .map(|field| {
            field.split_once('=').map_or_else(
                || encode_component(field, encoding),
                |(key, value)| {
                    format!(
                        "{}={}",
                        encode_component(key, encoding),
                        encode_component(value, encoding)
                    )
                },
            )
        })
        .collect::<Vec<_>>()
        .join("&"))
}

fn encode_component(value: &str, encoding: Option<&'static Encoding>) -> String {
    if value.as_bytes().windows(3).any(|bytes| {
        bytes[0] == b'%' && bytes[1].is_ascii_hexdigit() && bytes[2].is_ascii_hexdigit()
    }) {
        return value.to_owned();
    }
    let bytes = encoding
        .map(|encoding| encoding.encode(value).0.into_owned())
        .unwrap_or_else(|| value.as_bytes().to_vec());
    bytes
        .into_iter()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                (byte as char).to_string()
            } else if byte == b' ' {
                "+".into()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}
