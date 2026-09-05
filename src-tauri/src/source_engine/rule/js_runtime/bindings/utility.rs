use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use super::super::js_error;
use super::super::{chapter_numbers::normalize_chapter_numbers, time::format_epoch};

pub(super) fn install<'js>(ctx: Ctx<'js>, java: &Object<'js>) -> Result<(), AppError> {
    java.set(
        "timeFormatRaw",
        Function::new(ctx.clone(), |epoch: f64, pattern: String| {
            format_epoch(epoch as i64, &pattern)
        }),
    )
    .map_err(js_error)?;
    java.set(
        "timeFormatUtcRaw",
        Function::new(
            ctx.clone(),
            |epoch: f64, pattern: String, offset_hours: i32| {
                let offset_ms = i64::from(offset_hours) * 3_600 * 1_000;
                format_epoch(epoch as i64 + offset_ms, &pattern)
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "toNumChapter",
        Function::new(ctx.clone(), |value: String| {
            normalize_chapter_numbers(&value)
        }),
    )
    .map_err(js_error)?;
    java.set("toast", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("longToast", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("t2s", Function::new(ctx.clone(), |value: String| value))
        .map_err(js_error)?;
    java.set(
        "htmlFormat",
        Function::new(ctx.clone(), |value: String| value),
    )
    .map_err(js_error)?;
    java.set("startBrowser", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("startBrowserAwait", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("openUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("setContent", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshBookUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshTocUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshExplore", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set(
        "log",
        Function::new(ctx.clone(), |value: String| {
            tracing::debug!(target: "source", "JS: {value}");
        }),
    )
    .map_err(js_error)?;
    // Legado overloads these helpers (one, two, or three arguments). Native
    // QuickJS functions are fixed-arity, so expose small JS shims that coerce
    // omitted/string arguments before calling the typed bridge.
    ctx.eval::<(), _>(
        r#"
        java.timeFormat = function(epoch, pattern) {
            epoch = Number(epoch);
            if (!isFinite(epoch)) epoch = 0;
            pattern = pattern == null ? 'yyyy-MM-dd HH:mm:ss' : String(pattern);
            return java.timeFormatRaw(epoch, pattern);
        };
        java.timeFormatUTC = function(epoch, pattern, offset) {
            epoch = Number(epoch);
            if (!isFinite(epoch)) epoch = 0;
            pattern = pattern == null ? 'yyyy-MM-dd HH:mm:ss' : String(pattern);
            offset = offset == null ? 0 : Number(offset);
            if (!isFinite(offset)) offset = 0;
            return java.timeFormatUtcRaw(epoch, pattern, offset);
        };
        "#,
    )
    .map_err(js_error)?;
    Ok(())
}
