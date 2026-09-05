//! CSS selector execution for the currently supported source-rule subset.

use crate::domain::source::{BookInfo, BookSearchResult, BookSource};
use crate::source_engine::{import::normalize_rule, url::absolutize};
use scraper::{ElementRef, Html, Selector};

fn text(element: ElementRef<'_>) -> Option<String> {
    let value = element.text().collect::<Vec<_>>().join(" ");
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    (!value.is_empty()).then_some(value)
}

fn extract(element: ElementRef<'_>, rule: &str) -> Option<String> {
    let normalized = normalize_rule(rule);
    let rule = normalized.as_str();
    let (selector, attribute) = rule
        .split_once("::attr(")
        .and_then(|(selector, rest)| rest.strip_suffix(')').map(|attr| (selector, Some(attr))))
        .unwrap_or((rule, None));
    let selector = selector.trim();
    let node = if selector == "*" {
        element
    } else {
        element.select(&Selector::parse(selector).ok()?).next()?
    };
    attribute
        .map(|name| {
            node.value()
                .attr(name.trim())
                .map(str::trim)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| text(node))
        .filter(|value| !value.is_empty())
}

fn first_by_rule(document: &Html, rule: &str) -> Option<String> {
    let normalized = normalize_rule(rule);
    let rule = normalized.as_str();
    let selector = rule.split("::attr(").next()?.trim();
    let selector = Selector::parse(selector).ok()?;
    let node = document.select(&selector).next()?;
    if let Some((_, rest)) = rule.split_once("::attr(") {
        let attr = rest.strip_suffix(')')?.trim();
        node.value().attr(attr).map(str::trim).map(str::to_owned)
    } else {
        text(node)
    }
}

pub fn parse_catalog(source: &BookSource, html: &str) -> Result<Vec<(String, String)>, String> {
    let document = Html::parse_document(html);
    let item_rule = normalize_rule(&source.catalog_rule.item);
    let items =
        Selector::parse(&item_rule).map_err(|error| format!("目录结果选择器无效: {error}"))?;
    Ok(document
        .select(&items)
        .filter_map(|item| {
            Some((
                extract(item, &source.catalog_rule.title)?,
                absolutize(&source.base_url, &extract(item, &source.catalog_rule.url)?),
            ))
        })
        .collect())
}

type CatalogPage = (Vec<(String, String)>, Option<String>);

pub fn parse_catalog_page(source: &BookSource, html: &str) -> Result<CatalogPage, String> {
    let catalog = parse_catalog(source, html)?;
    let next = source.next_toc_url_selector.as_deref().and_then(|rule| {
        let document = Html::parse_document(html);
        first_by_rule(&document, rule)
    });
    Ok((catalog, next))
}

pub fn parse_content(source: &BookSource, html: &str) -> Result<String, String> {
    let document = Html::parse_document(html);
    let content_rule = normalize_rule(&source.content_selector);
    let selector =
        Selector::parse(&content_rule).map_err(|error| format!("正文选择器无效: {error}"))?;
    let content = document
        .select(&selector)
        .next()
        .ok_or("页面中没有找到正文")?
        .text()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    (!content.is_empty())
        .then_some(content)
        .ok_or_else(|| "页面正文为空".into())
}

pub fn parse_content_page(
    source: &BookSource,
    html: &str,
) -> Result<(String, Option<String>), String> {
    let content = parse_content(source, html)?;
    let next = source
        .next_content_url_selector
        .as_deref()
        .and_then(|rule| {
            let document = Html::parse_document(html);
            first_by_rule(&document, rule)
        });
    Ok((content, next))
}

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, String> {
    let document = Html::parse_document(html);
    let read = |rule: Option<&String>| -> Option<String> {
        let rule = rule?;
        first_by_rule(&document, rule)
    };
    Ok(BookInfo {
        title: read(source.info_rule.title.as_ref()),
        author: read(source.info_rule.author.as_ref()),
        intro: read(source.info_rule.intro.as_ref()),
        cover: source.info_rule.cover.as_ref().and_then(|rule| {
            let value = read(Some(rule))?;
            Some(absolutize(&source.base_url, &value))
        }),
        kind: read(source.info_rule.kind.as_ref()),
        latest_chapter: read(source.info_rule.latest_chapter.as_ref()),
    })
}

pub fn parse_search(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, String> {
    let document = Html::parse_document(html);
    // The flat projection is only a compatibility fallback.  It still needs
    // to understand the most common legado item spellings, otherwise a source
    // such as `tr!0||.panel-body > div` turns an otherwise recoverable empty
    // engine result into a hard `Token "!" was not expected` parse error.
    let mut invalid_selector = None;
    for branch in split_alternatives(&source.search_rule.item) {
        let item_rule = normalize_selector_projection(branch);
        if item_rule.is_empty() {
            continue;
        }
        let items = match Selector::parse(&item_rule) {
            Ok(items) => items,
            Err(error) => {
                invalid_selector.get_or_insert(format!("{error}"));
                continue;
            }
        };
        let results = document
            .select(&items)
            .filter_map(|item| {
                let title = extract(item, &source.search_rule.title)?;
                let url = absolutize(&source.base_url, &extract(item, &source.search_rule.url)?);
                let author = source
                    .search_rule
                    .author
                    .as_deref()
                    .and_then(|rule| extract(item, rule));
                let cover = source
                    .search_rule
                    .cover
                    .as_deref()
                    .and_then(|rule| extract(item, rule))
                    .map(|value| absolutize(&source.base_url, &value));
                Some(BookSearchResult {
                    source_id: source.id,
                    source_name: source.name.clone(),
                    title,
                    author,
                    cover,
                    url,
                    intro: None,
                    kind: None,
                    latest_chapter: None,
                    word_count: None,
                })
            })
            .collect::<Vec<_>>();
        if !results.is_empty() {
            return Ok(results);
        }
    }
    match invalid_selector {
        Some(error) => Err(format!("搜索结果选择器无效: {error}")),
        None => Ok(Vec::new()),
    }
}

fn split_alternatives(raw: &str) -> Vec<&str> {
    let mut branches = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    let mut quote = None;
    let bytes = raw.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let character = bytes[index] as char;
        match quote {
            Some(active) if character == active => quote = None,
            Some(_) => {}
            None => match character {
                '\'' | '"' => quote = Some(character),
                '[' | '(' => depth += 1,
                ']' | ')' => depth = (depth - 1).max(0),
                '|' if depth == 0 && bytes.get(index + 1) == Some(&b'|') => {
                    branches.push(raw[start..index].trim());
                    index += 1;
                    start = index + 1;
                }
                _ => {}
            },
        }
        index += 1;
    }
    branches.push(raw[start..].trim());
    branches
}

fn normalize_selector_projection(raw: &str) -> String {
    let normalized = normalize_rule(raw);
    strip_legacy_position(&normalized)
}

/// Removes a trailing legado index/range expression from a CSS projection.
/// The rule engine executes these filters with the correct semantics; the
/// fallback only needs a valid selector and therefore deliberately ignores the
/// position when projecting to scraper CSS.
fn strip_legacy_position(value: &str) -> String {
    let value = value.trim();
    if let Some(content) = value.strip_suffix(']') {
        if let Some(open) = content.rfind('[') {
            let expression = content[open + 1..].trim();
            if is_position_expression(expression) {
                return content[..open].trim_end().to_owned();
            }
        }
    }
    if let Some(bang) = value.rfind('!') {
        let suffix = value[bang + 1..].trim();
        if is_index_list(suffix) {
            return value[..bang].trim_end_matches('.').trim_end().to_owned();
        }
    }
    for (dot, _) in value.match_indices('.').rev() {
        let suffix = value[dot + 1..].trim();
        if is_index_list(suffix) {
            return value[..dot].trim_end().to_owned();
        }
    }
    value.to_owned()
}

fn is_position_expression(value: &str) -> bool {
    let value = value.strip_prefix('!').unwrap_or(value).trim();
    !value.is_empty()
        && value.split(',').all(|part| {
            let part = part.trim();
            if part.contains(':') {
                let pieces = part.split(':').collect::<Vec<_>>();
                (2..=3).contains(&pieces.len())
                    && pieces
                        .iter()
                        .all(|piece| piece.trim().is_empty() || piece.trim().parse::<i32>().is_ok())
            } else {
                part.parse::<i32>().is_ok()
            }
        })
}

fn is_index_list(value: &str) -> bool {
    !value.is_empty()
        && value
            .split(':')
            .all(|part| part.trim().parse::<i32>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{CatalogRule, InfoRule, SearchRule};

    fn source() -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: String::new(),
            explore_url: None,
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: Some(".author".into()),
                cover: Some("img::attr(src)".into()),
                url: "a::attr(href)".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: ".chapter".into(),
                title: "a".into(),
                url: "a::attr(href)".into(),
                next_url: None,
            },
            content_selector: ".content".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            raw_rules: Default::default(),
        }
    }

    #[test]
    fn parses_css_text_and_attributes() {
        let results = parse_search(&source(), r#"<div class="book"><a href="/book/1"><span class="title"> A Book </span></a><i class="author">Jane</i><img src="/cover.jpg"></div>"#).unwrap();
        assert_eq!(results[0].title, "A Book");
        assert_eq!(results[0].url, "https://example.com/book/1");
        assert_eq!(
            results[0].cover.as_deref(),
            Some("https://example.com/cover.jpg")
        );
    }

    #[test]
    fn projects_legacy_item_alternatives_with_exclusions() {
        let mut source = source();
        source.search_rule.item = "tr!0||.panel-body > div".into();
        source.search_rule.title = "a".into();
        source.search_rule.url = "a::attr(href)".into();
        let results = parse_search(
            &source,
            r#"<table><tr><td>head</td></tr><tr><td><a href="/book">Book</a></td></tr></table>"#,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Book");
    }

    #[test]
    fn normalizes_legacy_rules_when_raw_rules_are_missing() {
        let mut source = source();
        source.search_rule.item = "class.book".into();
        source.search_rule.title = "tag.a@text".into();
        source.search_rule.url = "tag.a@href".into();
        assert_eq!(normalize_rule("class.book"), ".book");
        assert_eq!(normalize_rule("tag.a@text"), "a");
        assert_eq!(normalize_rule("tag.a@href"), "a::attr(href)");
        let results = parse_search(
            &source,
            r#"<div class="book"><a href="/book/1">A Book</a></div>"#,
        )
        .unwrap();
        assert_eq!(results[0].title, "A Book");
        assert_eq!(results[0].url, "https://example.com/book/1");
    }

    #[test]
    fn parses_catalog_and_content() {
        let html = r#"<div class="chapter"><a href="/c1">第一章</a></div><article class="content"><p>第一段</p><p>第二段</p></article>"#;
        assert_eq!(
            parse_catalog(&source(), html).unwrap()[0].1,
            "https://example.com/c1"
        );
        assert_eq!(parse_content(&source(), html).unwrap(), "第一段\n第二段");
    }

    #[test]
    fn extracts_text_and_attributes_from_the_current_item() {
        let mut source = source();
        source.catalog_rule = CatalogRule {
            item: ".chapter a".into(),
            title: "*".into(),
            url: "*::attr(href)".into(),
            next_url: None,
        };
        let chapters = parse_catalog(
            &source,
            r#"<ul><li class="chapter"><a href="/c1">第一章</a></li></ul>"#,
        )
        .unwrap();
        assert_eq!(
            chapters,
            vec![("第一章".into(), "https://example.com/c1".into())]
        );
    }

    #[test]
    fn parses_info_and_page_links() {
        let mut source = source();
        source.info_rule = InfoRule {
            title: Some(".title".into()),
            author: Some(".author".into()),
            intro: Some(".intro".into()),
            cover: Some("img::attr(src)".into()),
            kind: None,
            latest_chapter: None,
        };
        source.next_toc_url_selector = Some(".next::attr(href)".into());
        source.next_content_url_selector = Some(".more::attr(href)".into());
        let html = r#"<h1 class="title">Book</h1><span class="author">Author</span><p class="intro">Intro</p><img src="/cover.jpg"><a class="next" href="/toc-2">下一页</a><a class="more" href="/c-2">继续</a><article class="content">Body</article>"#;
        let info = parse_book_info(&source, html).unwrap();
        assert_eq!(info.title.as_deref(), Some("Book"));
        assert_eq!(info.cover.as_deref(), Some("https://example.com/cover.jpg"));
        assert_eq!(
            parse_catalog_page(&source, html).unwrap().1.as_deref(),
            Some("/toc-2")
        );
        assert_eq!(
            parse_content_page(&source, html).unwrap().1.as_deref(),
            Some("/c-2")
        );
    }
}
