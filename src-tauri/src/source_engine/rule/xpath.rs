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
    let query = to_query(selector_text)?;
    let selector = Selector::parse(&query.css)
        .map_err(|error| RuleExecutionError::InvalidXPath(error.to_string()))?;
    let document = Html::parse_fragment(input);
    let mut values = Vec::new();
    for node in document.select(&selector) {
        if !query.matches_text(&node) {
            continue;
        }
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
    if let Some(value) = raw.strip_suffix("text()") {
        return (value.trim_end_matches('/'), Terminal::Text);
    }
    if let Some((value, attribute)) = raw.rsplit_once("/@") {
        if !attribute.is_empty() && !attribute.contains('/') {
            return (value.trim_end_matches('/'), Terminal::Attribute(attribute));
        }
    }
    (raw, Terminal::Node)
}

#[derive(Default)]
struct XPathQuery {
    css: String,
    text_equals: Option<String>,
    text_contains: Option<String>,
}

impl XPathQuery {
    fn matches_text(&self, node: &scraper::ElementRef<'_>) -> bool {
        let text = normalized_text(*node);
        self.text_equals.as_ref().is_none_or(|value| text == *value)
            && self
                .text_contains
                .as_ref()
                .is_none_or(|value| text.contains(value))
    }
}

fn to_query(raw: &str) -> Result<XPathQuery, RuleExecutionError> {
    let raw = raw.trim();
    let mut value = raw.strip_prefix("//").unwrap_or(raw);
    if value.starts_with('/') {
        value = value.trim_start_matches('/');
    }
    if value.is_empty() {
        return Err(RuleExecutionError::InvalidXPath("empty selector".into()));
    }
    let mut css = xpath_path_to_css(value);
    let mut query = XPathQuery::default();
    let predicates = regex::Regex::new(r#"\[\s*@([\w:-]+)\s*=\s*(['"])(.*?)['"]\s*\]"#)
        .expect("static xpath predicate regex");
    css = predicates.replace_all(&css, "[$1='$3']").into_owned();
    // The brackets have to be consumed along with the call, or the rewritten
    // predicate ends up nested inside the original ones.
    let contains =
        regex::Regex::new(r#"\[\s*contains\(\s*@([\w:-]+)\s*,\s*(['"])(.*?)['"]\s*\)\s*\]"#)
            .expect("static xpath contains regex");
    css = contains.replace_all(&css, "[$1*='$3']").into_owned();
    let not_attribute =
        regex::Regex::new(r#"\[\s*not\(\s*@([\w:-]+)\s*=\s*(['"])(.*?)['"]\s*\)\s*\]"#)
            .expect("static xpath not attribute regex");
    css = not_attribute
        .replace_all(&css, ":not([$1='$3'])")
        .into_owned();
    let text_equals = regex::Regex::new(r#"\[\s*text\(\)\s*=\s*(['"])(.*?)['"]\s*\]"#)
        .expect("static xpath text equality regex");
    if let Some(captures) = text_equals.captures(&css) {
        query.text_equals = captures.get(2).map(|value| value.as_str().to_owned());
        css = text_equals.replace(&css, "").into_owned();
    }
    let text_contains =
        regex::Regex::new(r#"\[\s*contains\(\s*text\(\)\s*,\s*(['"])(.*?)['"]\s*\)\s*\]"#)
            .expect("static xpath text contains regex");
    if let Some(captures) = text_contains.captures(&css) {
        query.text_contains = captures.get(2).map(|value| value.as_str().to_owned());
        css = text_contains.replace(&css, "").into_owned();
    }
    let position_range = regex::Regex::new(
        r"\[\s*position\(\)\s*>=\s*(\d+)\s+and\s+position\(\)\s*<\s*last\(\)\s*\]",
    )
    .expect("static xpath position range regex");
    css = position_range
        .replace_all(&css, ":nth-of-type(n+$1):not(:last-of-type)")
        .into_owned();
    let before_last = regex::Regex::new(r"\[\s*position\(\)\s*<\s*last\(\)\s*-\s*(\d+)\s*\]")
        .expect("static xpath before last regex");
    css = before_last
        .replace_all(&css, ":not(:nth-last-of-type(-n+$1))")
        .into_owned();
    let minimum = regex::Regex::new(r"\[\s*position\(\)\s*>=\s*(\d+)\s*\]")
        .expect("static xpath minimum position regex");
    css = minimum.replace_all(&css, ":nth-of-type(n+$1)").into_owned();
    let before_last_one = regex::Regex::new(r"\[\s*position\(\)\s*<\s*last\(\)\s*\]")
        .expect("static xpath before last regex");
    css = before_last_one
        .replace_all(&css, ":not(:last-of-type)")
        .into_owned();
    let position = regex::Regex::new(r"\[\s*(\d+)\s*\]").expect("static xpath index regex");
    css = position.replace_all(&css, ":nth-of-type($1)").into_owned();
    // Any `@` left inside a predicate is an axis or function this translation
    // does not cover; passing it to `Selector::parse` would only mismatch.
    if css.contains('[') && css.contains(']') && css.contains('@') {
        return Err(RuleExecutionError::InvalidXPath(
            "unsupported predicate".into(),
        ));
    }
    query.css = css;
    Ok(query)
}

fn xpath_path_to_css(value: &str) -> String {
    let mut css = String::with_capacity(value.len() + 8);
    let mut i = 0;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    while i < value.len() {
        let tail = &value[i..];
        let ch = tail.chars().next().expect("cursor is inside the string");
        if let Some(active) = quote {
            css.push(ch);
            if ch == active {
                quote = None;
            }
            i += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            css.push(ch);
        } else if ch == '[' {
            bracket_depth += 1;
            css.push(ch);
        } else if ch == ']' {
            bracket_depth = bracket_depth.saturating_sub(1);
            css.push(ch);
        } else if bracket_depth == 0 && tail.starts_with("following-sibling::") {
            css.push_str(" ~ ");
            i += "following-sibling::".len();
            continue;
        } else if bracket_depth == 0 && tail.starts_with("/following-sibling::") {
            css.push_str(" ~ ");
            i += "/following-sibling::".len();
            continue;
        } else if bracket_depth == 0 && ch == '/' {
            if tail.starts_with("//") {
                css.push(' ');
                i += 1;
            } else {
                css.push_str(" > ");
            }
        } else {
            css.push(ch);
        }
        i += ch.len_utf8();
    }
    css
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
            to_query("//div[contains(@class,'x')]").unwrap().css,
            "div[class*='x']"
        );
        assert_eq!(
            to_query("//ul/li[2]").unwrap().css,
            "ul > li:nth-of-type(2)"
        );
    }

    #[test]
    fn reports_predicates_outside_the_translated_slice() {
        assert!(matches!(
            to_query("//a[@href and @title]"),
            Err(RuleExecutionError::InvalidXPath(_))
        ));
        assert!(matches!(
            to_query("//"),
            Err(RuleExecutionError::InvalidXPath(_))
        ));
    }

    #[test]
    fn translates_real_corpus_axes_functions_and_position_ranges() {
        assert_eq!(
            to_query("//span/following-sibling::a").unwrap().css,
            "span ~ a"
        );
        assert_eq!(
            to_query("//div[@id='list']/dl/dd[position()>=13]")
                .unwrap()
                .css,
            "div[id='list'] > dl > dd:nth-of-type(n+13)"
        );
        let query = to_query("//*[contains(text(), 'next')]").unwrap();
        assert_eq!(query.css, "*");
        assert_eq!(query.text_contains.as_deref(), Some("next"));
        assert_eq!(
            to_query("//*[not(@class='blue')]").unwrap().css,
            "*:not([class='blue'])"
        );
    }
}
