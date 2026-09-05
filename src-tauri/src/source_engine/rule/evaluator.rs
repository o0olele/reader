pub use super::jsonpath::execute_json;
use super::jsoup::{execute_jsoup, Extraction};
use super::model::{RuleContext, RuleExecutionError, RuleMode, SourceRule};
use super::xpath::execute_xpath;
use super::{JsContext, JsValue, QuickJsRuntime};

pub(super) fn mode_name(mode: RuleMode) -> &'static str {
    match mode {
        RuleMode::Default => "Default",
        RuleMode::XPath => "XPath",
        RuleMode::Json => "Json",
        RuleMode::Js => "Js",
        RuleMode::Regex => "Regex",
        RuleMode::WebJs => "WebJs",
    }
}

/// Execute one rule in the mode selected by `RuleAnalyzer`.
///
/// `want` only affects Default mode, the one dialect that can yield either
/// element markup or extracted strings; the other modes always yield strings.
pub fn execute_rule(
    rule: &SourceRule,
    input: &str,
    want: Extraction,
) -> Result<Vec<String>, RuleExecutionError> {
    match rule.mode {
        RuleMode::Default => execute_jsoup(rule, input, want),
        RuleMode::Regex => execute_regex(rule, input),
        RuleMode::Json => execute_json(rule, input),
        RuleMode::XPath => execute_xpath(rule, input, want),
        RuleMode::Js => execute_js(rule, input, &mut RuleContext::default()),
        mode => Err(RuleExecutionError::UnsupportedMode(mode_name(mode))),
    }
}

pub fn execute_js(
    rule: &SourceRule,
    input: &str,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::Js {
        return Err(RuleExecutionError::UnsupportedMode(mode_name(rule.mode)));
    }
    let runtime = QuickJsRuntime::default();
    let js_context = JsContext {
        result: input.to_owned(),
        base_url: context.http.as_ref().map(|http| http.base_url.clone()),
        variables: context.snapshot(),
        http: context.http.clone(),
        ..Default::default()
    };
    let (value, variables) = runtime
        .execute_blocking_with_context(&rule.rule, js_context)
        .map_err(|error| RuleExecutionError::UnsupportedJsoup(error.to_string()))?;
    context.extend(variables);
    let mut values = match value {
        JsValue::String(value) => vec![value],
        JsValue::Number(value) => vec![value.to_string()],
        JsValue::Boolean(value) => vec![value.to_string()],
        JsValue::Null => Vec::new(),
        // Legado's JS rules commonly return an array of objects for
        // `bookList`/`chapterList`.  Preserve each element as its own input
        // node so the following per-item rules (`$.name`, `$.id`, …) see the
        // object rather than one giant JSON array string.
        JsValue::Json(value) => json_values(value),
    };
    apply_postprocess(rule, &mut values);
    Ok(values)
}

fn json_values(value: serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(values) => values.into_iter().flat_map(json_values).collect(),
        value => vec![value.to_string()],
    }
}

/// Executes one Regex-mode rule against text.
///
/// Legado regex rules return capture group 1 when present and the complete
/// match otherwise. Replacement directives are applied to each result after
/// matching, and a reversed rule reverses the result order.
pub fn execute_regex(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::Regex {
        return Err(RuleExecutionError::UnsupportedMode(mode_name(rule.mode)));
    }
    if rule.rule.trim().is_empty() {
        tracing::debug!("ignoring empty regex rule");
        return Ok(Vec::new());
    }
    let regex = regex::Regex::new(&rule.rule)
        .map_err(|error| RuleExecutionError::InvalidRegex(error.to_string()))?;
    let has_capture = regex.captures_len() > 1;
    let mut values = regex
        .captures_iter(input)
        .filter_map(|captures| {
            if has_capture {
                captures.get(1).map(|value| value.as_str().to_owned())
            } else {
                captures.get(0).map(|value| value.as_str().to_owned())
            }
        })
        .collect::<Vec<_>>();

    apply_postprocess(rule, &mut values);
    Ok(values)
}

pub(super) fn apply_postprocess(rule: &SourceRule, values: &mut Vec<String>) {
    if let Some(replacement) = &rule.replace {
        for value in &mut *values {
            let replaced = if replacement.first_only {
                replacement
                    .pattern
                    .replacen(value, 1, replacement.value.as_str())
            } else {
                replacement
                    .pattern
                    .replace_all(value, replacement.value.as_str())
            };
            *value = replaced.into_owned();
        }
    }
    if rule.reverse {
        values.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_engine::rule::split_rule;

    fn regex_rule(raw: &str) -> SourceRule {
        split_rule(raw).unwrap().remove(0).remove(0)
    }

    #[test]
    fn returns_capture_group_one_for_each_match() {
        let rule = regex_rule(":chapter ([0-9]+)");
        assert_eq!(
            execute_regex(&rule, "chapter 1\nchapter 12").unwrap(),
            vec!["1", "12"]
        );
    }

    #[test]
    fn returns_full_match_without_capture_groups() {
        let rule = regex_rule(":chapter [0-9]+");
        assert_eq!(
            execute_regex(&rule, "chapter 1").unwrap(),
            vec!["chapter 1"]
        );
    }

    #[test]
    fn applies_replacement_and_reverse() {
        let rule = regex_rule("-:(one|two)##(one|two)##$1!");
        assert_eq!(
            execute_regex(&rule, "one two").unwrap(),
            vec!["two!", "one!"]
        );
        let rule = regex_rule(":(one|two)##(one|two)##$1!");
        assert_eq!(
            execute_regex(&rule, "one two").unwrap(),
            vec!["one!", "two!"]
        );
    }

    #[test]
    fn rejects_invalid_or_non_regex_rules() {
        let mut rule = regex_rule(":valid");
        rule.rule = "[".into();
        assert!(matches!(
            execute_regex(&rule, "input"),
            Err(RuleExecutionError::InvalidRegex(_))
        ));
        rule.mode = RuleMode::Default;
        assert_eq!(
            execute_regex(&rule, "input").unwrap_err(),
            RuleExecutionError::UnsupportedMode("Default")
        );
    }

    #[test]
    fn executes_jsonpath_keys_indexes_and_wildcards() {
        let rule = regex_rule("@Json:$.books[*].title");
        assert_eq!(
            execute_json(&rule, r#"{"books":[{"title":"One"},{"title":"Two"}]}"#).unwrap(),
            vec!["One", "Two"]
        );
        let rule = regex_rule("@Json:$.books[1].title");
        assert_eq!(
            execute_rule(
                &rule,
                r#"{"books":[{"title":"One"},{"title":"Two"}]}"#,
                Extraction::Values
            )
            .unwrap(),
            vec!["Two"]
        );
    }

    #[test]
    fn expands_arrays_returned_by_javascript_rules() {
        let rule = regex_rule("@js:[{name:'一'},{name:'二'}]");
        assert_eq!(
            execute_js(&rule, "", &mut RuleContext::default()).unwrap(),
            vec![r#"{"name":"一"}"#, r#"{"name":"二"}"#]
        );
    }
}
