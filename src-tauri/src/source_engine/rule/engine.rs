//! Orchestration of a parsed rule string.
//!
//! [`super::split_rule`] turns a legado rule into alternatives (`||`) of
//! joined rules (`&&`, `%%`, chaining). This module executes that structure:
//!
//! - alternatives are tried in order, the first non-empty result wins
//! - [`RuleJoin::Chain`] feeds one rule's output into the next
//! - [`RuleJoin::Concat`] (`&&`) evaluates against the same input, appending
//! - [`RuleJoin::Interleave`] (`%%`) does the same but merges alternately
//!
//! `##` replacement and `-` reversal are applied per rule by the evaluators.

use super::evaluator::{execute_js, execute_rule, mode_name};
use super::jsoup::Extraction;
use super::model::{
    RuleAlternatives, RuleContext, RuleExecutionError, RuleJoin, RuleMode, SourceRule,
};
use super::{expand_template, split_rule, JsContext, JsValue, QuickJsRuntime};

/// Parses `raw` and executes it against `input`.
pub fn evaluate(
    raw: &str,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    let alternatives = match split_rule(raw) {
        Ok(value) => value,
        Err(error) => {
            // legado treats malformed/empty branches as a non-matching rule;
            // one broken spelling must not block an entire source.
            match error {
                super::model::RuleParseError::Unclosed(_)
                | super::model::RuleParseError::EmptyBranch(_) => {
                    tracing::debug!(rule = raw, error = %error, "ignoring malformed rule");
                    return Ok(Vec::new());
                }
                other => return Err(other.into()),
            }
        }
    };
    execute_alternatives(&alternatives, input, want, context)
}

/// Convenience wrapper for the common "one string out" case.
pub fn evaluate_first(
    raw: &str,
    input: &str,
    context: &mut RuleContext,
) -> Result<Option<String>, RuleExecutionError> {
    Ok(evaluate(raw, input, Extraction::Values, context)?
        .into_iter()
        .find(|value| !value.trim().is_empty()))
}

pub fn execute_alternatives(
    alternatives: &RuleAlternatives,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    let mut last_error = None;
    for group in alternatives {
        match execute_group(group, input, want, context) {
            Ok(values) if !values.is_empty() => return Ok(values),
            Ok(_) => {}
            // A broken alternative must not sink the ones after it; legado
            // treats `||` as "try the next spelling". The error is only
            // surfaced if every alternative fails.
            Err(error) => last_error = Some(error),
        }
    }
    match last_error {
        Some(error) => Err(error),
        None => Ok(Vec::new()),
    }
}

fn execute_group(
    group: &[SourceRule],
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    let mut chained = vec![input.to_owned()];
    let mut combined: Option<Vec<String>> = None;

    for rule in group {
        match rule.join {
            RuleJoin::Chain => {
                let mut next = Vec::new();
                for value in &chained {
                    next.extend(execute_one(rule, value, want, context)?);
                }
                chained = next;
                combined = Some(chained.clone());
            }
            // `&&` and `%%` re-run against the group's original input rather
            // than the chained output — they are combinators, not pipes.
            RuleJoin::Concat | RuleJoin::Interleave => {
                let values = execute_one(rule, input, want, context)?;
                let previous = combined.take().unwrap_or_default();
                combined = Some(if rule.join == RuleJoin::Concat {
                    [previous, values].concat()
                } else {
                    interleave(previous, values)
                });
                chained = combined.clone().unwrap_or_default();
            }
        }
    }
    Ok(combined.unwrap_or(chained))
}

fn execute_one(
    rule: &SourceRule,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode == RuleMode::WebJs {
        return Err(RuleExecutionError::UnsupportedMode(mode_name(rule.mode)));
    }
    for (key, value) in &rule.put {
        context.insert(key.clone(), value.clone());
    }
    // `@get:{key}` reads a variable instead of querying the document.
    if !rule.get.is_empty() && rule.rule.trim().is_empty() {
        return Ok(rule
            .get
            .iter()
            .filter_map(|key| context.get(key).map(str::to_owned))
            .collect());
    }
    let expanded = expand_template(&rule.rule, context);
    let rendered = if expanded.contains("{{") {
        Some(render_inline_template(&expanded, input, context)?)
    } else {
        None
    };
    if rule.mode == RuleMode::Default {
        if let Some(rendered) = rendered.as_ref() {
            let mut values = vec![rendered.clone()];
            super::evaluator::apply_postprocess(rule, &mut values);
            return Ok(values);
        }
    }
    let effective = if expanded == rule.rule {
        if let Some(rendered) = rendered {
            SourceRule {
                rule: rendered,
                ..rule.clone()
            }
        } else {
            rule.clone()
        }
    } else {
        SourceRule {
            rule: rendered.unwrap_or(expanded),
            ..rule.clone()
        }
    };
    if effective.mode == RuleMode::Js {
        return execute_js(&effective, input, context);
    }
    execute_rule(&effective, input, want)
}

fn render_inline_template(
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
            return Err(RuleExecutionError::UnsupportedJsoup(
                "template is missing `}}`".into(),
            ));
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

fn interleave(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left = left.into_iter();
    let mut right = right.into_iter();
    loop {
        match (left.next(), right.next()) {
            (None, None) => return result,
            (first, second) => result.extend(first.into_iter().chain(second)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = r#"<ul class="odd"><li><a href="/one">第一章</a></li><li><a href="/two">第二章</a></li></ul>"#;

    fn run(raw: &str, input: &str) -> Vec<String> {
        evaluate(raw, input, Extraction::Values, &mut RuleContext::default()).unwrap()
    }

    #[test]
    fn falls_back_to_the_next_alternative_until_one_matches() {
        assert_eq!(run(".missing@text||tag.a.0@text", LIST), vec!["第一章"]);
        assert_eq!(run("tag.a.0@text||tag.a.1@text", LIST), vec!["第一章"]);
    }

    #[test]
    fn concatenates_and_interleaves_against_the_same_input() {
        assert_eq!(
            run("tag.a.0@text&&tag.a.1@text", LIST),
            vec!["第一章", "第二章"]
        );
        assert_eq!(
            run("tag.a@href%%tag.a@text", LIST),
            vec!["/one", "第一章", "/two", "第二章"]
        );
    }

    #[test]
    fn chains_across_modes() {
        let nodes = evaluate(
            "class.odd",
            LIST,
            Extraction::Nodes,
            &mut RuleContext::default(),
        )
        .unwrap();
        assert_eq!(run("@XPath://a/@href", &nodes[0]), vec!["/one", "/two"]);
        // A JS tail is evaluated by the sandboxed runtime.
        assert!(matches!(
            evaluate(
                "tag.a.0@text@js:result",
                LIST,
                Extraction::Values,
                &mut RuleContext::default()
            ),
            Ok(values) if values == vec!["第一章".to_owned()]
        ));
    }

    #[test]
    fn passes_variables_from_put_to_get() {
        let mut context = RuleContext::default();
        evaluate(
            r#"@put:{"bid":"42"}tag.a.0@text"#,
            LIST,
            Extraction::Values,
            &mut context,
        )
        .unwrap();
        assert_eq!(context.get("bid"), Some("42"));
        assert_eq!(
            evaluate("@get:{bid}", LIST, Extraction::Values, &mut context).unwrap(),
            vec!["42"]
        );
    }

    #[test]
    fn expands_templates_from_the_context_before_executing() {
        let mut context = RuleContext::new([(String::from("cls"), String::from("odd"))]);
        assert_eq!(
            evaluate(
                "class.{{cls}}@tag.a.0@text",
                LIST,
                Extraction::Values,
                &mut context
            )
            .unwrap(),
            vec!["第一章"]
        );
    }

    #[test]
    fn renders_inline_json_and_javascript_templates() {
        let mut context = RuleContext::default();
        assert_eq!(
            evaluate(
                "书名：{{$.name}} / {{java.md5Encode('x')}}",
                r#"{"name":"斗破苍穹"}"#,
                Extraction::Values,
                &mut context,
            )
            .unwrap(),
            vec!["书名：斗破苍穹 / 9dd4e461268c8034f5c8564e155c67a6".to_owned()]
        );
    }

    #[test]
    fn expands_templates_inside_javascript_rules() {
        let mut context = RuleContext::default();
        assert_eq!(
            evaluate(
                "@js:params={'id':{{$.id}}};JSON.stringify(params)",
                r#"{"id":42}"#,
                Extraction::Values,
                &mut context,
            )
            .unwrap(),
            vec![r#"{"id":42}"#.to_owned()]
        );
    }

    #[test]
    fn renders_statement_templates_without_treating_them_as_css() {
        let mut context = RuleContext::default();
        let value = evaluate(
            "前{{if(true){result='中';} result}}后",
            "原始",
            Extraction::Values,
            &mut context,
        )
        .unwrap();
        assert_eq!(value, vec!["前中后"]);
    }

    #[test]
    fn reports_js_modes_instead_of_returning_empty() {
        assert_eq!(
            evaluate(
                "<js>result.trim()</js>",
                LIST,
                Extraction::Values,
                &mut RuleContext::default()
            )
            .unwrap(),
            vec![LIST]
        );
    }

    #[test]
    fn surfaces_the_error_only_when_every_alternative_fails() {
        let error = evaluate(
            "@Json:$.a||@Json:$.b",
            "not json",
            Extraction::Values,
            &mut RuleContext::default(),
        );
        assert!(matches!(error, Err(RuleExecutionError::InvalidJson(_))));
    }
}
