use reader_desktop_lib::domain::source::{BookSource, CatalogRule, InfoRule, SearchRule};
use reader_desktop_lib::source_engine::selector::{
    parse_book_info, parse_catalog_page, parse_content_page, parse_search,
};

fn source_a() -> BookSource {
    BookSource {
        id: 11,
        name: "Fixture A".into(),
        base_url: "https://fiction.example".into(),
        search_url: "https://fiction.example/search?q={{key}}".into(),
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
        enabled: true,
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
    assert_eq!(search[0].cover.as_deref(), Some("https://fiction.example/covers/long.jpg"));

    let info = parse_book_info(&source, include_str!("fixtures/source_a/book.html")).unwrap();
    assert_eq!(info.title.as_deref(), Some("The Long Road"));
    assert_eq!(info.author.as_deref(), Some("A. Writer"));
    assert_eq!(info.cover.as_deref(), Some("https://fiction.example/covers/long.jpg"));

    let (chapters, next) =
        parse_catalog_page(&source, include_str!("fixtures/source_a/toc.html")).unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[1].1, "https://fiction.example/books/long-road/chapter-2");
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
    assert_eq!(search[0].cover.as_deref(), Some("https://cdn.example/covers/book-7.webp"));

    let info = parse_book_info(&source, include_str!("fixtures/source_b/book.html")).unwrap();
    assert_eq!(info.title.as_deref(), Some("Seven Signals"));
    assert_eq!(info.intro.as_deref(), Some("A station hears seven impossible signals."));

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
