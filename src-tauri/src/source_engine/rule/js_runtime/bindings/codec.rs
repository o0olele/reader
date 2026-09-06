use super::super::js_error;
use crate::error::AppError;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rquickjs::{Ctx, Function, Object};

pub(super) fn install<'js>(ctx: Ctx<'js>, java: &Object<'js>) -> Result<(), AppError> {
    java.set(
        "base64Encode",
        Function::new(ctx.clone(), |value: String| STANDARD.encode(value)),
    )
    .map_err(js_error)?;
    java.set(
        "base64Decode",
        Function::new(ctx.clone(), |value: String| {
            STANDARD
                .decode(value)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "hexEncodeToString",
        Function::new(ctx.clone(), |value: String| {
            value
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "hexDecodeToString",
        Function::new(ctx.clone(), |value: String| {
            let bytes = value
                .as_bytes()
                .chunks(2)
                .filter_map(|chunk| {
                    (chunk.len() == 2)
                        .then(|| std::str::from_utf8(chunk).ok())
                        .flatten()
                        .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                })
                .collect::<Vec<_>>();
            String::from_utf8(bytes).unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "strToBytes",
        Function::new(ctx.clone(), |value: String| {
            value
                .as_bytes()
                .iter()
                .map(|byte| byte.to_string())
                .collect::<Vec<_>>()
                .join(",")
        }),
    )
    .map_err(js_error)?;
    java.set(
        "bytesToStr",
        Function::new(ctx.clone(), |value: String| {
            let bytes = value
                .split([',', ' ', '\n'])
                .filter(|part| !part.is_empty())
                .filter_map(|part| part.parse::<u8>().ok())
                .collect::<Vec<_>>();
            String::from_utf8(bytes).unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "encodeURI",
        Function::new(ctx.clone(), |value: String| {
            urlencoding::encode(&value).into_owned()
        }),
    )
    .map_err(js_error)?;
    // Common legado aliases used by older source scripts. Keep coercion
    // permissive, matching Rhino's Java helper behavior instead of raising
    // a QuickJS type error for missing or non-string values.
    java.set(
        "decodeURI",
        Function::new(ctx.clone(), |value: String| {
            urlencoding::decode(&value)
                .map(|decoded| decoded.into_owned())
                .unwrap_or(value)
        }),
    )
    .map_err(js_error)?;
    java.set(
        "toInt",
        Function::new(ctx.clone(), |value: String| {
            value.trim().parse::<i64>().unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "toBoolean",
        Function::new(ctx.clone(), |value: String| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "true" | "1" | "yes"
            )
        }),
    )
    .map_err(js_error)?;
    java.set(
        "isNull",
        Function::new(ctx.clone(), |value: Option<String>| value.is_none()),
    )
    .map_err(js_error)?;
    Ok(())
}
