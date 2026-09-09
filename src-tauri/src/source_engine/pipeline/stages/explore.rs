//! `exploreRule` / `ruleExplore` projection.

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
