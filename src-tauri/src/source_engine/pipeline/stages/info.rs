//! `ruleBookInfo` projection, with the flat-column CSS fallback.

use crate::{
    domain::source::{BookInfo, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::LegadoRules,
        pipeline::{first_in, joined_in, url_in},
        rule::RuleContext,
        url::absolutize,
    },
};

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, AppError> {
    if let Some(rules) = LegadoRules::decode(&source.raw_rules).book_info {
        let mut context = RuleContext::default();
        context.with_http(source.http_context());
        let info = BookInfo {
            title: first_in(source, rules.name.as_ref(), html, &mut context)?,
            author: first_in(source, rules.author.as_ref(), html, &mut context)?,
            intro: joined_in(source, rules.intro.as_ref(), html, &mut context)?,
            cover: url_in(source, rules.cover_url.as_ref(), html, &mut context)?
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
