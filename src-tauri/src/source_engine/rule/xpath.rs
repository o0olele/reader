//! XPath-mode evaluation.
//!
//! Book sources use a narrow, mostly path-and-predicate slice of XPath. Rather
//! than pull in an XML engine — which would also mean giving up the tolerant
//! HTML parsing the rest of the engine relies on — that slice is translated to
//! the equivalent CSS selector and run through `scraper`. Anything outside the
//! slice is reported, never silently matched as nothing.

use super::evaluator::apply_postprocess;
use super::jsoup::{normalized_text, Extraction};
use super::model::{RuleExecutionError, RuleMode, SourceRule};
use scraper::{Html, Selector};

pub fn execute_xpath(
    rule: &SourceRule,
    input: &str,
    want: Extraction,
) -> Result<Vec<String>, RuleExecutionError> {
    if rule.mode != RuleMode::XPath {
        return Err(RuleExecutionError::UnsupportedMode("non-XPath"));
    }
    let (selector_text, terminal) = split_terminal(rule.rule.trim());
    let css = to_css(selector_text)?;
    let selector = Selector::parse(&css)
        .map_err(|error| RuleExecutionError::InvalidXPath(error.to_string()))?;
    let document = Html::parse_fragment(input);
    let mut values = Vec::new();
    for node in document.select(&selector) {
        let value = match terminal {
            Terminal::Text => normalized_text(node),
            Terminal::Attribute(name) => node
                .value()
                .attr(name)
                .unwrap_or_default()
                .trim()
                .to_owned(),
            // A bare node path feeds either a follow-up rule or a text read.
            Terminal::Node => match want {
                Extraction::Nodes => node.html(),
                Extraction::Values => normalized_text(node),
            },
        };
        if !value.is_empty() {
            values.push(value);
        }
    }
    apply_postprocess(rule, &mut values);
    Ok(values)
}

#[derive(Clone, Copy)]
enum Terminal<'a> {
    Node,
    Text,
    Attribute(&'a str),
}

fn split_terminal(raw: &str) -> (&str, Terminal<'_>) {
    if let Some(value) = raw.strip_suffix("/text()") {
        return (value, Terminal::Text);
    }
    if let Some((value, attribute)) = raw.rsplit_once("/@") {
        if !attribute.is_empty() && !attribute.contains('/') {
            return (value, Terminal::Attribute(attribute));
        }
    }
    (raw, Terminal::Node)
}

fn to_css(raw: &str) -> Result<String, RuleExecutionError> {
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
    // The brackets have to be consumed along with the call, or the rewritten
    // predicate ends up nested inside the original ones.
    let contains =
        regex::Regex::new(r#"\[\s*contains\(\s*@([\w:-]+)\s*,\s*(['"])(.*?)['"]\s*\)\s*\]"#)
            .expect("static xpath contains regex");
    css = contains.replace_all(&css, "[$1*='$3']").into_owned();
    let position = regex::Regex::new(r"\[\s*(\d+)\s*\]").expect("static xpath index regex");
    css = position.replace_all(&css, ":nth-of-type($1)").into_owned();
    // Any `@` left inside a predicate is an axis or function this translation
    // does not cover; passing it to `Selector::parse` would only mismatch.
    if css.contains('[') && css.contains(']') && css.contains('@') {
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

    fn xpath_rule(raw: &str) -> SourceRule {
        split_rule(raw).unwrap().remove(0).remove(0)
    }

    #[test]
    fn executes_text_attributes_and_predicates() {
        let rule = xpath_rule("@XPath://article[@data-id='2']/text()");
        assert_eq!(
            execute_xpath(
                &rule,
                r#"<main><article data-id='1'>One</article><article data-id='2'>Two</article></main>"#,
                Extraction::Values
            )
            .unwrap(),
            vec!["Two"]
        );
        let rule = xpath_rule("@XPath://a/@href");
        assert_eq!(
            execute_xpath(
                &rule,
                r#"<a href='/chapter-1'>Chapter</a>"#,
                Extraction::Values
            )
            .unwrap(),
            vec!["/chapter-1"]
        );
    }

    #[test]
    fn node_paths_yield_markup_or_text_by_request() {
        let rule = xpath_rule("@XPath://article");
        let html = r#"<main><article data-id='1'>One</article></main>"#;
        assert_eq!(
            execute_xpath(&rule, html, Extraction::Values).unwrap(),
            vec!["One"]
        );
        assert!(execute_xpath(&rule, html, Extraction::Nodes).unwrap()[0].contains("data-id"));
    }

    #[test]
    fn translates_contains_and_positional_predicates() {
        assert_eq!(
            to_css("//div[contains(@class,'x')]").unwrap(),
            "div[class*='x']"
        );
        assert_eq!(to_css("//ul/li[2]").unwrap(), "ul li:nth-of-type(2)");
    }

    #[test]
    fn reports_predicates_outside_the_translated_slice() {
        assert!(matches!(
            to_css("//a[@href and @title]"),
            Err(RuleExecutionError::InvalidXPath(_))
        ));
        assert!(matches!(
            to_css("//"),
            Err(RuleExecutionError::InvalidXPath(_))
        ));
    }
}
