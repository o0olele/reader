//! CSS selector execution for the currently supported source-rule subset.

use crate::source::{BookSearchResult, BookSource};
use scraper::{ElementRef, Html, Selector};

fn text(element: ElementRef<'_>) -> Option<String> {
    let value = element.text().collect::<Vec<_>>().join(" ");
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    (!value.is_empty()).then_some(value)
}

fn extract(element: ElementRef<'_>, rule: &str) -> Option<String> {
    let (selector, attribute) = rule
        .split_once("::attr(")
        .and_then(|(selector, rest)| rest.strip_suffix(')').map(|attr| (selector, Some(attr))))
        .unwrap_or((rule, None));
    let node = element
        .select(&Selector::parse(selector.trim()).ok()?)
        .next()?;
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

fn absolute_url(base: &str, value: &str) -> String {
    reqwest::Url::parse(value)
        .map(|url| url.to_string())
        .unwrap_or_else(|_| {
            reqwest::Url::parse(base)
                .ok()
                .and_then(|url| url.join(value).ok())
                .map(|url| url.to_string())
                .unwrap_or_else(|| value.to_owned())
        })
}

pub fn parse_catalog(source: &BookSource, html: &str) -> Result<Vec<(String, String)>, String> {
    let document = Html::parse_document(html);
    let items = Selector::parse(&source.catalog_rule.item)
        .map_err(|error| format!("目录结果选择器无效: {error}"))?;
    Ok(document
        .select(&items)
        .filter_map(|item| {
            Some((
                extract(item, &source.catalog_rule.title)?,
                absolute_url(&source.base_url, &extract(item, &source.catalog_rule.url)?),
            ))
        })
        .collect())
}

pub fn parse_content(source: &BookSource, html: &str) -> Result<String, String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&source.content_selector)
        .map_err(|error| format!("正文选择器无效: {error}"))?;
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

pub fn parse_search(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, String> {
    let document = Html::parse_document(html);
    let items = Selector::parse(&source.search_rule.item)
        .map_err(|error| format!("搜索结果选择器无效: {error}"))?;
    Ok(document
        .select(&items)
        .filter_map(|item| {
            let title = extract(item, &source.search_rule.title)?;
            let url = absolute_url(&source.base_url, &extract(item, &source.search_rule.url)?);
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
                .map(|value| absolute_url(&source.base_url, &value));
            Some(BookSearchResult {
                source_id: source.id,
                source_name: source.name.clone(),
                title,
                author,
                cover,
                url,
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{CatalogRule, InfoRule, SearchRule};

    fn source() -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: String::new(),
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
            },
            content_selector: ".content".into(),
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
            enabled: true,
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
    fn parses_catalog_and_content() {
        let html = r#"<div class="chapter"><a href="/c1">第一章</a></div><article class="content"><p>第一段</p><p>第二段</p></article>"#;
        assert_eq!(
            parse_catalog(&source(), html).unwrap()[0].1,
            "https://example.com/c1"
        );
        assert_eq!(parse_content(&source(), html).unwrap(), "第一段\n第二段");
    }
}
