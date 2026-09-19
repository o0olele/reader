//! The HTML-only `org.jsoup.Jsoup.parse` surface used by source scripts.
//! This is not JVM access; networking and DOM mutation are not implemented.

use super::super::js_error;
use super::elements::{collection, element, element_js_error};
use crate::error::AppError;
use rquickjs::{Coerced, Ctx, Function, Object};
use scraper::{Html, Selector};

pub(super) fn install<'js>(ctx: &Ctx<'js>) -> Result<(), AppError> {
    // Preserve the legacy `org` input alias's string methods/coercion without
    // adding a namespace to every string's prototype.
    let org: Object = ctx.eval("new String(result)").map_err(js_error)?;
    let namespace = Object::new(ctx.clone()).map_err(js_error)?;
    let jsoup = Object::new(ctx.clone()).map_err(js_error)?;
    jsoup
        .set(
            "parse",
            Function::new(ctx.clone(), |ctx: Ctx<'js>, html: Coerced<String>| {
                document(&ctx, html.0).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    namespace.set("Jsoup", jsoup).map_err(js_error)?;
    org.set("jsoup", namespace).map_err(js_error)?;
    ctx.globals().set("org", org).map_err(js_error)
}

fn document<'js>(ctx: &Ctx<'js>, html: String) -> Result<Object<'js>, AppError> {
    let markup = Html::parse_document(&html).root_element().html();
    let object = element(ctx, markup.clone())?;
    // A document must keep head/body nodes, which fragment parsing discards.
    object
        .set(
            "select",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>, css: String| {
                let selector = Selector::parse(&css).map_err(|error| {
                    element_js_error(AppError::parse(format!("JSoup selector: {error}")))
                })?;
                let nodes = Html::parse_document(&html)
                    .select(&selector)
                    .map(|node| node.html())
                    .collect();
                collection(&ctx, nodes).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    // Document.html() contains the html element itself, unlike Element.html().
    object
        .set("html", Function::new(ctx.clone(), move || markup.clone()))
        .map_err(js_error)?;
    Ok(object)
}
