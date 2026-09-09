//! `searchRule` / `ruleSearch` projection, plus Legado's detail-page shortcut.

use crate::{
    domain::source::{BookSearchResult, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::LegadoRules,
        pipeline::{first_in, joined_in, values_in},
        rule::{Extraction, RuleContext},
        url::absolutize,
    },
};

use super::info::parse_book_info;

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
