//! Detection of legado rule syntax the CSS-only engine cannot execute.
//!
//! This exists so imports can report "partially supported" instead of silently
//! storing rules that will fail at request time. The full rule engine in
//! ROADMAP Step 2 supersedes this module.

use crate::domain::source::SourceImport;
use crate::source_engine::rule::{split_rule, RuleJoin, RuleMode};
use std::collections::HashSet;

const UNSUPPORTED_RULE_MARKERS: [&str; 7] = ["@XPath:", "@Json:", "$.", "<js>", "&&", "||", "##"];

fn rule_needs_full_engine(rule: &str) -> bool {
    if rule.trim().is_empty() {
        return false;
    }
    let Ok(alternatives) = split_rule(rule) else {
        return true;
    };
    alternatives.len() > 1
        || alternatives.iter().flatten().any(|parsed| {
            parsed.mode != RuleMode::Default
                || parsed.join != RuleJoin::Chain
                || parsed.reverse
                || parsed.replace.is_some()
                || !parsed.put.is_empty()
                || !parsed.get.is_empty()
                || !parsed.templates.is_empty()
                || parsed.rule.contains('@')
        })
}

/// Whether any rule kept by the CSS-compatible importer needs the full engine.
pub fn source_has_unsupported_rules(source: &SourceImport) -> bool {
    [
        source.search_rule.item.as_str(),
        source.search_rule.title.as_str(),
        source.search_rule.author.as_deref().unwrap_or_default(),
        source.search_rule.cover.as_deref().unwrap_or_default(),
        source.search_rule.url.as_str(),
        source.info_rule.title.as_deref().unwrap_or_default(),
        source.info_rule.author.as_deref().unwrap_or_default(),
        source.info_rule.intro.as_deref().unwrap_or_default(),
        source.info_rule.cover.as_deref().unwrap_or_default(),
        source.info_rule.kind.as_deref().unwrap_or_default(),
        source
            .info_rule
            .latest_chapter
            .as_deref()
            .unwrap_or_default(),
        source.catalog_rule.item.as_str(),
        source.catalog_rule.title.as_str(),
        source.catalog_rule.url.as_str(),
        source.catalog_rule.next_url.as_deref().unwrap_or_default(),
        source.content_selector.as_str(),
        source.next_toc_url_selector.as_deref().unwrap_or_default(),
        source
            .next_content_url_selector
            .as_deref()
            .unwrap_or_default(),
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

    #[test]
    fn analyzer_detects_supported_css_and_unsupported_directives() {
        assert!(!rule_needs_full_engine(".book"));
        assert!(!rule_needs_full_engine("@css:.book"));
        assert!(rule_needs_full_engine("@XPath://article"));
        assert!(rule_needs_full_engine(".title&&.author"));
        assert!(rule_needs_full_engine(".body##ads##"));
    }

    #[test]
    fn malformed_rules_are_reported_instead_of_being_accepted() {
        assert!(rule_needs_full_engine(".book["));
    }

    #[test]
    fn optional_rule_fields_are_checked() {
        let mut source = SourceImport {
            name: "demo".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com?q={{key}}".into(),
            explore_url: None,
            search_rule: crate::domain::source::SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a::attr(href)".into(),
            },
            info_rule: Default::default(),
            catalog_rule: crate::domain::source::CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a::attr(href)".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            header: None,
            login_url: None,
            login_method: "GET".into(),
            login_body: None,
            token_path: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            next_toc_url_selector: None,
            next_content_url_selector: None,
            enabled: true,
            raw_rules: Default::default(),
        };
        source.info_rule.intro = Some("@Json:$.intro".into());
        assert!(source_has_unsupported_rules(&source));
    }
}
