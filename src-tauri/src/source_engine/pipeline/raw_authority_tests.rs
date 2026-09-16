//! Deliberately matching flat selectors must never override a configured stage.

use crate::{
    domain::source::{BookSource, RawSourceRules},
    source_engine::{
        import::parse_sources_json,
        pipeline::{parse_book_info, parse_catalog_page, parse_content_page, parse_search},
    },
};

const HTML: &str = r#"<div class="book"><a href="/book">Unrelated book</a></div>
    <h1>Unrelated title</h1><div class="chapter"><a href="/chapter">Unrelated chapter</a></div>
    <article>Unrelated body</article><a class="next" href="/next">Next</a>"#;

fn source() -> BookSource {
    let imported = parse_sources_json(
        r#"{
        "name":"authority", "base_url":"https://example.com",
        "search_rule":{"item":".book","title":"a","url":"a@href"},
        "info_rule":{"title":"h1"},
        "catalog_rule":{"item":".chapter","title":"a","url":"a@href"},
        "content_selector":"article"
    }"#,
    )
    .unwrap();
    let mut source = BookSource::from_import(&imported[0]);
    source.raw_rules = RawSourceRules::default();
    source
}

#[test]
fn absent_raw_stages_keep_legacy_sources_readable() {
    let source = source();
    assert_eq!(
        parse_search(&source, HTML).unwrap()[0].title,
        "Unrelated book"
    );
    assert_eq!(
        parse_book_info(&source, HTML).unwrap().title.as_deref(),
        Some("Unrelated title")
    );
    assert_eq!(
        parse_catalog_page(&source, HTML).unwrap().0[0].0,
        "Unrelated chapter"
    );
    assert_eq!(
        parse_content_page(&source, HTML).unwrap().0,
        "Unrelated body"
    );
}

#[test]
fn raw_search_no_match_does_not_resurrect_flat_matches() {
    let mut source = source();
    source.raw_rules.search =
        Some(r#"{"bookList":".missing","name":"a@text","bookUrl":"a@href"}"#.into());
    assert!(parse_search(&source, HTML).unwrap().is_empty());
    // Even when the list matches, rejected entries must not be retried as CSS.
    source.raw_rules.search =
        Some(r#"{"bookList":".book","name":".missing@text","bookUrl":"a@href"}"#.into());
    assert!(parse_search(&source, HTML).unwrap().is_empty());
}

#[test]
fn raw_catalog_no_match_preserves_only_its_own_next_page() {
    let mut source = source();
    source.raw_rules.toc = Some(r#"{"chapterList":".missing","chapterName":"a@text","chapterUrl":"a@href","nextTocUrl":".next@href"}"#.into());
    let (chapters, next) = parse_catalog_page(&source, HTML).unwrap();
    assert!(chapters.is_empty());
    assert_eq!(next.as_deref(), Some("/next"));
    source.raw_rules.toc = Some(
        r#"{"chapterList":".chapter","chapterName":".missing@text","chapterUrl":"a@href"}"#.into(),
    );
    assert!(parse_catalog_page(&source, HTML).unwrap().0.is_empty());
}

#[test]
fn raw_info_empty_result_does_not_pick_up_unrelated_title() {
    let mut source = source();
    for rule in [r#"{"name":".missing@text"}"#, "{}"] {
        source.raw_rules.book_info = Some(rule.into());
        let info = parse_book_info(&source, HTML).unwrap();
        assert!(info.title.is_none());
        assert!(info.author.is_none());
        assert!(info.intro.is_none());
        assert!(info.cover.is_none());
        assert!(info.kind.is_none());
        assert!(info.latest_chapter.is_none());
    }
}

#[test]
fn raw_content_empty_result_is_an_error_instead_of_unrelated_body() {
    let mut source = source();
    for rule in [r#"{"content":".missing@text"}"#, r#"{"content":""}"#] {
        source.raw_rules.content = Some(rule.into());
        assert!(parse_content_page(&source, HTML)
            .unwrap_err()
            .to_string()
            .contains("页面正文为空"));
    }
}

#[test]
fn malformed_raw_stages_are_reported_instead_of_using_flat_columns() {
    let mut source = source();
    for raw in [
        "not json",
        "[]",
        "42",
        r#"{"bookList":42,"name":42,"chapterList":42,"content":42}"#,
    ] {
        source.raw_rules = RawSourceRules {
            search: Some(raw.into()),
            book_info: Some(raw.into()),
            toc: Some(raw.into()),
            content: Some(raw.into()),
            explore: None,
        };
        assert!(parse_search(&source, HTML)
            .unwrap_err()
            .to_string()
            .contains("ruleSearch"));
        assert!(parse_book_info(&source, HTML)
            .unwrap_err()
            .to_string()
            .contains("ruleBookInfo"));
        assert!(parse_catalog_page(&source, HTML)
            .unwrap_err()
            .to_string()
            .contains("ruleToc"));
        assert!(parse_content_page(&source, HTML)
            .unwrap_err()
            .to_string()
            .contains("ruleContent"));
    }
}

#[test]
fn incomplete_raw_stages_do_not_activate_legacy_defaults() {
    let mut source = source();
    source.raw_rules.search = Some("{}".into());
    source.raw_rules.toc = Some("{}".into());
    source.raw_rules.content = Some("{}".into());
    assert!(parse_search(&source, HTML)
        .unwrap_err()
        .to_string()
        .contains("bookList"));
    assert!(parse_catalog_page(&source, HTML)
        .unwrap_err()
        .to_string()
        .contains("chapterList"));
    assert!(parse_content_page(&source, HTML)
        .unwrap_err()
        .to_string()
        .contains("content"));
}

#[test]
fn unrelated_raw_stage_does_not_hide_invalid_legacy_search() {
    let mut source = source();
    source.raw_rules.content = Some(r#""article@text""#.into());
    source.search_rule.item = "[".into();
    assert!(parse_search(&source, HTML).is_err());
}

#[test]
fn null_and_blank_raw_stages_keep_the_absent_stage_semantics() {
    let mut source = source();
    for raw in ["null", "  "] {
        source.raw_rules = RawSourceRules {
            search: Some(raw.into()),
            book_info: Some(raw.into()),
            toc: Some(raw.into()),
            content: Some(raw.into()),
            explore: None,
        };
        assert_eq!(parse_search(&source, HTML).unwrap().len(), 1);
        assert!(parse_book_info(&source, HTML).unwrap().title.is_some());
        assert_eq!(parse_catalog_page(&source, HTML).unwrap().0.len(), 1);
        assert_eq!(
            parse_content_page(&source, HTML).unwrap().0,
            "Unrelated body"
        );
    }
}

#[test]
fn bare_content_rule_remains_supported() {
    let mut source = source();
    source.content_selector = ".missing".into();
    source.raw_rules.content = Some(r#""article@text""#.into());
    assert_eq!(
        parse_content_page(&source, HTML).unwrap().0,
        "Unrelated body"
    );
}
