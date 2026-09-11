//! Legado's `java.getElements` / `java.getElement` / `java.getStringList`
//! return Java containers, not strings: sources call `.select()`, `.attr()`,
//! `.text()`, `.html()`, `.get(i)`, `.size()`, `.toArray()` and `.sort()` on
//! them. A bare JavaScript array (or string) cannot answer any of that, which
//! is why 41 corpus rules failed with "not a function".
//!
//! The wrappers keep the old, working surface as well:
//!
//! - a collection really is a JavaScript array, so `length`, index access,
//!   `for…in`, `map` / `forEach` / `sort` / `concat` and `JSON.stringify` keep
//!   working — each element serializes back to its markup through `toJSON`;
//! - an element is an object whose `toString` / `valueOf` / `toJSON` all return
//!   its markup, so string concatenation and `String(element)` are unchanged.
//!
//! Every closure that needs to build a value takes the *current* `Ctx` as its
//! first parameter. Capturing the install-time context inside a closure would
//! keep the QuickJS context alive from one of its own objects, and the runtime
//! then fails `list_empty(&rt->gc_obj_list)` when it is freed.

mod collection;

use super::super::js_error;
use crate::error::AppError;
use crate::source_engine::rule::{engine::evaluate, jsoup::Extraction, model::RuleContext};
use rquickjs::{Ctx, Function, Object};
use scraper::{Html, Selector};

pub(super) use collection::{collection, string_list};

/// Converts a shim failure into a JavaScript exception.
pub(super) fn element_js_error(error: AppError) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Element", "String", error.to_string())
}

/// Resolves a Java-style index, where a negative value counts from the end.
pub(super) fn resolve(values: &[String], index: i32) -> Option<&String> {
    let length = values.len() as i64;
    let index = if index < 0 {
        length + index as i64
    } else {
        index as i64
    };
    usize::try_from(index).ok().and_then(|index| values.get(index))
}

/// Runs one rule against a single node's markup.
fn node_values(node: &str, rule: &str, want: Extraction) -> Vec<String> {
    evaluate(rule, node, want, &mut RuleContext::default()).unwrap_or_default()
}

pub(super) fn node_attr(node: &str, name: &str) -> String {
    node_values(node, &format!("*@{name}"), Extraction::Values)
        .into_iter()
        .next()
        .unwrap_or_default()
}

pub(super) fn node_text(node: &str) -> String {
    node_values(node, "*@text", Extraction::Values)
        .into_iter()
        .next()
        .unwrap_or_default()
}

pub(super) fn select_within(node: &str, css: &str) -> Vec<String> {
    // JSoup's `Element.select` takes plain CSS, not the legado rule dialect:
    // a bare `a` is a tag selector here, while the rule engine would read it as
    // an attribute name.
    let Ok(selector) = Selector::parse(css) else {
        return Vec::new();
    };
    Html::parse_fragment(node)
        .select(&selector)
        .map(|element| element.html())
        .collect()
}

/// Wraps one node's markup as a JSoup `Element`.
pub(super) fn element<'js>(ctx: &Ctx<'js>, node: String) -> Result<Object<'js>, AppError> {
    let object = Object::new(ctx.clone()).map_err(js_error)?;
    let markup = node.clone();
    object
        .set("html", Function::new(ctx.clone(), move || markup.clone()))
        .map_err(js_error)?;
    // `String(element)` and string concatenation must keep yielding the markup.
    for name in ["toString", "valueOf"] {
        let raw = node.clone();
        object
            .set(name, Function::new(ctx.clone(), move || raw.clone()))
            .map_err(js_error)?;
    }
    // `JSON.stringify(element)` would render `{}` without this hook.
    let serialized = node.clone();
    object
        .set(
            "toJSON",
            Function::new(ctx.clone(), move || serialized.clone()),
        )
        .map_err(js_error)?;
    let for_text = node.clone();
    object
        .set(
            "text",
            Function::new(ctx.clone(), move || node_text(&for_text)),
        )
        .map_err(js_error)?;
    let for_attr = node.clone();
    object
        .set(
            "attr",
            Function::new(ctx.clone(), move |name: String| {
                node_attr(&for_attr, &name)
            }),
        )
        .map_err(js_error)?;
    let for_select = node;
    object
        .set(
            "select",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>, css: String| {
                collection(&ctx, select_within(&for_select, &css)).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    Ok(object)
}
