//! `ruleToc` projection, including the paged catalog's `nextTocUrl`.

use crate::{
    domain::source::BookSource,
    error::AppError,
    source_engine::{
        legado_rules::{LegadoRules, LegadoTocRule},
        pipeline::{first_in, values_in},
        rule::{Extraction, RuleContext},
        url::absolutize,
    },
};

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
