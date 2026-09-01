use super::model::{RuleMode, SourceRule};
use scraper::{Html, Selector};
use serde_json::Value;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum RuleExecutionError {
    #[error("regex rule is empty")]
    EmptyRule,
    #[error("regex rule is invalid: {0}")]
    InvalidRegex(String),
    #[error("rule mode {0} is not supported by this evaluator")]
    UnsupportedMode(&'static str),
    #[error("json rule is invalid: {0}")]
    InvalidJson(String),
    #[error("json path is invalid: {0}")]
    InvalidJsonPath(String),
    #[error("xpath rule is invalid: {0}")]
    InvalidXPath(String),
}

/// Execute the mode selected by `RuleAnalyzer` against a document or value.
/// CSS execution remains in `source_engine::selector`; this entry point covers
/// the non-CSS modes so callers can progressively adopt the same rule model.
pub fn execute_rule(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    match rule.mode {
        RuleMode::Regex => execute_regex(rule, input),
        RuleMode::Json => execute_json(rule, input),
        RuleMode::XPath => execute_xpath(rule, input),
        mode => Err(RuleExecutionError::UnsupportedMode(match mode {
            RuleMode::Default => "Default",
            RuleMode::XPath => "XPath",
            RuleMode::Json => "Json",
            RuleMode::Js => "Js",
            RuleMode::Regex => "Regex",
            RuleMode::WebJs => "WebJs",
        })),
    }
}

/// Execute a JSONPath rule. This intentionally implements the portable subset
/// used by legado sources: object keys, array indexes, and `[*]` wildcards.
pub fn execute_json(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::Json {
        return Err(RuleExecutionError::UnsupportedMode("non-Json"));
    }
    let value: Value = serde_json::from_str(input)
        .map_err(|error| RuleExecutionError::InvalidJson(error.to_string()))?;
    let nodes = json_path(&value, rule.rule.trim())?;
    let mut values = nodes
        .into_iter()
        .filter_map(json_value_to_string)
        .collect::<Vec<_>>();
    apply_postprocess(rule, &mut values);
    Ok(values)
}

/// Execute the common XPath subset found in book sources. HTML is parsed with
/// the same tolerant parser as CSS selectors, then XPath predicates are mapped
/// to equivalent CSS selectors where possible.
pub fn execute_xpath(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::XPath {
        return Err(RuleExecutionError::UnsupportedMode("non-XPath"));
    }
    let (selector_text, terminal) = split_xpath_terminal(rule.rule.trim())?;
    let css = xpath_to_css(selector_text)?;
    let selector = Selector::parse(&css)
        .map_err(|error| RuleExecutionError::InvalidXPath(error.to_string()))?;
    let document = Html::parse_document(input);
    let mut values = Vec::new();
    for node in document.select(&selector) {
        let value = match terminal {
            XPathTerminal::Text => node.text().collect::<Vec<_>>().join(" "),
            XPathTerminal::Attribute(name) => {
                node.value().attr(name).unwrap_or_default().to_owned()
            }
            XPathTerminal::Node => node.text().collect::<Vec<_>>().join(" "),
        };
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        if !value.is_empty() {
            values.push(value);
        }
    }
    apply_postprocess(rule, &mut values);
    Ok(values)
}

/// Executes one Regex-mode rule against text.
///
/// Legado regex rules return capture group 1 when present and the complete
/// match otherwise. Replacement directives are applied to each result after
/// matching, and a reversed rule reverses the result order.
pub fn execute_regex(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::Regex {
        return Err(RuleExecutionError::UnsupportedMode(match rule.mode {
            RuleMode::Default => "Default",
            RuleMode::XPath => "XPath",
            RuleMode::Json => "Json",
            RuleMode::Js => "Js",
            RuleMode::Regex => "Regex",
            RuleMode::WebJs => "WebJs",
        }));
    }
    if rule.rule.trim().is_empty() {
        return Err(RuleExecutionError::EmptyRule);
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

fn apply_postprocess(rule: &SourceRule, values: &mut Vec<String>) {
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

fn json_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Null => None,
        Value::Array(_) | Value::Object(_) => Some(value.to_string()),
    }
}

fn json_path<'a>(root: &'a Value, path: &str) -> Result<Vec<&'a Value>, RuleExecutionError> {
    let path = path.trim();
    if !path.starts_with('$') {
        return Err(RuleExecutionError::InvalidJsonPath(
            "path must start with `$`".into(),
        ));
    }
    let mut current = vec![root];
    let mut cursor = 1;
    while cursor < path.len() {
        let bytes = path.as_bytes();
        if bytes[cursor] == b'.' {
            cursor += 1;
            if cursor < path.len() && bytes[cursor] == b'.' {
                return Err(RuleExecutionError::InvalidJsonPath(
                    "recursive descent is not supported".into(),
                ));
            }
            let start = cursor;
            while cursor < path.len()
                && (path.as_bytes()[cursor].is_ascii_alphanumeric()
                    || path.as_bytes()[cursor] == b'_')
            {
                cursor += 1;
            }
            if start == cursor {
                return Err(RuleExecutionError::InvalidJsonPath(
                    "missing object key".into(),
                ));
            }
            current = current
                .into_iter()
                .filter_map(|value| value.get(&path[start..cursor]))
                .collect();
        } else if bytes[cursor] == b'[' {
            let end = path[cursor + 1..]
                .find(']')
                .map(|offset| cursor + 1 + offset)
                .ok_or_else(|| RuleExecutionError::InvalidJsonPath("unclosed bracket".into()))?;
            let token = path[cursor + 1..end].trim();
            if token == "*" {
                current = current
                    .into_iter()
                    .flat_map(|value| value.as_array().into_iter().flatten())
                    .collect();
            } else if let Ok(index) = token.parse::<usize>() {
                current = current
                    .into_iter()
                    .filter_map(|value| value.as_array().and_then(|items| items.get(index)))
                    .collect();
            } else if (token.starts_with('\'') && token.ends_with('\''))
                || (token.starts_with('"') && token.ends_with('"'))
            {
                let key = &token[1..token.len() - 1];
                current = current
                    .into_iter()
                    .filter_map(|value| value.get(key))
                    .collect();
            } else {
                return Err(RuleExecutionError::InvalidJsonPath(format!(
                    "unsupported bracket `{token}`"
                )));
            }
            cursor = end + 1;
        } else {
            return Err(RuleExecutionError::InvalidJsonPath(format!(
                "unexpected character at {cursor}"
            )));
        }
    }
    Ok(current)
}

#[derive(Clone, Copy)]
enum XPathTerminal<'a> {
    Node,
    Text,
    Attribute(&'a str),
}

fn split_xpath_terminal(raw: &str) -> Result<(&str, XPathTerminal<'_>), RuleExecutionError> {
    if let Some(value) = raw.strip_suffix("/text()") {
        return Ok((value, XPathTerminal::Text));
    }
    if let Some((value, attribute)) = raw.rsplit_once("/@") {
        if !attribute.is_empty() && !attribute.contains('/') {
            return Ok((value, XPathTerminal::Attribute(attribute)));
        }
    }
    Ok((raw, XPathTerminal::Node))
}

fn xpath_to_css(raw: &str) -> Result<String, RuleExecutionError> {
    let raw = raw.trim();
    let mut value = raw.strip_prefix("//").unwrap_or(raw);
    if value.starts_with('/') {
        value = value.trim_start_matches('/');
    }
    if value.is_empty() {
        return Err(RuleExecutionError::InvalidXPath("empty selector".into()));
    }
    let mut css = value.replace('/', " ");
    let predicates = regex::Regex::new(r#"\[\s*@([\w:-]+)\s*=\s*(['"])(.*?)['"]\s*\]"#)
        .expect("static xpath predicate regex");
    css = predicates.replace_all(&css, "[$1='$3']").into_owned();
    let contains = regex::Regex::new(r#"contains\(\s*@([\w:-]+)\s*,\s*(['"])(.*?)['"]\s*\)"#)
        .expect("static xpath contains regex");
    css = contains.replace_all(&css, "[$1*='$3']").into_owned();
    let position = regex::Regex::new(r"\[\s*(\d+)\s*\]").expect("static xpath index regex");
    css = position.replace_all(&css, ":nth-of-type($1)").into_owned();
    if css.contains('[') && css.contains(']') && css.contains("@") {
        return Err(RuleExecutionError::InvalidXPath(
            "unsupported predicate".into(),
        ));
    }
    Ok(css)
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
            execute_rule(&rule, r#"{"books":[{"title":"One"},{"title":"Two"}]}"#).unwrap(),
            vec!["Two"]
        );
    }

    #[test]
    fn executes_xpath_text_attributes_and_predicates() {
        let rule = regex_rule("@XPath://article[@data-id='2']/text()");
        assert_eq!(
            execute_xpath(&rule, r#"<main><article data-id='1'>One</article><article data-id='2'>Two</article></main>"#).unwrap(),
            vec!["Two"]
        );
        let rule = regex_rule("@XPath://a/@href");
        assert_eq!(
            execute_rule(&rule, r#"<a href='/chapter-1'>Chapter</a>"#).unwrap(),
            vec!["/chapter-1"]
        );
    }
}
