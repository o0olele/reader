//! The parsing entry point used by the services.

use crate::{
    domain::source::{BookInfo, BookSearchResult, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::{LegadoRules, LegadoTocRule},
        rule::{evaluate, evaluate_first, Extraction, RuleContext},
        selector,
        url::absolutize,
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

fn values_in(
    source: &BookSource,
    rule: &str,
    input: &str,
    want: Extraction,
    context: &mut RuleContext,
) -> Result<Vec<String>, AppError> {
    match evaluate(rule, input, want, context) {
        Ok(values) => Ok(values),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(Vec::new())
        }
    }
}

fn first_in(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
    context: &mut RuleContext,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    match evaluate_first(rule, input, context) {
        Ok(value) => Ok(value),
        Err(error) if strict_engine() => Err(engine_error(source, rule, error)),
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Ok(None)
        }
    }
}

fn joined_in(
    source: &BookSource,
    rule: Option<&String>,
    input: &str,
    context: &mut RuleContext,
) -> Result<Option<String>, AppError> {
    let Some(rule) = rule else { return Ok(None) };
    let text = values_in(source, rule, input, Extraction::Values, context)?.join("\n");
    Ok((!text.trim().is_empty()).then_some(text))
}

pub fn parse_search(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).search {
        if let Some(list) = rules.book_list.as_deref() {
            let mut list_context = RuleContext::default();
            list_context.with_http(source.http_context());
            let items = values_in(source, list, html, Extraction::Nodes, &mut list_context)?;
            let mut results = Vec::new();
            for item in &items {
                let mut context = RuleContext::new(list_context.snapshot());
                context.with_http(source.http_context());
                let Some(title) = first_in(source, rules.name.as_ref(), item, &mut context)? else {
                    continue;
                };
                let Some(url) = first_in(source, rules.book_url.as_ref(), item, &mut context)?
                else {
                    continue;
                };
                results.push(BookSearchResult {
                    source_id: source.id,
                    source_name: source.name.clone(),
                    title,
                    author: first_in(source, rules.author.as_ref(), item, &mut context)?,
                    cover: first_in(source, rules.cover_url.as_ref(), item, &mut context)?
                        .map(|value| absolutize(&source.base_url, &value)),
                    url: absolutize(&source.base_url, &url),
                    intro: joined_in(source, rules.intro.as_ref(), item, &mut context)?,
                    kind: first_in(source, rules.kind.as_ref(), item, &mut context)?,
                    latest_chapter: first_in(
                        source,
                        rules.last_chapter.as_ref(),
                        item,
                        &mut context,
                    )?,
                    word_count: first_in(source, rules.word_count.as_ref(), item, &mut context)?,
                });
            }
            if !results.is_empty() {
                return Ok(results);
            }
        }
    }
    match selector::parse_search(source, html) {
        Ok(results) => Ok(results),
        // A raw legado rule is authoritative. Its flat projection may be a
        // JSONPath/private expression that cannot be compiled by scraper (for
        // example `$.list` or `tr!0`). If the engine produced no items, do not
        // turn that expected no-match into a misleading CSS parse failure.
        Err(error) if !source.raw_rules.is_empty() && error.starts_with("搜索结果选择器无效:") =>
        {
            tracing::debug!(target: "source", source = %source.name, error, "ignoring invalid CSS projection after raw rule no-match");
            Ok(Vec::new())
        }
        Err(error) => Err(AppError::parse(error)),
    }
}

pub fn parse_explore(source: &BookSource, html: &str) -> Result<Vec<BookSearchResult>, AppError> {
    let Some(rules) = LegadoRules::decode(&source.raw_rules).explore else {
        return Err(AppError::parse(format!(
            "书源 `{}` 未配置发现规则",
            source.name
        )));
    };
    let Some(list) = rules.book_list.as_deref() else {
        return Err(AppError::parse(format!(
            "书源 `{}` 的发现规则缺少 bookList",
            source.name
        )));
    };
    let mut list_context = RuleContext::default();
    list_context.with_http(source.http_context());
    let items = values_in(source, list, html, Extraction::Nodes, &mut list_context)?;
    let mut results = Vec::new();
    for item in &items {
        let mut context = RuleContext::new(list_context.snapshot());
        context.with_http(source.http_context());
        let Some(title) = first_in(source, rules.name.as_ref(), item, &mut context)? else {
            continue;
        };
        let Some(url) = first_in(source, rules.book_url.as_ref(), item, &mut context)? else {
            continue;
        };
        results.push(BookSearchResult {
            source_id: source.id,
            source_name: source.name.clone(),
            title,
            author: first_in(source, rules.author.as_ref(), item, &mut context)?,
            cover: first_in(source, rules.cover_url.as_ref(), item, &mut context)?
                .map(|value| absolutize(&source.base_url, &value)),
            url: absolutize(&source.base_url, &url),
            intro: joined_in(source, rules.intro.as_ref(), item, &mut context)?,
            kind: first_in(source, rules.kind.as_ref(), item, &mut context)?,
            latest_chapter: first_in(source, rules.last_chapter.as_ref(), item, &mut context)?,
            word_count: first_in(source, rules.word_count.as_ref(), item, &mut context)?,
        });
    }
    Ok(results)
}

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).book_info {
        let mut context = RuleContext::default();
        context.with_http(source.http_context());
        let info = BookInfo {
            title: first_in(source, rules.name.as_ref(), html, &mut context)?,
            author: first_in(source, rules.author.as_ref(), html, &mut context)?,
            intro: joined_in(source, rules.intro.as_ref(), html, &mut context)?,
            cover: first_in(source, rules.cover_url.as_ref(), html, &mut context)?
                .map(|value| absolutize(&source.base_url, &value)),
            kind: first_in(source, rules.kind.as_ref(), html, &mut context)?,
            latest_chapter: first_in(source, rules.last_chapter.as_ref(), html, &mut context)?,
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
    let mut list_context = RuleContext::default();
    list_context.with_http(source.http_context());
    let items = values_in(source, list, html, Extraction::Nodes, &mut list_context)?;
    let mut chapters = Vec::new();
    for item in &items {
        let mut context = RuleContext::new(list_context.snapshot());
        context.with_http(source.http_context());
        let Some(name) = first_in(source, rules.chapter_name.as_ref(), item, &mut context)? else {
            continue;
        };
        let Some(url) = first_in(source, rules.chapter_url.as_ref(), item, &mut context)? else {
            continue;
        };
        chapters.push((name, absolutize(&source.base_url, &url)));
    }
    Ok((!chapters.is_empty()).then_some(chapters))
}

pub fn parse_catalog_page(source: &BookSource, html: &str) -> Result<CatalogPage, AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).toc;
    if let Some(rules) = rules.as_ref() {
        if let Some(chapters) = engine_catalog(source, rules, html)? {
            let mut context = RuleContext::default();
            context.with_http(source.http_context());
            return Ok((
                chapters,
                first_in(source, rules.next_toc_url.as_ref(), html, &mut context)?,
            ));
        }
    }
    selector::parse_catalog_page(source, html).map_err(AppError::parse)
}

pub fn parse_content_page(
    source: &BookSource,
    html: &str,
) -> Result<(String, Option<String>), AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).content;
    if let Some(rules) = rules.as_ref() {
        if let Some(rule) = rules.content.as_deref() {
            let mut context = RuleContext::default();
            context.with_http(source.http_context());
            let content =
                values_in(source, rule, html, Extraction::Values, &mut context)?.join("\n");
            if !content.trim().is_empty() {
                return Ok((
                    content,
                    first_in(source, rules.next_content_url.as_ref(), html, &mut context)?,
                ));
            }
        }
    }
    selector::parse_content_page(source, html).map_err(AppError::parse)
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
