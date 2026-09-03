use crate::{
    error::AppError,
    source_engine::rule::{JsContext, JsValue, QuickJsRuntime},
};

pub(super) fn parse_options(
    raw: &str,
) -> Result<serde_json::Map<String, serde_json::Value>, AppError> {
    let raw = raw.trim();
    if let Ok(options) = serde_json::from_str(raw) {
        return Ok(options);
    }
    if let Ok(JsValue::String(json)) = QuickJsRuntime::default()
        .execute_blocking(&format!("JSON.stringify(({raw}))"), JsContext::default())
    {
        return serde_json::from_str(&json)
            .map_err(|error| AppError::Parse(format!("请求选项无效: {error}")));
    }
    let normalized = raw.replace('\'', "\"").replace("undefined", "null");
    serde_json::from_str(&normalized)
        .map_err(|error| AppError::Parse(format!("请求选项无效: {error}")))
}

pub(super) fn parse_headers(value: &serde_json::Value) -> Result<Vec<(String, String)>, AppError> {
    let object = if let Some(object) = value.as_object() {
        object.clone()
    } else if let Some(raw) = value.as_str() {
        parse_options(raw)
            .map_err(|error| AppError::Parse(format!("请求 headers 无效: {error}")))?
    } else {
        return Err(AppError::Parse("请求 headers 必须是对象".into()));
    };
    Ok(object
        .into_iter()
        .map(|(key, value)| {
            (
                key,
                value_string(&value).unwrap_or_else(|| value.to_string()),
            )
        })
        .collect())
}

pub(super) fn value_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::String(value) => Some(value.clone()),
        other => Some(other.to_string()),
    }
}

pub(super) fn value_usize(value: &serde_json::Value) -> Option<usize> {
    value
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .or_else(|| value.as_str()?.parse().ok())
}
