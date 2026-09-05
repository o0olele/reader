//! JSONPath evaluation for the subset used by real legado book sources.
mod filter;
mod parser;

use super::evaluator::apply_postprocess;
use super::model::{RuleExecutionError, RuleMode, SourceRule};
use filter::eval_filter;
use parser::{parse_path, Token};
use serde_json::Value;

pub fn execute_json(rule: &SourceRule, input: &str) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::Json {
        return Err(RuleExecutionError::UnsupportedMode("non-Json"));
    }
    let root: Value =
        serde_json::from_str(input).map_err(|e| RuleExecutionError::InvalidJson(e.to_string()))?;
    // Legado treats a selected JSON array as a collection of values.  This is
    // especially important for book-list rules such as `$.list` and chapter
    // rules such as `$.bookChapters`: returning the array as one JSON string
    // leaves the subsequent per-item rule with an array instead of an object.
    // Wildcard paths already yield individual values, so flattening here is
    // idempotent for those spellings and brings direct array selections in line
    // with Legado's `getAll()` semantics.
    let mut values = Vec::new();
    for value in select(&root, rule.rule.trim())? {
        push_json_values(value, &mut values);
    }
    apply_postprocess(rule, &mut values);
    Ok(values)
}

fn select<'a>(root: &'a Value, path: &str) -> Result<Vec<&'a Value>, RuleExecutionError> {
    let mut current = vec![root];
    for token in parse_path(path)? {
        current = match token {
            Token::Key(k) => current.into_iter().filter_map(|v| v.get(&k)).collect(),
            Token::RecursiveKey(k) => {
                let mut out = Vec::new();
                for v in current {
                    recursive_key(v, &k, &mut out);
                }
                out
            }
            Token::Wildcard => current.into_iter().flat_map(children).collect(),
            Token::RecursiveWildcard => {
                let mut out = Vec::new();
                for v in current {
                    descendants(v, &mut out);
                }
                out
            }
            Token::Index(i) => current.into_iter().filter_map(|v| index(v, i)).collect(),
            Token::Filter(expr) => current
                .into_iter()
                .flat_map(|v| {
                    v.as_array()
                        .into_iter()
                        .flatten()
                        .filter(|x| eval_filter(&expr, x))
                })
                .collect(),
        };
    }
    Ok(current)
}

fn children(value: &Value) -> impl Iterator<Item = &Value> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .chain(value.as_object().into_iter().flat_map(|m| m.values()))
}
fn index(value: &Value, index: isize) -> Option<&Value> {
    let a = value.as_array()?;
    let i = if index < 0 {
        a.len() as isize + index
    } else {
        index
    };
    (i >= 0).then(|| a.get(i as usize)).flatten()
}
fn recursive_key<'a>(value: &'a Value, key: &str, out: &mut Vec<&'a Value>) {
    if let Some(m) = value.as_object() {
        if let Some(v) = m.get(key) {
            out.push(v)
        }
        for v in m.values() {
            recursive_key(v, key, out)
        }
    } else if let Some(a) = value.as_array() {
        for v in a {
            recursive_key(v, key, out)
        }
    }
}
fn descendants<'a>(value: &'a Value, out: &mut Vec<&'a Value>) {
    for v in children(value) {
        out.push(v);
        descendants(v, out)
    }
}
fn to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(v) => Some(v.clone()),
        Value::Number(v) => Some(v.to_string()),
        Value::Bool(v) => Some(v.to_string()),
        Value::Null => None,
        Value::Array(_) | Value::Object(_) => Some(value.to_string()),
    }
}

fn push_json_values(value: &Value, out: &mut Vec<String>) {
    if let Value::Array(values) = value {
        for value in values {
            push_json_values(value, out);
        }
    } else if let Some(value) = to_string(value) {
        out.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_engine::rule::split_rule;
    fn rule(raw: &str) -> SourceRule {
        split_rule(raw).unwrap().remove(0).remove(0)
    }
    #[test]
    fn recursive_and_filter() {
        let r = rule("@Json:$..title");
        assert_eq!(
            execute_json(&r, r#"{"a":{"title":"A"},"items":[{"title":"B"}]}"#).unwrap(),
            vec!["A", "B"]
        );
        let r = rule("@Json:$.books[?(@.status=='done'&&@.free)].title");
        assert_eq!(execute_json(&r,r#"{"books":[{"title":"A","status":"done","free":true},{"title":"B","status":"done","free":false}]}"#).unwrap(),vec!["A"]);
    }
    #[test]
    fn exported_spellings() {
        let r = rule("@Json:$.[*]");
        assert_eq!(execute_json(&r, r#"["a","b"]"#).unwrap(), vec!["a", "b"]);
        let r = rule("@Json:$.items[-1]");
        assert_eq!(execute_json(&r, r#"{"items":[1,2,3]}"#).unwrap(), vec!["3"]);
    }

    #[test]
    fn direct_array_selection_expands_each_value() {
        let r = rule("$.list");
        assert_eq!(
            execute_json(&r, r#"{"list":[{"name":"一"},{"name":"二"}]}"#).unwrap(),
            vec![r#"{"name":"一"}"#.to_owned(), r#"{"name":"二"}"#.to_owned()]
        );
    }
}
