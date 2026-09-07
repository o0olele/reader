//! Stage parsers shared by the search, explore, reader and debugger flows.

use crate::{
    domain::source::{BookInfo, BookSearchResult, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::{LegadoRules, LegadoTocRule},
        rule::{Extraction, RuleContext},
        url::absolutize,
    },
};

use super::{first_in, joined_in, values_in};

pub fn parse_search(
    source: &BookSource,
    html: &str,
) -> Result<Vec<BookSearchResult>, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).search {
        if let Some(list) = rules.book_list.as_deref() {
            let mut list_context = RuleContext::default();
            list_context.with_http(source.http_context());
            let items = values_in(source, list, html, Extraction::Nodes, &mut list_context)?;
            let mut results = Vec::new();
            for item in &items {
                let mut context = RuleContext::new(list_context.snapshot());
                context.with_http(source.http_context());
                let Some(title) = first_in(source, rules.name.as_ref(), item, &mut context)? else { continue };
                let Some(url) = first_in(source, rules.book_url.as_ref(), item, &mut context)? else { continue };
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
            if !results.is_empty() {
                return Ok(results);
            }
        }
    }
    match crate::source_engine::selector::parse_search(source, html) {
        Ok(results) => Ok(results),
        // A raw legado rule is authoritative. Its flat projection may be a
        // JSONPath/private expression that cannot be compiled by scraper (for
        // example `$.list` or `tr!0`). If the engine produced no items, do not
        // turn that expected no-match into a misleading CSS parse failure.
        Err(error)
            if !source.raw_rules.is_empty()
                && error.to_string().starts_with("parse error: 搜索结果选择器无效:") =>
        {
            tracing::debug!(target: "source", source = %source.name, %error, "ignoring invalid CSS projection after raw rule no-match");
            Ok(Vec::new())
        }
        Err(error) => Err(error),
    }
}

/// Applies Legado's shortcut for search URLs that resolve directly to a book page.
pub fn parse_search_response(
    source: &BookSource,
    html: &str,
    response_url: &str,
) -> Result<Vec<BookSearchResult>, AppError> {
    let pattern_matches = source
        .book_url_pattern
        .as_deref()
        .filter(|pattern| !pattern.trim().is_empty() && !pattern.eq_ignore_ascii_case("NONE"))
        .is_some_and(|pattern| {
            let normalized =
                crate::source_engine::rule::regex_compat::normalize_java_regex(pattern);
            fancy_regex::Regex::new(&format!("^(?:{normalized})$"))
                .ok()
                .and_then(|regex| regex.is_match(response_url).ok())
                .unwrap_or(false)
        });
    if !pattern_matches {
        return parse_search(source, html);
    }
    let info = parse_book_info(source, html)?;
    let Some(title) = info.title else {
        return Ok(Vec::new());
    };
    Ok(vec![BookSearchResult {
        source_id: source.id,
        source_name: source.name.clone(),
        title,
        author: info.author,
        cover: info.cover,
        url: response_url.to_owned(),
        intro: info.intro,
        kind: info.kind,
        latest_chapter: info.latest_chapter,
        word_count: None,
    }])
}

pub fn parse_explore(
    source: &BookSource,
    html: &str,
) -> Result<Vec<BookSearchResult>, AppError> {
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
    crate::source_engine::selector::parse_book_info(source, html)
}

pub(crate) type CatalogPage = (Vec<(String, String)>, Option<String>);

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

pub fn parse_catalog_page(
    source: &BookSource,
    html: &str,
) -> Result<CatalogPage, AppError> {
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
    crate::source_engine::selector::parse_catalog_page(source, html)
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
    crate::source_engine::selector::parse_content_page(source, html)
}
