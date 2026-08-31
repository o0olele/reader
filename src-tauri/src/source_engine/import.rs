//! Legado source import normalization for the CSS-compatible subset.

use crate::domain::source::{CatalogRule, InfoRule, SearchRule, SourceImport};

fn rule(value: Option<&serde_json::Value>, keys: &[&str]) -> Option<String> {
    value
        .and_then(|object| keys.iter().find_map(|key| object.get(*key)?.as_str()))
        .map(|value| {
            value
                .split("&&")
                .next()
                .unwrap_or(value)
                .trim()
                .trim_start_matches("@css:")
                .trim()
                .to_owned()
        })
        .filter(|value| !value.is_empty())
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
            .unwrap_or_default()
            .replace("<searchKey>", "{{key}}");
        if name.is_empty() || base_url.is_empty() || search_url.is_empty() {
            continue;
        }
        let search = json_value(
            object
                .get("search_rule")
                .or_else(|| object.get("ruleSearch")),
        );
        let catalog = json_value(object.get("catalog_rule").or_else(|| object.get("ruleToc")));
        let content_selector = field(object, "content_selector", "ruleContent")
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
                item: rule(search.as_ref(), &["item", "bookList"])
                    .unwrap_or_else(|| ".book".into()),
                title: rule(search.as_ref(), &["title", "name"]).unwrap_or_else(|| ".title".into()),
                author: rule(search.as_ref(), &["author"]),
                cover: rule(search.as_ref(), &["cover", "coverUrl"])
                    .map(|value| attr(value, "src")),
                url: rule(search.as_ref(), &["url", "bookUrl"])
                    .map(|value| attr(value, "href"))
                    .unwrap_or_else(|| "a::attr(href)".into()),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: rule(catalog.as_ref(), &["item", "chapterList"])
                    .unwrap_or_else(|| "a".into()),
                title: rule(catalog.as_ref(), &["title", "chapterName"])
                    .unwrap_or_else(|| "a".into()),
                url: rule(catalog.as_ref(), &["url", "chapterUrl"])
                    .map(|value| attr(value, "href"))
                    .unwrap_or_else(|| "a::attr(href)".into()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_legacy_legado_source() {
        let input = r#"[{"bookSourceName":"Demo","bookSourceUrl":"https://example.com","searchUrl":"https://example.com/s?q={{key}}","ruleSearch":{"bookList":".book","name":".name","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;
        let sources = parse_sources_json(input).unwrap();
        assert_eq!(sources[0].name, "Demo");
        assert_eq!(sources[0].search_rule.url, "a::attr(href)");
        assert_eq!(sources[0].catalog_rule.item, ".chapter");
    }
}
