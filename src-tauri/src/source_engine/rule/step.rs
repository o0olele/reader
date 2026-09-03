//! Parsing of a single `@`-separated step inside a Default (JSoup) rule.

use super::jsoup::{Matcher, Selection};
use super::model::RuleExecutionError;
use super::position::split_position;

/// The extraction terminals legado's default mode understands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Terminal<'a> {
    Text,
    TextNodes,
    OwnText,
    Html,
    All,
    Attribute(&'a str),
}

pub(super) enum Step<'a> {
    Select(Selection),
    Terminal(Terminal<'a>),
}

/// Terminals that always end a rule, whatever their position. Recognizing them
/// unconditionally lets a misplaced one be reported instead of silently
/// degrading into a CSS selector that matches nothing.
fn keyword_terminal(step: &str) -> Option<Terminal<'static>> {
    match step {
        "text" => Some(Terminal::Text),
        "textNodes" => Some(Terminal::TextNodes),
        "ownText" => Some(Terminal::OwnText),
        "html" | "outerHtml" => Some(Terminal::Html),
        "all" => Some(Terminal::All),
        _ => None,
    }
}

/// A bare identifier — no CSS punctuation, no dots — is an attribute name when
/// it closes the rule, and a tag selector anywhere else.
fn is_bare_identifier(step: &str) -> bool {
    !step.is_empty()
        && step
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub(super) fn parse_step(step: &str, is_last: bool) -> Result<Step<'_>, RuleExecutionError> {
    if let Some(terminal) = keyword_terminal(step) {
        return Ok(Step::Terminal(terminal));
    }
    if is_last && is_bare_identifier(step) && private_kind(step).is_none() {
        return Ok(Step::Terminal(Terminal::Attribute(step)));
    }
    parse_selection(step).map(Step::Select)
}

fn private_kind(step: &str) -> Option<(&str, &str)> {
    let (kind, rest) = step.split_once('.')?;
    matches!(kind, "class" | "id" | "tag" | "text" | "children").then_some((kind, rest))
}

fn parse_selection(step: &str) -> Result<Selection, RuleExecutionError> {
    let (body, positions) = split_position(step);
    if body.is_empty() {
        return Ok(Selection {
            matcher: Matcher::Children,
            positions,
        });
    }
    if let Some((kind, name)) = private_kind(body) {
        let matcher = match kind {
            "class" => Matcher::Css(format!(".{name}")),
            "id" => Matcher::Css(format!("#{name}")),
            "tag" => Matcher::Css(name.to_owned()),
            "text" => Matcher::Text(name.to_owned()),
            _ => Matcher::Children,
        };
        return Ok(Selection { matcher, positions });
    }
    if body == "children" {
        return Ok(Selection {
            matcher: Matcher::Children,
            positions,
        });
    }
    Ok(Selection {
        matcher: Matcher::Css(body.to_owned()),
        positions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selection(step: &str) -> Selection {
        match parse_step(step, false).unwrap() {
            Step::Select(selection) => selection,
            Step::Terminal(_) => panic!("{step} parsed as a terminal"),
        }
    }

    #[test]
    fn recognizes_keyword_terminals_in_any_position() {
        assert!(matches!(
            parse_step("text", false).unwrap(),
            Step::Terminal(Terminal::Text)
        ));
        assert!(matches!(
            parse_step("textNodes", true).unwrap(),
            Step::Terminal(Terminal::TextNodes)
        ));
    }

    #[test]
    fn treats_bare_identifiers_as_attributes_only_at_the_end() {
        assert!(matches!(
            parse_step("href", true).unwrap(),
            Step::Terminal(Terminal::Attribute("href"))
        ));
        assert!(matches!(
            parse_step("href", false).unwrap(),
            Step::Select(_)
        ));
    }

    #[test]
    fn maps_private_kinds_and_indexes() {
        assert!(matches!(selection("class.odd").matcher, Matcher::Css(ref css) if css == ".odd"));
        assert!(matches!(selection("id.main").matcher, Matcher::Css(ref css) if css == "#main"));
        assert_ne!(
            selection("tag.a.-1").positions,
            super::super::position::PositionFilter::All
        );
        assert!(matches!(selection("text.下一章").matcher, Matcher::Text(_)));
        assert!(matches!(selection("children.0").matcher, Matcher::Children));
    }

    #[test]
    fn keeps_dotted_css_class_names_intact() {
        assert!(
            matches!(selection(".col-2 a").matcher, Matcher::Css(ref css) if css == ".col-2 a")
        );
        assert_eq!(
            selection(".col-2 a").positions,
            super::super::position::PositionFilter::All
        );
    }

    #[test]
    fn parses_ranges_and_exclusions() {
        assert!(matches!(
            parse_step("tag.a.0:2", false),
            Ok(Step::Select(_))
        ));
        assert!(matches!(
            parse_step("tag.a[!0:2]", false),
            Ok(Step::Select(_))
        ));
    }
}
