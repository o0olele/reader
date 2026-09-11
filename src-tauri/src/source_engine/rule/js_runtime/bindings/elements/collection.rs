//! The collection half of the Java container shim: JSoup's `Elements` and
//! Java's `ArrayList<String>`.

use super::super::super::js_error;
use super::{element, element_js_error, node_attr, node_text, resolve, select_within};
use crate::error::AppError;
use rquickjs::{Array, Ctx, Function};

/// Wraps extracted node markup as a JSoup `Elements` collection.
pub(crate) fn collection<'js>(
    ctx: &Ctx<'js>,
    nodes: Vec<String>,
) -> Result<Array<'js>, AppError> {
    let array = Array::new(ctx.clone()).map_err(js_error)?;
    for (index, node) in nodes.iter().enumerate() {
        array
            .set(index, element(ctx, node.clone())?)
            .map_err(js_error)?;
    }
    let object = array.as_object();
    let first = nodes.first().cloned().unwrap_or_default();
    object
        .set(
            "attr",
            Function::new(ctx.clone(), move |name: String| node_attr(&first, &name)),
        )
        .map_err(js_error)?;
    let texts = nodes.clone();
    object
        .set(
            "text",
            Function::new(ctx.clone(), move || {
                texts
                    .iter()
                    .map(|node| node_text(node))
                    .collect::<Vec<_>>()
                    .join(" ")
            }),
        )
        .map_err(js_error)?;
    let markup = nodes.clone();
    object
        .set("html", Function::new(ctx.clone(), move || markup.join("\n")))
        .map_err(js_error)?;
    let size = nodes.len();
    object
        .set("size", Function::new(ctx.clone(), move || size as i32))
        .map_err(js_error)?;
    let for_get = nodes.clone();
    object
        .set(
            "get",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>, index: i32| {
                element(&ctx, resolve(&for_get, index).cloned().unwrap_or_default())
                    .map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    let for_first = nodes.clone();
    object
        .set(
            "first",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>| {
                element(&ctx, for_first.first().cloned().unwrap_or_default())
                    .map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    let last = nodes.last().cloned().unwrap_or_default();
    object
        .set(
            "last",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>| {
                element(&ctx, last.clone()).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    let for_to_array = nodes.clone();
    object
        .set(
            "toArray",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>| {
                collection(&ctx, for_to_array.clone()).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    let for_select = nodes;
    object
        .set(
            "select",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>, css: String| {
                let found = for_select
                    .iter()
                    .flat_map(|node| select_within(node, &css))
                    .collect();
                collection(&ctx, found).map_err(element_js_error)
            }),
        )
        .map_err(js_error)?;
    Ok(array)
}

/// Adds Java's `ArrayList<String>` methods to a plain string array.
///
/// The array already provides `map` / `forEach` / `join`, so only the Java
/// spellings need adding.
pub(crate) fn string_list<'js>(
    ctx: &Ctx<'js>,
    values: Vec<String>,
) -> Result<Array<'js>, AppError> {
    let array = Array::new(ctx.clone()).map_err(js_error)?;
    for (index, value) in values.iter().enumerate() {
        array.set(index, value.clone()).map_err(js_error)?;
    }
    let object = array.as_object();
    let size = values.len();
    object
        .set("size", Function::new(ctx.clone(), move || size as i32))
        .map_err(js_error)?;
    let for_get = values.clone();
    object
        .set(
            "get",
            Function::new(ctx.clone(), move |index: i32| {
                resolve(&for_get, index).cloned().unwrap_or_default()
            }),
        )
        .map_err(js_error)?;
    let first = values.first().cloned().unwrap_or_default();
    object
        .set("first", Function::new(ctx.clone(), move || first.clone()))
        .map_err(js_error)?;
    let last = values.last().cloned().unwrap_or_default();
    object
        .set("last", Function::new(ctx.clone(), move || last.clone()))
        .map_err(js_error)?;
    let for_to_array = values.clone();
    object
        .set(
            "toArray",
            Function::new(ctx.clone(), move |ctx: Ctx<'js>| {
                let array = Array::new(ctx.clone())?;
                for (index, value) in for_to_array.iter().enumerate() {
                    array.set(index, value.clone())?;
                }
                Ok::<_, rquickjs::Error>(array)
            }),
        )
        .map_err(js_error)?;
    Ok(array)
}
