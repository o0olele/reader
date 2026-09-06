use reader_desktop_lib::source_engine::rule::{split_rule, RuleMode};

pub(super) const JSON: &str = r#"{"data":{"list":[{"id":"1","title":"audit","content":"audit"}],"score":1},"list":[{"id":"1","title":"audit","content":"audit"}],"id":"1","title":"audit","content":"audit","score":1}"#;
pub(super) const HTML: &str = "<html><body><div class=\"item\"><a class=\"name\">audit</a></div></body></html>";

/// A source can search an API and read chapters from HTML pages. Respect
/// explicit rule modes and HTML extraction syntax before the source heuristic.
pub(super) fn input_for(raw: &str, json_source: bool) -> &'static str {
    match rule_input(raw) {
        Some(true) => JSON,
        Some(false) => HTML,
        None if json_source => JSON,
        None => HTML,
    }
}

fn rule_input(raw: &str) -> Option<bool> {
    let alternatives = split_rule(raw).ok()?;
    let first = alternatives.first()?.first()?;
    match first.mode {
        RuleMode::Json => Some(true),
        RuleMode::XPath => Some(false),
        RuleMode::Default => {
            let lower = raw.trim_start().to_ascii_lowercase();
            if lower.starts_with("@css:") || lower.starts_with("@@") {
                return Some(false);
            }
            let selector = &first.rule;
            if selector.contains("{$.") || selector.contains("{{") {
                return None;
            }
            if selector.contains('@')
                || selector.starts_with('.')
                || selector.starts_with('#')
                || selector.contains(':')
                || selector.contains(" > ")
                || selector.contains(" + ")
                || selector.contains(" = ")
                || selector.contains("class.")
                || selector.contains("tag.")
                || selector.contains("id.")
            {
                Some(false)
            } else {
                None
            }
        }
        _ => None,
    }
}
