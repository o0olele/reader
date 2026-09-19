//! `ruleBookInfo` projection; flat columns are used only without a raw stage.

use crate::{
    domain::source::{BookInfo, BookSource},
    error::AppError,
    source_engine::{
        legado_rules::{checked_stage, LegadoRules},
        pipeline::{first_in, joined_in, url_in},
        rule::RuleContext,
        url::absolutize,
    },
};

pub fn parse_book_info(source: &BookSource, html: &str) -> Result<BookInfo, AppError> {
    if let Some(rules) = checked_stage(
        source.raw_rules.book_info.as_ref(),
        LegadoRules::decode(&source.raw_rules).book_info,
        "ruleBookInfo",
    )? {
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
        return Ok(info);
    }
    crate::source_engine::selector::parse_book_info(source, html)
}
