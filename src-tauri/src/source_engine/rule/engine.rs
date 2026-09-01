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
use super::{expand_template, split_rule};

/// Parses `raw` and executes it against `input`.
pub fn evaluate(
    raw: &str,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, RuleExecutionError> {
    let alternatives = split_rule(raw)?;
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
    let effective = if expanded == rule.rule {
        rule.clone()
    } else {
        SourceRule {
            rule: expanded,
            ..rule.clone()
        }
    };
    if effective.mode == RuleMode::Js {
        return execute_js(&effective, input, context);
    }
    execute_rule(&effective, input, want)
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
