//! JSoup's regex attribute operator `[attr~=pattern]`.
//!
//! JSoup spells "this attribute matches this regex" as `[href~=/item/\d+\.htm]`.
//! Standard CSS gives `~=` the whitespace-word meaning — and rejects most of
//! these patterns outright — so the constraint is lifted out of the selector and
//! re-applied as a filter after selection. The pattern may contain `]` (for
//! example `/[^/]+/`), so brackets are matched with depth counting rather than
//! by scanning to the first `]`.

use crate::source_engine::rule::model::RuleExecutionError;
use crate::source_engine::rule::regex_compat::normalize_java_regex;

/// One compiled `[attr~=pattern]` constraint.
pub(crate) type Compiled = (String, regex::Regex);

/// Compiles the `[attr~=pattern]` constraints of a selector.
///
/// A pattern the regex engine cannot compile is reported instead of silently
/// matching nothing.
pub(crate) fn compile(css: &str) -> Result<Vec<Compiled>, RuleExecutionError> {
    extract(css)
        .into_iter()
        .map(|constraint| {
            let pattern = normalize_java_regex(&constraint.pattern);
            match regex::Regex::new(&pattern) {
                Ok(regex) => Ok((constraint.attr, regex)),
                Err(error) => Err(RuleExecutionError::UnsupportedJsoup(format!(
                    "`[{}~={}]` is not a valid regular expression: {error}",
                    constraint.attr, constraint.pattern
                ))),
            }
        })
        .collect()
}

/// One `[attr~=pattern]` constraint.
#[derive(Debug, PartialEq)]
pub(super) struct RegexAttr {
    pub attr: String,
    pub pattern: String,
}

struct Span {
    start: usize,
    end: usize,
    attr: RegexAttr,
}

/// Rewrites every `[attr~=pattern]` to `[attr]` so `scraper` can compile the
/// rest of the selector.
pub(super) fn strip(css: &str) -> String {
    let spans = spans(css);
    if spans.is_empty() {
        return css.to_owned();
    }
    let mut out = String::with_capacity(css.len());
    let mut cursor = 0;
    for span in spans {
        out.push_str(&css[cursor..span.start]);
        out.push('[');
        out.push_str(&span.attr.attr);
        out.push(']');
        cursor = span.end;
    }
    out.push_str(&css[cursor..]);
    out
}

/// The `[attr~=pattern]` constraints of a selector, in source order.
pub(super) fn extract(css: &str) -> Vec<RegexAttr> {
    spans(css).into_iter().map(|span| span.attr).collect()
}

fn spans(css: &str) -> Vec<Span> {
    let bytes = css.as_bytes();
    let mut found = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\'' | b'"' => index = skip_quoted(css, index),
            b'[' => match matching_bracket(css, index) {
                Some(end) => {
                    if let Some(attr) = parse_inner(&css[index + 1..end]) {
                        found.push(Span {
                            start: index,
                            end: end + 1,
                            attr,
                        });
                    }
                    index = end + 1;
                }
                None => index += 1,
            },
            _ => index += 1,
        }
    }
    found
}

fn skip_quoted(css: &str, start: usize) -> usize {
    let quote = css.as_bytes()[start];
    let mut index = start + 1;
    while index < css.len() {
        match css.as_bytes()[index] {
            b'\\' => index += 2,
            byte if byte == quote => return index + 1,
            _ => index += 1,
        }
    }
    css.len()
}

fn matching_bracket(css: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut index = open;
    while index < css.len() {
        match css.as_bytes()[index] {
            b'\'' | b'"' => {
                index = skip_quoted(css, index);
                continue;
            }
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// `attr ~= pattern`, tolerating JSoup's whitespace around the operator and a
/// quoted pattern.
fn parse_inner(inner: &str) -> Option<RegexAttr> {
    let (attr, rest) = inner.split_once('~')?;
    let pattern = rest.trim_start().strip_prefix('=')?.trim();
    let attr = attr.trim();
    if attr.is_empty() || pattern.is_empty() {
        return None;
    }
    Some(RegexAttr {
        attr: attr.to_owned(),
        pattern: unquote(pattern),
    })
}

fn unquote(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 {
        let quote = bytes[0];
        if (quote == b'\'' || quote == b'"') && bytes[bytes.len() - 1] == quote {
            return value[1..value.len() - 1].to_owned();
        }
    }
    value.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_patterns_that_contain_brackets() {
        let found = extract(r#".listmain a[href~=/[^/]+/\d+\.htm]"#);
        assert_eq!(
            found,
            vec![RegexAttr {
                attr: "href".into(),
                pattern: r"/[^/]+/\d+\.htm".into(),
            }]
        );
    }

    #[test]
    fn tolerates_whitespace_and_alternation() {
        assert_eq!(
            extract("[property~ =category|status|update_time]"),
            vec![RegexAttr {
                attr: "property".into(),
                pattern: "category|status|update_time".into(),
            }]
        );
    }

    #[test]
    fn collects_every_constraint_in_order() {
        let found = extract("[data-src~=\\S][href~=^https?]");
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].attr, "data-src");
        assert_eq!(found[1].pattern, "^https?");
    }

    #[test]
    fn leaves_other_attribute_operators_alone() {
        assert!(extract("[class='a']").is_empty());
        assert!(extract("a[href^=http]").is_empty());
        assert_eq!(strip("a[href^=http]"), "a[href^=http]");
    }

    #[test]
    fn strips_only_the_regex_constraint() {
        assert_eq!(
            strip(r#"a[href~=/[^/]+/\d+\.htm][class=x]"#),
            "a[href][class=x]"
        );
    }
}
