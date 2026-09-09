//! `ruleContent` projection, including `nextContentUrl` and HTML-to-text.

use crate::{
    domain::source::BookSource,
    error::AppError,
    source_engine::{
        html_text::html_to_text,
        legado_rules::LegadoRules,
        pipeline::{first_in, values_in},
        rule::{Extraction, RuleContext},
    },
};

pub fn parse_content_page(
    source: &BookSource,
    html: &str,
) -> Result<(String, Option<String>), AppError> {
    let rules = LegadoRules::decode(&source.raw_rules).content;
    if let Some(rules) = rules.as_ref() {
        if let Some(rule) = rules.content.as_deref() {
            let mut context = RuleContext::default();
            context.with_http(source.http_context());
            let raw =
                values_in(source, rule, html, Extraction::Values, &mut context)?.join("\n");
            // Legado runs HtmlFormatter.formatKeepImg on the extracted content
            // before any further processing; without it, `@html`/`@all` rules
            // leak markup like <div>, <br> and &nbsp; into the reader.
            let content = html_to_text(&raw);
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
