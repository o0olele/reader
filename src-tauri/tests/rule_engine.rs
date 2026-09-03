//! End-to-end coverage of the rule engine on a source written in native
//! legado syntax (private JSoup selectors, XPath, `||`, `##`, `text.`).
//!
//! Every source here deliberately carries CSS projection selectors that match
//! nothing, so any assertion that passes can only have been produced by the
//! engine — the fallback path cannot fake it.

use reader_desktop_lib::domain::source::{
    BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule,
};
use reader_desktop_lib::source_engine::pipeline::{
    parse_book_info, parse_catalog_page, parse_content_page, parse_search,
};

const SOURCE_JSON: &str = include_str!("fixtures/source_c/source.json");

fn raw_rules() -> RawSourceRules {
    let value: serde_json::Value = serde_json::from_str(SOURCE_JSON).expect("valid fixture json");
    let encode = |key: &str| serde_json::to_string(value.get(key).expect(key)).ok();
    RawSourceRules {
        search: encode("ruleSearch"),
        book_info: encode("ruleBookInfo"),
        toc: encode("ruleToc"),
        content: encode("ruleContent"),
        explore: None,
    }
}

/// Projection selectors are valid CSS that matches nothing, so the fallback
/// path yields empty results and cannot be mistaken for the engine.
fn source_c() -> BookSource {
    BookSource {
        id: 21,
        name: "Fixture C".into(),
        base_url: "https://legado.example".into(),
        search_url: "https://legado.example/s?wd={{key}}".into(),
        explore_url: None,
        search_rule: SearchRule {
            item: ".no-such-item".into(),
            title: ".no-such-title".into(),
            author: None,
            cover: None,
            url: "a.no-such-url::attr(href)".into(),
        },
        info_rule: InfoRule::default(),
        catalog_rule: CatalogRule {
            item: ".no-such-chapter".into(),
            title: "a".into(),
            url: "a::attr(href)".into(),
            next_url: None,
        },
        content_selector: ".no-such-content".into(),
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
        raw_rules: raw_rules(),
    }
}

#[test]
fn search_runs_private_jsoup_syntax_with_fallback_and_replacement() {
    let results = parse_search(&source_c(), include_str!("fixtures/source_c/search.html")).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "星海归途");
    // `##作者：##` strips the label the page prefixes onto the author.
    assert_eq!(results[0].author.as_deref(), Some("陈九"));
    assert_eq!(results[0].url, "https://legado.example/book/1201");
    // `tag.img@data-original||tag.img@src` prefers the lazy-load attribute.
    assert_eq!(
        results[0].cover.as_deref(),
        Some("https://legado.example/img/1.jpg")
    );
    // The second row has no `data-original`, so the alternative wins.
    assert_eq!(
        results[1].cover.as_deref(),
        Some("https://cdn.example/img/2.jpg")
    );
    assert_eq!(results[1].url, "https://legado.example/book/1202");
}

#[test]
fn book_info_mixes_jsoup_xpath_and_text_nodes() {
    let info = parse_book_info(&source_c(), include_str!("fixtures/source_c/book.html")).unwrap();

    assert_eq!(info.title.as_deref(), Some("星海归途"));
    // This one is an `@XPath:` rule inside an otherwise JSoup source.
    assert_eq!(info.author.as_deref(), Some("陈九"));
    // An intro spanning several lines of one text node keeps its line breaks
    // but not the markup's indentation.
    assert_eq!(
        info.intro.as_deref(),
        Some("一艘失联七年的勘探船忽然发回信号。\n接收站里只剩下最后一个人。")
    );
    assert_eq!(info.kind.as_deref(), Some("科幻"));
    assert_eq!(info.latest_chapter.as_deref(), Some("第四十二章 灯塔"));
    assert_eq!(
        info.cover.as_deref(),
        Some("https://legado.example/img/1.jpg")
    );
}

#[test]
fn catalog_resolves_relative_urls_and_finds_the_next_page_by_text() {
    let (chapters, next) =
        parse_catalog_page(&source_c(), include_str!("fixtures/source_c/toc.html")).unwrap();

    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0].0, "第一章 信号");
    assert_eq!(chapters[0].1, "https://legado.example/book/1201/c-1");
    assert_eq!(chapters[2].1, "https://legado.example/book/1201/c-3");
    // `text.下一页@href` must pick the next link, not the previous one.
    assert_eq!(next.as_deref(), Some("/book/1201/toc?p=3"));
}

#[test]
fn content_collects_text_nodes_and_skips_scripts() {
    let (content, next) =
        parse_content_page(&source_c(), include_str!("fixtures/source_c/chapter.html")).unwrap();

    assert_eq!(
        content,
        "七年之后，接收机again响了。\n值班的人把耳机按紧了一点。\n窗外没有风。"
    );
    assert!(!content.contains("var ad"));
    assert_eq!(next.as_deref(), Some("/book/1201/c-1b"));
}

#[test]
fn a_source_without_raw_rules_still_uses_the_css_projection() {
    let mut source = source_c();
    source.raw_rules = RawSourceRules::default();
    source.search_rule.item = "ul.result-list li".into();
    source.search_rule.title = ".bookname a".into();
    source.search_rule.url = ".bookname a::attr(href)".into();

    let results = parse_search(&source, include_str!("fixtures/source_c/search.html")).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "星海归途");
}

#[test]
fn an_unexecutable_rule_falls_back_instead_of_failing() {
    let mut source = source_c();
    // A JS rule the engine deliberately refuses; the projection must take over.
    source.raw_rules.search = Some(r#"{"bookList":"<js>result</js>"}"#.into());
    source.search_rule.item = "ul.result-list li".into();
    source.search_rule.title = ".bookname a".into();
    source.search_rule.url = ".bookname a::attr(href)".into();

    let results = parse_search(&source, include_str!("fixtures/source_c/search.html")).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[1].title, "寂静山脉");
}
