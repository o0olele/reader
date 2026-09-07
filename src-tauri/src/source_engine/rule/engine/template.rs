//! Inline `{{ expression }}` template rendering for the rule engine.

use super::evaluate;
use crate::source_engine::rule::{Extraction, JsContext, JsValue, QuickJsRuntime, RuleContext, RuleExecutionError};

pub(super) fn render_inline_template(
    raw: &str,
    input: &str,
    context: &mut RuleContext,
) -> Result<String, RuleExecutionError> {
    let mut output = String::with_capacity(raw.len());
    let mut cursor = 0;
    while let Some(relative_start) = raw[cursor..].find("{{") {
        let start = cursor + relative_start;
        output.push_str(&raw[cursor..start]);
        let expression_start = start + 2;
        let Some(relative_end) = raw[expression_start..].find("}}") else {
            // Legado treats an unterminated template as literal text. Preserve
            // the remaining input instead of rejecting the whole source.
            output.push_str(&raw[start..]);
            return Ok(output);
        };
        let end = expression_start + relative_end;
        let expression = raw[expression_start..end].trim();
        output.push_str(&evaluate_inline_expression(expression, input, context)?);
        cursor = end + 2;
    }
    output.push_str(&raw[cursor..]);
    Ok(output)
}

fn evaluate_inline_expression(
    expression: &str,
    input: &str,
    context: &mut RuleContext,
) -> Result<String, RuleExecutionError> {
    if let Some(value) = context.get(expression) {
        return Ok(value.to_owned());
    }
    if let Some(rule) = expression.strip_prefix("@@") {
        return Ok(evaluate(rule, input, Extraction::Values, context)?
            .into_iter()
            .next()
            .unwrap_or_default());
    }
    if expression.starts_with("@Json:")
        || expression.starts_with("@json:")
        || expression.starts_with("$.")
        || expression.starts_with("$[")
    {
        return Ok(evaluate(expression, input, Extraction::Values, context)?
            .into_iter()
            .next()
            .unwrap_or_default());
    }
    let runtime = QuickJsRuntime::default();
    let (value, variables) = runtime
        .execute_blocking_with_context(
            expression,
            JsContext {
                result: input.to_owned(),
                base_url: context.http.as_ref().map(|http| http.base_url.clone()),
                variables: context.snapshot(),
                http: context.http.clone(),
                ..Default::default()
            },
        )
        .map_err(|error| RuleExecutionError::UnsupportedJsoup(error.to_string()))?;
    context.extend(variables);
    Ok(match value {
        JsValue::String(value) => value,
        JsValue::Number(value) => value.to_string(),
        JsValue::Boolean(value) => value.to_string(),
        JsValue::Null => String::new(),
        JsValue::Json(value) => value.to_string(),
    })
}
