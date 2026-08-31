use super::model::{RuleParseError, RuleReplacement};
use super::scanner::find_ignore_ascii_case;
use regex::Regex;
use std::collections::HashMap;

pub(super) fn extract_replacement(
    raw: &str,
) -> Result<(String, Option<RuleReplacement>), RuleParseError> {
    let Some(first) = raw.find("##") else {
        return Ok((raw.trim().to_owned(), None));
    };
    let selector = raw[..first].trim();
    let remaining = &raw[first + 2..];
    let (pattern, replacement) = remaining.split_once("##").unwrap_or((remaining, ""));
    let first_only = replacement.ends_with("###");
    let value = if first_only {
        replacement[..replacement.len() - 3].to_owned()
    } else {
        replacement.to_owned()
    };
    let pattern =
        Regex::new(pattern).map_err(|error| RuleParseError::InvalidRegex(error.to_string()))?;
    Ok((
        selector.to_owned(),
        Some(RuleReplacement {
            pattern,
            value,
            first_only,
        }),
    ))
}

pub(super) fn extract_put(raw: &str) -> Result<(HashMap<String, String>, String), RuleParseError> {
    let mut values = HashMap::new();
    let mut output = String::with_capacity(raw.len());
    let mut cursor = 0;
    while let Some(relative) = find_ignore_ascii_case(raw, cursor, "@put:") {
        output.push_str(&raw[cursor..relative]);
        let object_start = relative + 5;
        if raw.as_bytes().get(object_start) != Some(&b'{') {
            output.push_str("@put:");
            cursor = object_start;
            continue;
        }
        let object_end =
            find_balanced_end(raw, object_start).ok_or(RuleParseError::Unclosed("@put object"))?;
        let parsed = parse_put_object(&raw[object_start..=object_end])?;
        values.extend(parsed);
        cursor = object_end + 1;
    }
    output.push_str(&raw[cursor..]);
    Ok((values, output))
}

pub(super) fn extract_get(raw: &str) -> (Vec<String>, String) {
    let mut result = Vec::new();
    let mut cleaned = String::with_capacity(raw.len());
    let mut cursor = 0;
    while let Some(start) = find_ignore_ascii_case(raw, cursor, "@get:") {
        cleaned.push_str(&raw[cursor..start]);
        let value_start = start + 5;
        if raw.as_bytes().get(value_start) == Some(&b'{') {
            if let Some(end) = raw[value_start + 1..].find('}') {
                result.push(
                    raw[value_start + 1..value_start + 1 + end]
                        .trim()
                        .to_owned(),
                );
                cursor = value_start + end + 2;
                continue;
            }
        }
        let end = raw[value_start..]
            .find(|character: char| !(character.is_alphanumeric() || character == '_'))
            .map(|offset| value_start + offset)
            .unwrap_or(raw.len());
        if end > value_start {
            result.push(raw[value_start..end].to_owned());
        }
        cursor = end.max(value_start + 1);
    }
    cleaned.push_str(&raw[cursor..]);
    (result, cleaned)
}

pub(super) fn extract_templates(raw: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut cursor = 0;
    while let Some(relative) = raw[cursor..].find("{{") {
        let start = cursor + relative + 2;
        let Some(end) = raw[start..].find("}}") else {
            break;
        };
        result.push(raw[start..start + end].trim().to_owned());
        cursor = start + end + 2;
    }
    result
}

fn parse_put_object(object: &str) -> Result<HashMap<String, String>, RuleParseError> {
    if let Ok(parsed) = serde_json::from_str(object) {
        return Ok(parsed);
    }
    let inner = object
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| RuleParseError::InvalidPut("object must be enclosed in braces".into()))?;
    let mut values = HashMap::new();
    for entry in split_comma_separated(inner) {
        let Some((key, value)) = entry.split_once(':') else {
            return Err(RuleParseError::InvalidPut(entry.into()));
        };
        let key = unquote(key.trim());
        let value = unquote(value.trim());
        if key.is_empty() {
            return Err(RuleParseError::InvalidPut("empty key".into()));
        }
        values.insert(key, value);
    }
    Ok(values)
}

fn split_comma_separated(value: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = 0;
    let mut quote = None;
    let mut depth = 0;
    for (index, character) in value.char_indices() {
        if let Some(active) = quote {
            if character == active {
                quote = None;
            }
        } else if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '{' || character == '[' {
            depth += 1;
        } else if character == '}' || character == ']' {
            depth -= 1;
        } else if character == ',' && depth == 0 {
            entries.push(value[start..index].trim());
            start = index + 1;
        }
    }
    if start < value.len() {
        entries.push(value[start..].trim());
    }
    entries
}

fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(value)
        .trim()
        .to_owned()
}

fn find_balanced_end(raw: &str, start: usize) -> Option<usize> {
    let mut depth = 0;
    let mut quote = None;
    let mut escaped = false;
    for (offset, character) in raw[start..].char_indices() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if let Some(active) = quote {
            if character == active {
                quote = None;
            }
        } else if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '{' {
            depth += 1;
        } else if character == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(start + offset);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_first_only_replacement() {
        let (rule, replacement) = extract_replacement(".body##ads?#####").unwrap();
        assert_eq!(rule, ".body");
        let replacement = replacement.unwrap();
        assert!(replacement.first_only);
        assert_eq!(replacement.value, "");
    }

    #[test]
    fn parses_legacy_put_and_braced_get() {
        let (put, rule) = extract_put(r#"@put:{bid:"123", token:'abc'}.body"#).unwrap();
        assert_eq!(put.get("bid").map(String::as_str), Some("123"));
        assert_eq!(put.get("token").map(String::as_str), Some("abc"));
        let (get, rule) = extract_get(&format!("@get:{{bid}}{rule}"));
        assert_eq!(get, vec!["bid"]);
        assert_eq!(rule, ".body");
    }
}
