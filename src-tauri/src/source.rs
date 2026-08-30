use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRule {
    pub item: String,
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSource {
    pub id: i64,
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    pub search_rule: SearchRule,
    pub info_rule: InfoRule,
    pub catalog_rule: CatalogRule,
    pub content_selector: String,
    pub header: Option<String>,
    pub login_url: Option<String>,
    pub login_method: String,
    pub login_body: Option<String>,
    pub token_path: Option<String>,
    pub access_token: Option<String>,
    pub session_cookie: Option<String>,
    pub session_expires_at: Option<String>,
    pub sign_script: Option<String>,
    pub proxy_url: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoRule {
    pub title: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogRule {
    pub item: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceImport {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    pub search_rule: SearchRule,
    pub info_rule: InfoRule,
    pub catalog_rule: CatalogRule,
    pub content_selector: String,
    pub header: Option<String>,
    pub login_url: Option<String>,
    pub login_method: String,
    pub login_body: Option<String>,
    pub token_path: Option<String>,
    pub sign_script: Option<String>,
    pub proxy_url: Option<String>,
    pub enabled: bool,
}

fn legacy_rule(value: Option<&serde_json::Value>, keys: &[&str]) -> Option<String> {
    value
        .and_then(|object| {
            keys.iter()
                .find_map(|key| object.get(*key).and_then(serde_json::Value::as_str))
        })
        .map(|rule| {
            let rule = rule.split("&&").next().unwrap_or(rule).trim();
            rule.strip_prefix("@css:").unwrap_or(rule).trim().to_owned()
        })
        .filter(|rule| !rule.is_empty())
}

fn rule_value(value: Option<&serde_json::Value>) -> Option<serde_json::Value> {
    let value = value?;
    value
        .as_str()
        .and_then(|text| serde_json::from_str(text).ok())
        .or_else(|| Some(value.clone()))
}

fn header_value(value: Option<&serde_json::Value>) -> Option<String> {
    let value = value?;
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| serde_json::to_string(value).ok())
}

pub fn parse_sources_json(input: &str) -> Result<Vec<SourceImport>, String> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|e| format!("书源 JSON 格式无效: {e}"))?;
    let values = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    let mut sources = Vec::new();
    for value in values {
        let object = value.as_object().ok_or("书源必须是 JSON 对象")?;
        let name = object
            .get("name")
            .or_else(|| object.get("bookSourceName"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .trim()
            .to_owned();
        let base_url = object
            .get("base_url")
            .or_else(|| object.get("bookSourceUrl"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .trim()
            .to_owned();
        let search_url = object
            .get("search_url")
            .or_else(|| object.get("searchUrl"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .trim()
            .replace("<searchKey>", "{{key}}");
        if name.is_empty() || base_url.is_empty() || search_url.is_empty() {
            continue;
        }
        let search = rule_value(
            object
                .get("search_rule")
                .or_else(|| object.get("ruleSearch")),
        );
        let search_rule = SearchRule {
            item: legacy_rule(search.as_ref(), &["item", "bookList"])
                .unwrap_or_else(|| ".book".into()),
            title: legacy_rule(search.as_ref(), &["title", "name"])
                .unwrap_or_else(|| ".title".into()),
            author: legacy_rule(search.as_ref(), &["author"]),
            cover: legacy_rule(search.as_ref(), &["cover", "coverUrl"]).map(|rule| {
                if rule.contains("::attr(") {
                    rule
                } else {
                    format!("{rule}::attr(src)")
                }
            }),
            url: legacy_rule(search.as_ref(), &["url", "bookUrl"])
                .map(|rule| {
                    if rule.contains("::attr(") {
                        rule
                    } else {
                        format!("{rule}::attr(href)")
                    }
                })
                .unwrap_or_else(|| "a::attr(href)".into()),
        };
        let catalog = rule_value(object.get("catalog_rule").or_else(|| object.get("ruleToc")));
        let content = object
            .get("content_selector")
            .or_else(|| object.get("ruleContent"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("body")
            .split("&&")
            .next()
            .unwrap_or("body")
            .trim()
            .to_owned();
        sources.push(SourceImport {
            name,
            base_url,
            search_url,
            search_rule,
            info_rule: InfoRule {
                title: None,
                author: None,
                intro: None,
            },
            catalog_rule: CatalogRule {
                item: legacy_rule(catalog.as_ref(), &["item", "chapterList"])
                    .unwrap_or_else(|| "a".into()),
                title: legacy_rule(catalog.as_ref(), &["title", "chapterName"])
                    .unwrap_or_else(|| "a".into()),
                url: legacy_rule(catalog.as_ref(), &["url", "chapterUrl"])
                    .map(|rule| {
                        if rule.contains("::attr(") {
                            rule
                        } else {
                            format!("{rule}::attr(href)")
                        }
                    })
                    .unwrap_or_else(|| "a::attr(href)".into()),
            },
            content_selector: content,
            header: header_value(object.get("header").or_else(|| object.get("headers"))),
            login_url: object
                .get("login_url")
                .or_else(|| object.get("loginUrl"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            login_method: object
                .get("login_method")
                .or_else(|| object.get("loginMethod"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("POST")
                .to_uppercase(),
            login_body: object
                .get("login_body")
                .or_else(|| object.get("loginBody"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            token_path: object
                .get("token_path")
                .or_else(|| object.get("tokenPath"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            sign_script: object
                .get("sign_script")
                .or_else(|| object.get("signScript"))
                .or_else(|| object.get("js"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            proxy_url: object
                .get("proxy_url")
                .or_else(|| object.get("proxyUrl"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            enabled: object
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true),
        });
    }
    if sources.is_empty() {
        Err("JSON 中没有可导入的书源".into())
    } else {
        Ok(sources)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSearchResult {
    pub source_id: i64,
    pub source_name: String,
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub url: String,
}

fn extract(element: ElementRef<'_>, rule: &str) -> Option<String> {
    let (selector_text, attribute) = rule
        .split_once("::attr(")
        .and_then(|(selector, rest)| rest.strip_suffix(')').map(|attr| (selector, Some(attr))))
        .unwrap_or((rule, None));
    let selector = Selector::parse(selector_text.trim()).ok()?;
    let node = element.select(&selector).next()?;
    if let Some(attribute) = attribute {
        node.value()
            .attr(attribute.trim())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    } else {
        let value = node.text().collect::<Vec<_>>().join(" ");
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        (!value.is_empty()).then_some(value)
    }
}

fn absolute_url(base_url: &str, value: &str) -> String {
    reqwest::Url::parse(value)
        .map(|url| url.to_string())
        .unwrap_or_else(|_| {
            reqwest::Url::parse(base_url)
                .ok()
                .and_then(|base| base.join(value).ok())
                .map(|url| url.to_string())
                .unwrap_or_else(|| value.to_owned())
        })
}

#[allow(dead_code)]
fn extract_document(document: &Html, rule: &str) -> Option<String> {
    let selector = Selector::parse(rule).ok()?;
    let node = document.select(&selector).next()?;
    let value = node.text().collect::<Vec<_>>().join(" ");
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    (!value.is_empty()).then_some(value)
}

#[allow(dead_code)]
pub fn parse_book_info(
    source: &BookSource,
    html: &str,
    fallback_title: &str,
) -> (String, Option<String>, Option<String>) {
    let document = Html::parse_document(html);
    let title = source
        .info_rule
        .title
        .as_deref()
        .and_then(|rule| extract_document(&document, rule))
        .unwrap_or_else(|| fallback_title.to_owned());
    let author = source
        .info_rule
        .author
        .as_deref()
        .and_then(|rule| extract_document(&document, rule));
    let intro = source
        .info_rule
        .intro
        .as_deref()
        .and_then(|rule| extract_document(&document, rule));
    (title, author, intro)
}

pub fn parse_catalog(source: &BookSource, html: &str) -> Result<Vec<(String, String)>, String> {
    let document = Html::parse_document(html);
    let item_selector = Selector::parse(&source.catalog_rule.item)
        .map_err(|e| format!("目录结果选择器无效: {e}"))?;
    let mut chapters = Vec::new();
    for item in document.select(&item_selector) {
        let Some(title) = extract(item, &source.catalog_rule.title) else {
            continue;
        };
        let Some(url) = extract(item, &source.catalog_rule.url) else {
            continue;
        };
        chapters.push((title, absolute_url(&source.base_url, &url)));
    }
    Ok(chapters)
}

pub fn parse_content(source: &BookSource, html: &str) -> Result<String, String> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse(&source.content_selector).map_err(|e| format!("正文选择器无效: {e}"))?;
    let node = document
        .select(&selector)
        .next()
        .ok_or("页面中没有找到正文")?;
    let content = node.text().collect::<Vec<_>>().join("\n");
    let content = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    if content.is_empty() {
        Err("页面正文为空".into())
    } else {
        Ok(content)
    }
}

pub fn parse_search(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, String> {
    let document = Html::parse_document(html);
    let item_selector = Selector::parse(&source.search_rule.item)
        .map_err(|e| format!("搜索结果选择器无效: {e}"))?;
    let mut results = Vec::new();
    for item in document.select(&item_selector) {
        let Some(title) = extract(item, &source.search_rule.title) else {
            continue;
        };
        let url = extract(item, &source.search_rule.url)
            .map(|value| absolute_url(&source.base_url, &value));
        let Some(url) = url else { continue };
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
        results.push(BookSearchResult {
            source_id: source.id,
            source_name: source.name.clone(),
            title,
            author,
            cover,
            url,
        });
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_css_text_and_attributes() {
        let source = BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: "".into(),
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
        };
        let results = parse_search(&source, r#"<div class="book"><a href="/book/1"><span class="title"> A Book </span></a><i class="author">Jane</i><img src="/cover.jpg"></div>"#).unwrap();
        assert_eq!(results[0].title, "A Book");
        assert_eq!(results[0].url, "https://example.com/book/1");
        assert_eq!(
            results[0].cover.as_deref(),
            Some("https://example.com/cover.jpg")
        );
    }

    #[test]
    fn parses_catalog_and_content() {
        let source = BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: "".into(),
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
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
        };
        let html = r#"<div class="chapter"><a href="/c1">第一章</a></div><article class="content"><p>第一段</p><p>第二段</p></article>"#;
        assert_eq!(
            parse_catalog(&source, html).unwrap()[0].1,
            "https://example.com/c1"
        );
        assert_eq!(parse_content(&source, html).unwrap(), "第一段\n第二段");
    }

    #[test]
    fn imports_legacy_legado_source() {
        let json = r#"[{"bookSourceName":"Demo","bookSourceUrl":"https://example.com","searchUrl":"https://example.com/s?q={{key}}","ruleSearch":{"bookList":".book","name":".name","author":".author","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;
        let sources = parse_sources_json(json).unwrap();
        assert_eq!(sources[0].name, "Demo");
        assert_eq!(sources[0].search_rule.url, "a::attr(href)");
        assert_eq!(sources[0].catalog_rule.item, ".chapter");
    }
}
