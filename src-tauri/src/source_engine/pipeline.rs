//! The parsing entry point used by the services.
//!
//! A source imported from legado carries its rule objects verbatim, and those
//! run on the full rule engine. Sources written by hand in the UI only have the
//! flat CSS selectors, so this module falls back to [`super::selector`] for
//! them — and also whenever the engine cannot make sense of a legado rule, so
//! that adopting the engine cannot regress a source that used to work.

use crate::{
    domain::source::{BookInfo, BookSearchResult, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::{LegadoRules, LegadoTocRule},
        rule::{evaluate, evaluate_first, Extraction, RuleContext},
        selector,
    },
};

/// Runs `rule` against `input`, logging and swallowing engine errors so the
/// caller can fall back. Errors here mean "this rule is not executable", which
/// is expected while the engine's coverage grows.
fn values(source: &BookSource, rule: &str, input: &str, want: Extraction) -> Vec<String> {
    let mut context = RuleContext::default();
    context.with_http(source.http_context());
    match evaluate(rule, input, want, &mut context) {
        Ok(values) => values,
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            Vec::new()
        }
    }
}

fn first(source: &BookSource, rule: Option<&String>, input: &str) -> Option<String> {
    let rule = rule?;
    let mut context = RuleContext::default();
    context.with_http(source.http_context());
    match evaluate_first(rule, input, &mut context) {
        Ok(value) => value,
        Err(error) => {
            tracing::debug!(target: "source", source = %source.name, rule = %rule, %error, "rule engine could not execute rule");
            None
        }
    }
}

/// Like [`first`], but for prose that legitimately spans several nodes.
fn joined(source: &BookSource, rule: Option<&String>, input: &str) -> Option<String> {
    let rule = rule?;
    let text = values(source, rule, input, Extraction::Values).join("\n");
    (!text.trim().is_empty()).then_some(text)
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
    let engine = LegadoRules::decode(&source.raw_rules)
        .search
        .and_then(|rules| {
            let list = rules.book_list.as_deref()?;
            let items = values(source, list, html, Extraction::Nodes);
            let results = items
                .iter()
                .filter_map(|item| {
                    Some(BookSearchResult {
                        source_id: source.id,
                        source_name: source.name.clone(),
                        title: first(source, rules.name.as_ref(), item)?,
                        author: first(source, rules.author.as_ref(), item),
                        cover: first(source, rules.cover_url.as_ref(), item)
                            .map(|value| absolute(&source.base_url, &value)),
                        url: first(source, rules.book_url.as_ref(), item)
                            .map(|value| absolute(&source.base_url, &value))?,
                    })
                })
                .collect::<Vec<_>>();
            (!results.is_empty()).then_some(results)
        });
    match engine {
        Some(results) => Ok(results),
        None => selector::parse_search(source, html).map_err(AppError::parse),
    }
}

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, AppError> {
    let engine = LegadoRules::decode(&source.raw_rules)
        .book_info
        .map(|rules| BookInfo {
            title: first(source, rules.name.as_ref(), html),
            author: first(source, rules.author.as_ref(), html),
            intro: joined(source, rules.intro.as_ref(), html),
            cover: first(source, rules.cover_url.as_ref(), html)
                .map(|value| absolute(&source.base_url, &value)),
            kind: first(source, rules.kind.as_ref(), html),
            latest_chapter: first(source, rules.last_chapter.as_ref(), html),
        })
        // An info object that resolved nothing is indistinguishable from a
        // missing one; prefer the selector path over returning empty fields.
        .filter(|info| {
            [
                &info.title,
                &info.author,
                &info.intro,
                &info.cover,
                &info.kind,
                &info.latest_chapter,
            ]
            .iter()
            .any(|value| value.is_some())
        });
    match engine {
        Some(info) => Ok(info),
        None => selector::parse_book_info(source, html).map_err(AppError::parse),
    }
}

type CatalogPage = (Vec<(String, String)>, Option<String>);

fn engine_catalog(
    source: &BookSource,
    rules: &LegadoTocRule,
    html: &str,
) -> Option<Vec<(String, String)>> {
    let list = rules.chapter_list.as_deref()?;
    let chapters = values(source, list, html, Extraction::Nodes)
        .iter()
        .filter_map(|item| {
            Some((
                first(source, rules.chapter_name.as_ref(), item)?,
                absolute(
                    &source.base_url,
                    &first(source, rules.chapter_url.as_ref(), item)?,
                ),
            ))
        })
        .collect::<Vec<_>>();
    (!chapters.is_empty()).then_some(chapters)
}

pub fn parse_catalog_page(source: &BookSource, html: &str) -> Result<CatalogPage, AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).toc;
    let engine = rules
        .as_ref()
        .and_then(|rules| engine_catalog(source, rules, html));
    match engine {
        Some(chapters) => {
            let next = rules
                .as_ref()
                .and_then(|rules| first(source, rules.next_toc_url.as_ref(), html));
            Ok((chapters, next))
        }
        None => selector::parse_catalog_page(source, html).map_err(AppError::parse),
    }
}

pub fn parse_content_page(
    source: &BookSource,
    html: &str,
) -> Result<(String, Option<String>), AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).content;
    let engine = rules.as_ref().and_then(|rules| {
        let rule = rules.content.as_deref()?;
        let text = values(source, rule, html, Extraction::Values).join("\n");
        (!text.trim().is_empty()).then_some(text)
    });
    match engine {
        Some(content) => {
            let next = rules
                .as_ref()
                .and_then(|rules| first(source, rules.next_content_url.as_ref(), html));
            Ok((content, next))
        }
        None => selector::parse_content_page(source, html).map_err(AppError::parse),
    }
}
