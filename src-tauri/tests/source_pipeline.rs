use reader_desktop_lib::domain::source::{
    BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule,
};
use reader_desktop_lib::source_engine::selector::{
    parse_book_info, parse_catalog_page, parse_content_page, parse_search,
};

fn source_a() -> BookSource {
    BookSource {
        id: 11,
        name: "Fixture A".into(),
        base_url: "https://fiction.example".into(),
        search_url: "https://fiction.example/search?q={{key}}".into(),
        explore_url: None,
        search_rule: SearchRule {
            item: "article.result".into(),
            title: "h2 a".into(),
            author: Some(".author".into()),
            cover: Some("img::attr(data-src)".into()),
            url: "h2 a::attr(href)".into(),
        },
        info_rule: InfoRule {
            title: Some("h1".into()),
            author: Some(".meta .author".into()),
            intro: Some(".summary".into()),
            cover: Some(".cover::attr(src)".into()),
            kind: Some(".meta .kind".into()),
            latest_chapter: Some(".latest a".into()),
        },
        catalog_rule: CatalogRule {
            item: "ol.chapters li".into(),
            title: "a".into(),
            url: "a::attr(href)".into(),
            next_url: None,
        },
        content_selector: "article.chapter".into(),
        next_toc_url_selector: Some("a.next::attr(href)".into()),
        next_content_url_selector: Some("a.more::attr(href)".into()),
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
        raw_rules: RawSourceRules::default(),
    }
}

fn source_b() -> BookSource {
    let mut source = source_a();
    source.id = 12;
    source.name = "Fixture B".into();
    source.base_url = "https://mirror.example/novels/".into();
    source.search_rule = SearchRule {
        item: "div[data-book]".into(),
        title: ".name".into(),
        author: Some("[data-author]".into()),
        cover: Some("picture img::attr(src)".into()),
        url: "a.detail::attr(href)".into(),
    };
    source.info_rule = InfoRule {
        title: Some(".book-title".into()),
        author: Some("[data-role='author']".into()),
        intro: Some("#description".into()),
        cover: Some("meta[property='og:image']::attr(content)".into()),
        kind: None,
        latest_chapter: Some(".recent a".into()),
    };
    source.catalog_rule = CatalogRule {
        item: "section.toc div.chapter".into(),
        title: ".label".into(),
        url: "a::attr(href)".into(),
        next_url: None,
    };
    source.content_selector = "div#正文".into();
    source.next_toc_url_selector = Some("link[rel='next']::attr(href)".into());
    source.next_content_url_selector = Some("a[data-next]::attr(href)".into());
    source
}

#[test]
fn source_a_pipeline_parses_all_stages_from_fixtures() {
    let source = source_a();
    let search = parse_search(&source, include_str!("fixtures/source_a/search.html")).unwrap();
    assert_eq!(search.len(), 2);
    assert_eq!(search[0].title, "The Long Road");
    assert_eq!(search[0].url, "https://fiction.example/books/long-road");
    assert_eq!(
        search[0].cover.as_deref(),
        Some("https://fiction.example/covers/long.jpg")
    );

    let info = parse_book_info(&source, include_str!("fixtures/source_a/book.html")).unwrap();
    assert_eq!(info.title.as_deref(), Some("The Long Road"));
    assert_eq!(info.author.as_deref(), Some("A. Writer"));
    assert_eq!(
        info.cover.as_deref(),
        Some("https://fiction.example/covers/long.jpg")
    );

    let (chapters, next) =
        parse_catalog_page(&source, include_str!("fixtures/source_a/toc.html")).unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(
        chapters[1].1,
        "https://fiction.example/books/long-road/chapter-2"
    );
    assert_eq!(next.as_deref(), Some("/books/long-road/toc?page=2"));

    let (content, next) =
        parse_content_page(&source, include_str!("fixtures/source_a/chapter.html")).unwrap();
    assert_eq!(content, "The rain stopped.\nMara opened the gate.");
    assert_eq!(next.as_deref(), Some("/books/long-road/chapter-2"));
}

#[test]
fn source_b_pipeline_handles_relative_and_absolute_markup() {
    let source = source_b();
    let search = parse_search(&source, include_str!("fixtures/source_b/search.html")).unwrap();
    assert_eq!(search.len(), 1);
    assert_eq!(search[0].url, "https://mirror.example/novels/book-7");
    assert_eq!(
        search[0].cover.as_deref(),
        Some("https://cdn.example/covers/book-7.webp")
    );

    let info = parse_book_info(&source, include_str!("fixtures/source_b/book.html")).unwrap();
    assert_eq!(info.title.as_deref(), Some("Seven Signals"));
    assert_eq!(
        info.intro.as_deref(),
        Some("A station hears seven impossible signals.")
    );

    let (chapters, next) =
        parse_catalog_page(&source, include_str!("fixtures/source_b/toc.html")).unwrap();
    assert_eq!(chapters[0].0, "Signal One");
    assert_eq!(chapters[0].1, "https://mirror.example/novels/chapter-1");
    assert_eq!(next.as_deref(), Some("toc?page=2"));

    let (content, next) =
        parse_content_page(&source, include_str!("fixtures/source_b/chapter.html")).unwrap();
    assert_eq!(content, "The receiver clicked.\nSeven lights answered.");
    assert_eq!(next.as_deref(), Some("/novels/chapter-2"));
}

/// Adopting the rule engine must not change what a working source produces.
///
/// `source_a` is described twice over the same markup — once as flat CSS
/// selectors, once as the legado rules in `source_a/source.json` — and both
/// descriptions have to agree stage by stage.
#[test]
fn the_engine_and_the_css_projection_agree_on_the_same_source() {
    use reader_desktop_lib::source_engine::pipeline;

    let css = source_a();
    let mut engine = source_a();
    let rules: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/source_a/source.json")).unwrap();
    let encode = |key: &str| serde_json::to_string(rules.get(key).expect(key)).ok();
    engine.raw_rules = RawSourceRules {
        search: encode("ruleSearch"),
        book_info: encode("ruleBookInfo"),
        toc: encode("ruleToc"),
        content: encode("ruleContent"),
        explore: None,
    };

    let search_html = include_str!("fixtures/source_a/search.html");
    let from_css = parse_search(&css, search_html).unwrap();
    let from_engine = pipeline::parse_search(&engine, search_html).unwrap();
    assert_eq!(from_engine.len(), from_css.len());
    for (engine_hit, css_hit) in from_engine.iter().zip(&from_css) {
        assert_eq!(engine_hit.title, css_hit.title);
        assert_eq!(engine_hit.author, css_hit.author);
        assert_eq!(engine_hit.url, css_hit.url);
        assert_eq!(engine_hit.cover, css_hit.cover);
    }

    let book_html = include_str!("fixtures/source_a/book.html");
    let css_info = parse_book_info(&css, book_html).unwrap();
    let engine_info = pipeline::parse_book_info(&engine, book_html).unwrap();
    assert_eq!(engine_info.title, css_info.title);
    assert_eq!(engine_info.author, css_info.author);
    assert_eq!(engine_info.intro, css_info.intro);
    assert_eq!(engine_info.cover, css_info.cover);
    assert_eq!(engine_info.kind, css_info.kind);
    assert_eq!(engine_info.latest_chapter, css_info.latest_chapter);

    let toc_html = include_str!("fixtures/source_a/toc.html");
    assert_eq!(
        pipeline::parse_catalog_page(&engine, toc_html).unwrap(),
        parse_catalog_page(&css, toc_html).unwrap()
    );

    let chapter_html = include_str!("fixtures/source_a/chapter.html");
    assert_eq!(
        pipeline::parse_content_page(&engine, chapter_html).unwrap(),
        parse_content_page(&css, chapter_html).unwrap()
    );
}
