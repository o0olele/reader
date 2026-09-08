//! The parsing entry point used by the services.

#[path = "pipeline/stages.rs"]
mod stages;

pub use stages::{
    parse_book_info, parse_catalog_page, parse_content_page, parse_explore, parse_search,
    parse_search_response,
};

use crate::{
    domain::source::BookSource,
    error::AppError,
    source_engine::rule::{evaluate, evaluate_first, Extraction, RuleContext},
};

fn strict_engine_value(value: Option<&str>) -> bool {
    matches!(value, Some(value) if value == "1" || value.eq_ignore_ascii_case("true"))
}

fn strict_engine() -> bool {
    strict_engine_value(std::env::var("READER_STRICT_ENGINE").ok().as_deref())
}

fn engine_error(source: &BookSource, rule: &str, error: impl std::fmt::Display) -> AppError {
    tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
    AppError::parse(format!(
        "source `{}` rule `{rule}` is not executable: {error}",
        source.name
    ))
}

fn values_in(
    source: &BookSource,
    rule: &str,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, AppError> {
    match evaluate(rule, input, want, context) {
        Ok(values) => Ok(values),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(Vec::new())
        }
    }
}

fn first_in(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
    context: &mut RuleContext,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    match evaluate_first(rule, input, context) {
        Ok(value) => Ok(value),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(None)
        }
    }
}

fn joined_in(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
    context: &mut RuleContext,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    let text = values_in(source, rule, input, Extraction::Values, context)?.join("\n");
    Ok((!text.trim().is_empty()).then_some(text))
}

#[cfg(test)]
mod tests {
    use super::{parse_search_response, strict_engine_value};
    use crate::domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule};

    #[test]
    fn strict_engine_only_accepts_enabled_values() {
        assert!(!strict_engine_value(None));
        assert!(!strict_engine_value(Some("0")));
        assert!(!strict_engine_value(Some("yes")));
        assert!(strict_engine_value(Some("1")));
        assert!(strict_engine_value(Some("TRUE")));
    }

    #[test]
    fn book_url_pattern_turns_a_detail_page_into_one_search_result() {
        let source = BookSource {
            id: 9,
            name: "direct".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com/search?q={{key}}".into(),
            explore_url: None,
            book_url_pattern: Some(r".*".into()),
            enabled_cookie_jar: true,
            search_rule: SearchRule {
                item: ".missing".into(),
                title: ".missing".into(),
                author: None,
                cover: None,
                url: "a".into(),
            },
            info_rule: InfoRule {
                title: Some("h1".into()),
                author: Some(".author".into()),
                intro: Some(".intro".into()),
                ..Default::default()
            },
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "GET".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            source_group: None,
            custom_order: 0,
            weight: 0,
            enabled_explore: true,
            respond_time: None,
            last_update_time: None,
            raw_rules: RawSourceRules::default(),
        };
        let results = parse_search_response(
            &source,
            r#"<h1>Direct Book</h1><b class="author">Writer</b><p class="intro">Intro</p>"#,
            "https://example.com/book/42",
        )
        .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Direct Book");
        assert_eq!(results[0].author.as_deref(), Some("Writer"));
        assert_eq!(results[0].url, "https://example.com/book/42");
    }
}
