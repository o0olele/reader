//! Detection of legado rule syntax the CSS-only engine cannot execute.
//!
//! This exists so imports can report "partially supported" instead of silently
//! storing rules that will fail at request time. The full rule engine in
//! ROADMAP Step 2 supersedes this module.

use crate::domain::source::SourceImport;
use std::collections::HashSet;

const UNSUPPORTED_RULE_MARKERS: [&str; 7] = ["@XPath:", "@Json:", "$.", "<js>", "&&", "||", "##"];

fn rule_needs_full_engine(rule: &str) -> bool {
    rule.starts_with("@Json:")
        || rule.starts_with("$.")
        || ["@XPath:", "<js>", "&&", "||", "##"]
            .iter()
            .any(|marker| rule.contains(marker))
}

/// Whether any rule kept by the CSS-compatible importer needs the full engine.
pub fn source_has_unsupported_rules(source: &SourceImport) -> bool {
    [
        source.search_rule.item.as_str(),
        source.search_rule.title.as_str(),
        source.search_rule.url.as_str(),
        source.catalog_rule.item.as_str(),
        source.catalog_rule.title.as_str(),
        source.catalog_rule.url.as_str(),
        source.content_selector.as_str(),
    ]
    .iter()
    .any(|rule| rule_needs_full_engine(rule))
}

/// Names of sources whose *raw* JSON carries syntax the importer drops.
///
/// The importer truncates at `&&` and strips `@css:`, so rules that are lost
/// before [`source_has_unsupported_rules`] ever sees them are caught here.
pub fn raw_unsupported_source_names(input: &str) -> HashSet<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(input) else {
        return HashSet::new();
    };
    let values = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    values
        .into_iter()
        .filter_map(|value| {
            let object = value.as_object()?;
            let name = object
                .get("name")
                .or_else(|| object.get("bookSourceName"))?
                .as_str()?
                .trim()
                .to_owned();
            let encoded = serde_json::to_string(object).ok()?;
            UNSUPPORTED_RULE_MARKERS
                .iter()
                .any(|marker| encoded.contains(marker))
                .then_some(name)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legado_xpath_is_reported() {
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article"}}]"#;
        assert!(raw_unsupported_source_names(input).contains("XPath source"));
    }

    #[test]
    fn a_dollar_sign_inside_a_css_rule_is_not_jsonpath() {
        assert!(!rule_needs_full_engine("a[href$=.html]"));
        assert!(rule_needs_full_engine("$.data.books"));
    }
}
