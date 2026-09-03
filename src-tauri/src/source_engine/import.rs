//! Legado source import normalization for the CSS-compatible subset.
//!
//! The flat selector fields produced here are a lossy projection kept as a
//! fallback; the untouched legado rule objects travel alongside them in
//! [`SourceImport::raw_rules`] and are what the rule engine actually executes.

use crate::domain::source::{CatalogRule, InfoRule, RawSourceRules, SearchRule, SourceImport};

fn rule(value: Option<&serde_json::Value>, keys: &[&str]) -> Option<String> {
    value
        .and_then(|object| keys.iter().find_map(|key| object.get(*key)?.as_str()))
        .map(normalize_rule)
        .filter(|value| !value.is_empty())
}

/// Normalize the selector spellings used by Legado's JSON exports.
///
/// Besides `@css:` Legado commonly emits `css:` and uses `@href`/`@src`
/// suffixes instead of the explicit `::attr(...)` form used internally.
fn normalize_rule(value: &str) -> String {
    let value = value.split("&&").next().unwrap_or(value).trim();
    let value = value
        .strip_prefix("@css:")
        .or_else(|| value.strip_prefix("css:"))
        .map(str::trim)
        .unwrap_or(value);
    if value.eq_ignore_ascii_case("text") || value.eq_ignore_ascii_case("@text") {
        return "*".into();
    }
    if let Some(attribute) = value.strip_prefix('@') {
        if !attribute.is_empty()
            && attribute
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return format!("*::attr({attribute})");
        }
    }
    if !value.contains("::attr(") {
        if let Some((selector, attribute)) = value.rsplit_once('@') {
            if !selector.is_empty()
                && !attribute.is_empty()
                && attribute
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                return format!("{}::attr({attribute})", selector.trim());
            }
        }
    }
    value.to_owned()
}
fn json_value(value: Option<&serde_json::Value>) -> Option<serde_json::Value> {
    let value = value?;
    value
        .as_str()
        .and_then(|text| serde_json::from_str(text).ok())
        .or_else(|| Some(value.clone()))
}
fn field(
    object: &serde_json::Map<String, serde_json::Value>,
    primary: &str,
    legacy: &str,
) -> Option<String> {
    object
        .get(primary)
        .or_else(|| object.get(legacy))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}
/// Re-encodes a legado rule object verbatim, so the engine can read it back
/// without inheriting any of the normalization applied above.
fn raw_rule(object: &serde_json::Map<String, serde_json::Value>, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| object.get(*key))
        .filter(|value| !value.is_null())
        .and_then(|value| serde_json::to_string(value).ok())
}

fn attr(rule: String, name: &str) -> String {
    if rule.contains("::attr(") {
        rule
    } else {
        format!("{rule}::attr({name})")
    }
}

pub fn parse_sources_json(input: &str) -> Result<Vec<SourceImport>, String> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|error| format!("书源 JSON 格式无效: {error}"))?;
    let values = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    let mut sources = Vec::new();
    for value in values {
        let object = value.as_object().ok_or("书源必须是 JSON 对象")?;
        let name = field(object, "name", "bookSourceName").unwrap_or_default();
        let base_url = field(object, "base_url", "bookSourceUrl").unwrap_or_default();
        let search_url = field(object, "search_url", "searchUrl")
            .unwrap_or_else(|| base_url.clone())
            .replace("<searchKey>", "{{key}}");
        if name.is_empty() || base_url.is_empty() {
            continue;
        }
        let search = json_value(
            object
                .get("search_rule")
                .or_else(|| object.get("ruleSearch")),
        );
        let catalog = json_value(object.get("catalog_rule").or_else(|| object.get("ruleToc")));
        let info = json_value(
            object
                .get("info_rule")
                .or_else(|| object.get("ruleBookInfo")),
        );
        let content = json_value(
            object
                .get("content_rule")
                .or_else(|| object.get("ruleContent")),
        );
        let content_selector = content
            .as_ref()
            .and_then(|value| value.as_str().map(normalize_rule))
            .or_else(|| rule(content.as_ref(), &["content", "selector", "main"]))
            .or_else(|| field(object, "content_selector", "ruleContent"))
            .unwrap_or_else(|| "body".into())
            .split("&&")
            .next()
            .unwrap_or("body")
            .trim()
            .into();
        sources.push(SourceImport {
            name,
            base_url,
            search_url,
            search_rule: SearchRule {
                item: rule(search.as_ref(), &["item", "bookList", "list"])
                    .unwrap_or_else(|| ".book".into()),
                title: rule(search.as_ref(), &["title", "name"]).unwrap_or_else(|| ".title".into()),
                author: rule(search.as_ref(), &["author"]),
                cover: rule(search.as_ref(), &["cover", "coverUrl"])
                    .map(|value| attr(value, "src")),
                url: rule(search.as_ref(), &["url", "bookUrl", "detail"])
                    .map(|value| attr(value, "href"))
                    .unwrap_or_else(|| "a::attr(href)".into()),
            },
            info_rule: InfoRule {
                title: rule(info.as_ref(), &["title", "name"]),
                author: rule(info.as_ref(), &["author"]),
                intro: rule(info.as_ref(), &["intro"]),
                cover: rule(info.as_ref(), &["coverUrl", "cover"]).map(|value| attr(value, "src")),
                kind: rule(info.as_ref(), &["kind"]),
                latest_chapter: rule(info.as_ref(), &["lastChapter", "latestChapter"]),
            },
            catalog_rule: CatalogRule {
                item: rule(catalog.as_ref(), &["item", "chapterList", "list"])
                    .unwrap_or_else(|| "a".into()),
                title: rule(catalog.as_ref(), &["title", "chapterName", "name"])
                    .unwrap_or_else(|| "a".into()),
                url: rule(catalog.as_ref(), &["url", "chapterUrl"])
                    .map(|value| attr(value, "href"))
                    .unwrap_or_else(|| "a::attr(href)".into()),
                next_url: rule(catalog.as_ref(), &["nextTocUrl", "nextUrl", "next"])
                    .map(|value| attr(value, "href")),
            },
            content_selector,
            header: object
                .get("header")
                .or_else(|| object.get("headers"))
                .and_then(|value| {
                    value
                        .as_str()
                        .map(str::to_owned)
                        .or_else(|| serde_json::to_string(value).ok())
                }),
            login_url: field(object, "login_url", "loginUrl"),
            login_method: field(object, "login_method", "loginMethod")
                .unwrap_or_else(|| "POST".into())
                .to_uppercase(),
            login_body: field(object, "login_body", "loginBody"),
            token_path: field(object, "token_path", "tokenPath"),
            sign_script: field(object, "sign_script", "signScript")
                .or_else(|| field(object, "js", "js")),
            proxy_url: field(object, "proxy_url", "proxyUrl"),
            concurrent_rate: field(object, "concurrent_rate", "concurrentRate"),
            next_toc_url_selector: rule(catalog.as_ref(), &["nextTocUrl", "nextUrl", "next"])
                .map(|value| attr(value, "href")),
            next_content_url_selector: rule(
                content.as_ref(),
                &["nextContentUrl", "next_url", "nextUrl", "next"],
            )
            .map(|value| attr(value, "href")),
            enabled: object
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true),
            raw_rules: RawSourceRules {
                search: raw_rule(object, &["search_rule", "ruleSearch"]),
                book_info: raw_rule(object, &["info_rule", "ruleBookInfo"]),
                toc: raw_rule(object, &["catalog_rule", "ruleToc"]),
                content: raw_rule(object, &["content_rule", "ruleContent"]),
            },
        });
    }
    if sources.is_empty() {
        Err("JSON 中没有可导入的书源".into())
    } else {
        Ok(sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_legacy_legado_source() {
        let input = r#"[{"bookSourceName":"Demo","bookSourceUrl":"https://example.com","searchUrl":"https://example.com/s?q={{key}}","concurrentRate":"5/1000","ruleSearch":{"bookList":".book","name":".name","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;
        let sources = parse_sources_json(input).unwrap();
        assert_eq!(sources[0].name, "Demo");
        assert_eq!(sources[0].search_rule.url, "a::attr(href)");
        assert_eq!(sources[0].catalog_rule.item, ".chapter");
        assert_eq!(sources[0].concurrent_rate.as_deref(), Some("5/1000"));
    }

    #[test]
    fn imports_sources_without_search_url_and_normalizes_common_legado_rules() {
        let input = r#"[{"bookSourceUrl":"https://m.example.com","bookSourceName":"燃文小说","ruleSearch":{"list":"css: article","name":"css: .title","cover":"css: div.image img@src","detail":"css: div.image a@href"},"ruleBookInfo":{"name":"css: .info .title"},"ruleToc":{"list":"css: .chapter li a","name":"text","url":"@href","next":"css: .listpage .right a@href"},"ruleContent":{"content":"css: #text p","next":"css: .pagebar a:nth-child(4)@href"}}]"#;
        let sources = parse_sources_json(input).unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].search_url, "https://m.example.com");
        assert_eq!(sources[0].search_rule.item, "article");
        assert_eq!(
            sources[0].search_rule.cover.as_deref(),
            Some("div.image img::attr(src)")
        );
        assert_eq!(sources[0].search_rule.url, "div.image a::attr(href)");
        assert_eq!(sources[0].catalog_rule.title, "*");
        assert_eq!(sources[0].catalog_rule.url, "*::attr(href)");
        assert_eq!(sources[0].content_selector, "#text p");
        assert_eq!(
            sources[0].next_content_url_selector.as_deref(),
            Some(".pagebar a:nth-child(4)::attr(href)")
        );
    }
}
