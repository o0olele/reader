use reader_desktop_lib::source_engine::rule::{split_rule, RuleMode};

/// The same object shape is reachable as `$.list[0]`, `$.data.list[0]` and
/// `$.data.books[0]`, because a deterministic payload cannot know which
/// containers a given source nests its results under.
pub(super) const JSON: &str = r#"{"data":{"list":[{"id":"1","name":"audit","title":"audit","content":"audit","author":"audit","coverUrl":"https://example.test/cover.jpg","bookUrl":"https://example.test/book/1","kind":"audit","intro":"audit","lastChapter":"audit","wordCount":"1","score":1}],"books":[{"id":"1","name":"audit","title":"audit","content":"audit","author":"audit","coverUrl":"https://example.test/cover.jpg","bookUrl":"https://example.test/book/1","kind":"audit","intro":"audit","lastChapter":"audit","wordCount":"1","score":1}],"info":{"sound":{"intro":"audit"}},"score":1},"list":[{"id":"1","name":"audit","title":"audit","content":"audit","author":"audit","coverUrl":"https://example.test/cover.jpg","bookUrl":"https://example.test/book/1","kind":"audit","intro":"audit","lastChapter":"audit","wordCount":"1","score":1}],"id":"1","name":"audit","title":"audit","content":"audit","author":"audit","coverUrl":"https://example.test/cover.jpg","bookUrl":"https://example.test/book/1","kind":"audit","intro":"audit","lastChapter":"audit","wordCount":"1","score":1}"#;
pub(super) const HTML: &str =
    "<html><body><div class=\"item\"><a class=\"name\">audit</a></div></body></html>";

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

/// Candidate inputs a rule is dry-run against.
///
/// The audit's job is to prove a rule *can* execute. A deterministic dummy
/// document can never contain every field a source expects, so judging a rule
/// on a single input conflates "the engine cannot run this rule" with "this
/// dummy payload has no such key" — the failure attribution the roadmap warns
/// about. A rule is only counted as failing when no candidate input lets it
/// run. Genuine engine gaps (invalid CSS, undeclared JavaScript variables,
/// unclosed JS) fail on every candidate and stay reported.
pub(super) fn inputs_for(raw: &str, json_source: bool) -> Vec<&'static str> {
    let preferred = input_for(raw, json_source);
    let other = if preferred == JSON { HTML } else { JSON };
    vec![preferred, other]
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
