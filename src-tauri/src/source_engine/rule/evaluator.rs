use super::model::{RuleMode, SourceRule};

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum RuleExecutionError {
    #[error("regex rule is empty")]
    EmptyRule,
    #[error("regex rule is invalid: {0}")]
    InvalidRegex(String),
    #[error("rule mode {0} is not supported by this evaluator")]
    UnsupportedMode(&'static str),
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

    if let Some(replacement) = &rule.replace {
        for value in &mut values {
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
    Ok(values)
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
}
