use super::{
    js_error,
    normalize::normalize_js_statement_boundaries,
    statements::{has_top_level_return, split_last_statement},
    JsValue,
};
use crate::error::AppError;
use rquickjs::{context::EvalOptions, CatchResultExt, Ctx};
use serde_json::Value as JsonValue;

pub(super) fn evaluate_script<'js>(ctx: Ctx<'js>, script: &str) -> Result<JsValue, AppError> {
    let script = normalize_js_statement_boundaries(script);
    let mut sloppy = EvalOptions::default();
    sloppy.strict = false;
    // Most legado rules are expressions (`result.trim()`). Try that first;
    // statement blocks use an explicit `return` and are evaluated second.
    let expression = format!("JSON.stringify(({script}))");
    let serialized = match ctx.eval_with_options::<String, _>(expression.as_str(), sloppy) {
        Ok(serialized) => serialized,
        Err(_) => {
            // Expression failure is expected for statement-style rules. A
            // direct eval preserves JavaScript's completion value, including
            // scripts whose statements are separated only by newlines (a
            // common Legado style). The old function wrapper tried to infer
            // the final expression from semicolons and turned scripts such as
            // `headerSign=...\nparamSign=...\nurl` into an invalid single
            // `return (...)` expression.
            let _ = ctx.catch();
            let source = serde_json::to_string(&script)
                .map_err(|error| AppError::Parse(format!("JavaScript 编码失败: {error}")))?;
            let program = format!(
                "JSON.stringify((function() {{ return eval({source}); }})()) || JSON.stringify(result)"
            );
            let mut direct_options = EvalOptions::default();
            direct_options.strict = false;
            let direct = ctx
                .eval_with_options::<String, _>(program.as_str(), direct_options)
                .catch(&ctx);
            match direct {
                Ok(serialized) => serialized,
                Err(_direct_error) if has_top_level_return(&script) => {
                    // A script containing an explicit top-level `return`
                    // cannot run through eval. Keep the function-wrapper
                    // fallback for those exports and report its real error.
                    let block_script = split_last_statement(&script)
                        .map(|(body, tail)| format!("{body}; return ({tail});"))
                        .unwrap_or_else(|| script.to_owned());
                    let block = format!(
                        "JSON.stringify((function() {{ {block_script} }})()) || JSON.stringify(result)"
                    );
                    let mut block_options = EvalOptions::default();
                    block_options.strict = false;
                    ctx.eval_with_options::<String, _>(block.as_str(), block_options)
                        .catch(&ctx)
                        .map_err(js_error)?
                }
                Err(direct_error) => return Err(js_error(direct_error)),
            }
        }
    };
    let value: JsonValue = serde_json::from_str(&serialized)
        .map_err(|error| AppError::Parse(format!("JavaScript 返回值不是 JSON: {error}")))?;
    Ok(match value {
        JsonValue::String(value) => JsValue::String(value),
        JsonValue::Number(value) => JsValue::Number(value.as_f64().unwrap_or_default()),
        JsonValue::Bool(value) => JsValue::Boolean(value),
        JsonValue::Null => JsValue::Null,
        other => JsValue::Json(other),
    })
}
