//! Default (JSoup) mode evaluation.
//!
//! Legado's default mode mixes two selector dialects inside one rule string,
//! separated by `@`:
//!
//! ```text
//! class.odd.0@tag.a.0@text     private syntax: kind.name(.index)
//! .book-list li@href           CSS + attribute terminal
//! id.content@textNodes
//! tag.div.-1@html
//! text.下一章@href             match by element text
//! ```
//!
//! Unsupported spellings return [`RuleExecutionError::UnsupportedJsoup`] rather
//! than an empty result, so callers can tell "no match" from "cannot execute".

use super::evaluator::apply_postprocess;
use super::model::{RuleExecutionError, SourceRule};
use super::position::{apply_positions, PositionFilter};
use super::step::{parse_step, Step, Terminal};
use scraper::{ElementRef, Html, Selector};

/// What the caller wants back: element markup, or extracted strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extraction {
    /// Outer HTML of each matched element, re-parsable by a follow-up rule.
    Nodes,
    /// Text or attribute values.
    Values,
}

/// Collapses all descendant text into a single whitespace-normalized string.
pub fn normalized_text(element: ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn own_text(element: ElementRef<'_>) -> String {
    element
        .children()
        .filter_map(|child| child.value().as_text().map(|text| text.trim()))
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn execute_jsoup(
    rule: &SourceRule,
    input: &str,
    want: Extraction,
) -> Result<Vec<String>, RuleExecutionError> {
    let raw = rule.rule.trim();
    if raw.is_empty() {
        return Err(RuleExecutionError::EmptyRule);
    }
    let document = Html::parse_fragment(input);
    let mut nodes = vec![document.root_element()];
    let steps = split_steps(raw);

    for (index, step) in steps.iter().enumerate() {
        let is_last = index + 1 == steps.len();
        match parse_step(step, is_last)? {
            Step::Terminal(terminal) if is_last => {
                let mut values = collect_terminal(&nodes, terminal);
                apply_postprocess(rule, &mut values);
                return Ok(values);
            }
            // A terminal in the middle of a chain has no element to hand on.
            Step::Terminal(_) => {
                return Err(RuleExecutionError::UnsupportedJsoup(format!(
                    "`{step}` produces text but is not the last step"
                )))
            }
            Step::Select(selection) => nodes = apply_selection(&nodes, &selection)?,
        }
    }

    let mut values = match want {
        Extraction::Nodes => nodes.iter().map(|node| node.html()).collect(),
        Extraction::Values => nodes
            .iter()
            .map(|node| normalized_text(*node))
            .filter(|value| !value.is_empty())
            .collect(),
    };
    apply_postprocess(rule, &mut values);
    Ok(values)
}

/// A selection step: a matcher plus an optional index into its matches.
pub(super) struct Selection {
    pub matcher: Matcher,
    pub positions: PositionFilter,
}

pub(super) enum Matcher {
    Css(String),
    /// Elements whose normalized text contains this needle.
    Text(String),
    /// Direct element children.
    Children,
}

fn apply_selection<'a>(
    nodes: &[ElementRef<'a>],
    selection: &Selection,
) -> Result<Vec<ElementRef<'a>>, RuleExecutionError> {
    let selector = match &selection.matcher {
        Matcher::Css(css) => Some(Selector::parse(css).map_err(|error| {
            RuleExecutionError::UnsupportedJsoup(format!("`{css}` is not a CSS selector: {error}"))
        })?),
        _ => None,
    };
    let mut matched = Vec::new();
    for node in nodes {
        let candidates = match (&selection.matcher, &selector) {
            (Matcher::Css(_), Some(selector)) => node.select(selector).collect(),
            (Matcher::Text(needle), _) => node
                .descendants()
                .filter_map(ElementRef::wrap)
                .filter(|element| own_text(*element).contains(needle.as_str()))
                .collect(),
            (Matcher::Children, _) => node.children().filter_map(ElementRef::wrap).collect(),
            _ => unreachable!("CSS matchers always have a compiled selector"),
        };
        matched.extend(apply_positions(candidates, &selection.positions));
    }
    Ok(matched)
}

fn collect_terminal(nodes: &[ElementRef<'_>], terminal: Terminal<'_>) -> Vec<String> {
    if matches!(terminal, Terminal::Html | Terminal::All) {
        let markup = nodes
            .iter()
            .map(|node| node.html())
            .collect::<Vec<_>>()
            .join("\n");
        return (!markup.is_empty()).then_some(markup).into_iter().collect();
    }
    let mut values = Vec::new();
    for node in nodes {
        match terminal {
            Terminal::Text => values.push(normalized_text(*node)),
            Terminal::OwnText => values.push(own_text(*node)),
            // Legado returns one newline-joined value per element.
            Terminal::TextNodes => values.push(
                node.children()
                    .filter_map(|child| child.value().as_text().map(|text| text.to_string()))
                    .flat_map(|text| {
                        text.lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty())
                            .map(str::to_owned)
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Terminal::Html | Terminal::All => unreachable!("markup terminals return above"),
            Terminal::Attribute(name) => values.push(
                node.value()
                    .attr(name)
                    .unwrap_or_default()
                    .trim()
                    .to_owned(),
            ),
        }
    }
    values.retain(|value| !value.is_empty());
    values
}

/// Splits on `@` at nesting depth zero, leaving `::attr(...)`, bracketed CSS
/// filters and quoted strings intact.
fn split_steps(raw: &str) -> Vec<&str> {
    let mut steps = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    let mut quote = None;
    for (index, character) in raw.char_indices() {
        match quote {
            Some(active) if character == active => quote = None,
            Some(_) => {}
            None => match character {
                '\'' | '"' => quote = Some(character),
                '[' | '(' => depth += 1,
                ']' | ')' => depth -= 1,
                '@' if depth == 0 => {
                    steps.push(raw[start..index].trim());
                    start = index + 1;
                }
                _ => {}
            },
        }
    }
    steps.push(raw[start..].trim());
    steps.retain(|step| !step.is_empty());
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_engine::rule::split_rule;

    fn rule(raw: &str) -> SourceRule {
        split_rule(raw).unwrap().remove(0).remove(0)
    }

    fn run(raw: &str, html: &str) -> Vec<String> {
        execute_jsoup(&rule(raw), html, Extraction::Values).unwrap()
    }

    const LIST: &str = r#"<ul class="odd"><li><a href="/one">第一章</a></li><li><a href="/two">第二章</a></li></ul>"#;

    #[test]
    fn executes_private_syntax_with_kind_name_and_index() {
        assert_eq!(run("class.odd@tag.a.0@text", LIST), vec!["第一章"]);
        assert_eq!(run("class.odd@tag.a.-1@href", LIST), vec!["/two"]);
        assert_eq!(run("tag.a@text", LIST), vec!["第一章", "第二章"]);
    }

    #[test]
    fn applies_legacy_position_lists_and_exclusions() {
        let html = "<ul><li>零</li><li>一</li><li>二</li><li>三</li></ul>";
        assert_eq!(run("tag.li.0:2:-1@text", html), vec!["零", "二", "三"]);
        assert_eq!(run("tag.li!0:2@text", html), vec!["一", "三"]);
        assert_eq!(run("tag.li.!-1@text", html), vec!["零", "一", "二"]);
    }

    #[test]
    fn applies_bracket_ranges_including_reverse_and_exclusion() {
        let html = "<ul><li>零</li><li>一</li><li>二</li><li>三</li></ul>";
        assert_eq!(run("tag.li[1:3]@text", html), vec!["一", "二", "三"]);
        assert_eq!(run("tag.li[-1:0]@text", html), vec!["三", "二", "一", "零"]);
        assert_eq!(run("tag.li[!1:2]@text", html), vec!["零", "三"]);
    }

    #[test]
    fn applies_positions_independently_for_each_parent() {
        let html = "<div><i>A0</i><i>A1</i></div><div><i>B0</i><i>B1</i></div>";
        assert_eq!(run("tag.div@tag.i.0@text", html), vec!["A0", "B0"]);
    }

    #[test]
    fn executes_css_selectors_and_attribute_terminals() {
        assert_eq!(run("ul.odd li a@href", LIST), vec!["/one", "/two"]);
        assert_eq!(run(".odd a.0@text", LIST), vec!["第一章"]);
    }

    #[test]
    fn matches_elements_by_text_content() {
        assert_eq!(run("text.第二章@href", LIST), vec!["/two"]);
    }

    #[test]
    fn distinguishes_text_owntext_and_textnodes() {
        let html = r#"<div id="c">前<span>中</span>后</div>"#;
        assert_eq!(run("id.c@text", html), vec!["前 中 后"]);
        assert_eq!(run("id.c@ownText", html), vec!["前 后"]);
        assert_eq!(run("id.c@textNodes", html), vec!["前\n后"]);
    }

    #[test]
    fn combines_markup_terminals_into_one_value() {
        let html = "<p>一</p><p>二</p>";
        let all = run("tag.p@all", html);
        assert_eq!(all.len(), 1);
        assert!(all[0].contains("<p>一</p>\n<p>二</p>"));
    }

    #[test]
    fn returns_node_markup_when_extraction_is_nodes() {
        let nodes = execute_jsoup(&rule("tag.li"), LIST, Extraction::Nodes).unwrap();
        assert_eq!(nodes.len(), 2);
        assert!(nodes[0].contains(r#"href="/one""#));
    }

    #[test]
    fn reports_unsupported_spellings_instead_of_returning_empty() {
        assert!(matches!(
            execute_jsoup(&rule("class.odd@text@tag.a"), LIST, Extraction::Values),
            Err(RuleExecutionError::UnsupportedJsoup(_))
        ));
    }

    #[test]
    fn splits_steps_without_breaking_attribute_filters() {
        assert_eq!(
            split_steps("a[data-x='p@q']@href"),
            vec!["a[data-x='p@q']", "href"]
        );
        assert_eq!(split_steps("a::attr(href)"), vec!["a::attr(href)"]);
    }
}
