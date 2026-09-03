//! The parsing entry point used by the services.

use crate::{
    domain::source::{BookInfo, BookSearchResult, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::{LegadoRules, LegadoTocRule},
        rule::{evaluate, evaluate_first, Extraction, RuleContext},
        selector,
    },
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

fn values(
    source: &BookSource,
    rule: &str,
    input: &str,
    want: Extraction,
) -> Result<Vec<String>, AppError> {
    let mut context = RuleContext::default();
    context.with_http(source.http_context());
    match evaluate(rule, input, want, &mut context) {
        Ok(values) => Ok(values),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(Vec::new())
        }
    }
}

fn first(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    let mut context = RuleContext::default();
    context.with_http(source.http_context());
    match evaluate_first(rule, input, &mut context) {
        Ok(value) => Ok(value),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(None)
        }
    }
}

fn joined(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    let text = values(source, rule, input, Extraction::Values)?.join("\n");
    Ok((!text.trim().is_empty()).then_some(text))
}

fn absolute(base: &str, value: &str) -> String {
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

pub fn parse_search(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).search {
        if let Some(list) = rules.book_list.as_deref() {
            let items = values(source, list, html, Extraction::Nodes)?;
            let mut results = Vec::new();
            for item in &items {
                let Some(title) = first(source, rules.name.as_ref(), item)? else {
                    continue;
                };
                let Some(url) = first(source, rules.book_url.as_ref(), item)? else {
                    continue;
                };
                results.push(BookSearchResult {
                    source_id: source.id,
                    source_name: source.name.clone(),
                    title,
                    author: first(source, rules.author.as_ref(), item)?,
                    cover: first(source, rules.cover_url.as_ref(), item)?
                        .map(|value| absolute(&source.base_url, &value)),
                    url: absolute(&source.base_url, &url),
                });
            }
            if !results.is_empty() {
                return Ok(results);
            }
        }
    }
    selector::parse_search(source, html).map_err(AppError::parse)
}

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).book_info {
        let info = BookInfo {
            title: first(source, rules.name.as_ref(), html)?,
            author: first(source, rules.author.as_ref(), html)?,
            intro: joined(source, rules.intro.as_ref(), html)?,
            cover: first(source, rules.cover_url.as_ref(), html)?
                .map(|value| absolute(&source.base_url, &value)),
            kind: first(source, rules.kind.as_ref(), html)?,
            latest_chapter: first(source, rules.last_chapter.as_ref(), html)?,
        };
        if [
            &info.title,
            &info.author,
            &info.intro,
            &info.cover,
            &info.kind,
            &info.latest_chapter,
        ]
        .iter()
        .any(|v| v.is_some())
        {
            return Ok(info);
        }
    }
    selector::parse_book_info(source, html).map_err(AppError::parse)
}

type CatalogPage = (Vec<(String, String)>, Option<String>);

fn engine_catalog(
    source: &BookSource,
    rules: &LegadoTocRule,
    html: &str,
) -> Result<Option<Vec<(String, String)>>, AppError> {
    let Some(list) = rules.chapter_list.as_deref() else {
        return Ok(None);
    };
    let items = values(source, list, html, Extraction::Nodes)?;
    let mut chapters = Vec::new();
    for item in &items {
        let Some(name) = first(source, rules.chapter_name.as_ref(), item)? else {
            continue;
        };
        let Some(url) = first(source, rules.chapter_url.as_ref(), item)? else {
            continue;
        };
        chapters.push((name, absolute(&source.base_url, &url)));
    }
    Ok((!chapters.is_empty()).then_some(chapters))
}

pub fn parse_catalog_page(source: &BookSource, html: &str) -> Result<CatalogPage, AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).toc;
    if let Some(rules) = rules.as_ref() {
        if let Some(chapters) = engine_catalog(source, rules, html)? {
            return Ok((chapters, first(source, rules.next_toc_url.as_ref(), html)?));
        }
    }
    selector::parse_catalog_page(source, html).map_err(AppError::parse)
}

#[cfg(test)]
mod tests {
    use super::strict_engine_value;

    #[test]
    fn strict_engine_only_accepts_enabled_values() {
        assert!(!strict_engine_value(None));
        assert!(!strict_engine_value(Some("0")));
        assert!(!strict_engine_value(Some("yes")));
        assert!(strict_engine_value(Some("1")));
        assert!(strict_engine_value(Some("TRUE")));
    }
}

pub fn parse_content_page(
    source: &BookSource,
    html: &str,
) -> Result<(String, Option<String>), AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).content;
    if let Some(rules) = rules.as_ref() {
        if let Some(rule) = rules.content.as_deref() {
            let content = values(source, rule, html, Extraction::Values)?.join("\n");
            if !content.trim().is_empty() {
                return Ok((
                    content,
                    first(source, rules.next_content_url.as_ref(), html)?,
                ));
            }
        }
    }
    selector::parse_content_page(source, html).map_err(AppError::parse)
}
