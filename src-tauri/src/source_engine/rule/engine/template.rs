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

fn evaluate_inline_expression(    expression: &str,
    input: &str,
    context: &mut RuleContext,
) -> Result<String, RuleExecutionError> {
    if let Some(value) = context.get(expression) {
        return Ok(value.to_owned());
    }
    if let Some(rule) = expression.strip_prefix("@@") {
        return Ok(inline_rule_value(rule, input, context));
    }
    // Legado's `{{@rule}}` evaluates a rule against the current input, exactly
    // like `{{@@rule}}` but with the analyzer's own mode detection (so
    // `{{@css:…}}`, `{{@XPath:…}}` and `{{@Json:…}}` all work). Corpus sources
    // use it heavily for display templates such as
    // `🔖 {{@class.tagList.0@text}}`; without this branch the rule text was
    // handed to the JavaScript runtime, which failed on the first `@`.
    if let Some(rule) = expression.strip_prefix('@') {
        return Ok(inline_rule_value(rule, input, context));
    }
    if expression.starts_with("$.") || expression.starts_with("$[") {
        return Ok(inline_rule_value(expression, input, context));
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

/// Evaluates a nested rule referenced from inside a `{{ … }}` template.
fn inline_rule_value(rule: &str, input: &str, context: &mut RuleContext) -> String {
    evaluate(rule, input, Extraction::Values, context)
        .map(|values| values.into_iter().next().unwrap_or_default())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML: &str = r#"<div class="tagList">标签一</div><div class="intro">简介</div>"#;

    fn render(template: &str, input: &str) -> String {
        render_inline_template(template, input, &mut RuleContext::default()).unwrap()
    }

    #[test]
    fn renders_rule_templates_against_the_current_input() {
        assert_eq!(render("🔖 {{@class.tagList.0@text}}", HTML), "🔖 标签一");
        assert_eq!(render("{{@.intro@text}}", HTML), "简介");
        assert_eq!(render("{{@@class.intro@text}}", HTML), "简介");
    }

    #[test]
    fn traces_template_rule_forms() {
        let html = r#"<div class="intro">简介</div>"#;
        for rule in ["@css:.intro@text", "@css:div.intro@text", "@.intro@text", "@class.intro@text"] {
            println!(
                "RULE {rule} -> bare {:?} | bracketed {:?}",
                render(rule, html),
                render(&format!("{{{{{rule}}}}}"), html),
            );
        }
    }

    #[test]
    fn renders_jsonpath_templates_against_json_input() {
        assert_eq!(render("{{$.title}}", r#"{"title":"书"}"#), "书");
    }

    #[test]
    fn keeps_unterminated_templates_literal() {
        assert_eq!(render("prefix {{unterminated", HTML), "prefix {{unterminated");
    }
}
