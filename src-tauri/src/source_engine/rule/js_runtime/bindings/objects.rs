use super::super::js_error;
use super::super::JsHttpContext;
use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub(super) fn install<'js>(
    ctx: Ctx<'js>,
    globals: &Object<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    http: Option<JsHttpContext>,
) -> Result<(), AppError> {
    let book = Object::new(ctx.clone()).map_err(js_error)?;
    let chapter = Object::new(ctx.clone()).map_err(js_error)?;
    for object in [&book, &chapter] {
        let get_values = Arc::clone(variables);
        object
            .set(
                "getVariable",
                Function::new(ctx.clone(), move |key: Option<String>| {
                    key.as_deref()
                        .and_then(|key| {
                            get_values
                                .lock()
                                .ok()
                                .and_then(|values| values.get(key).cloned())
                        })
                        .unwrap_or_default()
                }),
            )
            .map_err(js_error)?;
        let put_values = Arc::clone(variables);
        object
            .set(
                "putVariable",
                Function::new(ctx.clone(), move |key: String, value: String| {
                    if let Ok(mut values) = put_values.lock() {
                        values.insert(key, value.clone());
                    }
                    value
                }),
            )
            .map_err(js_error)?;
    }
    globals.set("book", book).map_err(js_error)?;
    globals.set("chapter", chapter).map_err(js_error)?;
    let cookie = Object::new(ctx.clone()).map_err(js_error)?;
    let cookie_value = http
        .as_ref()
        .and_then(|context| context.session_cookie.clone())
        .unwrap_or_default();
    let get_cookie = cookie_value.clone();
    cookie
        .set(
            "getCookie",
            Function::new(ctx.clone(), move |_url: Option<String>| get_cookie.clone()),
        )
        .map_err(js_error)?;
    let get_cookie_key = cookie_value;
    cookie
        .set(
            "getKey",
            Function::new(ctx.clone(), move |_url: Option<String>, key: String| {
                get_cookie_key
                    .split(';')
                    .filter_map(|part| part.trim().split_once('='))
                    .find_map(|(name, value)| {
                        (name.trim() == key).then_some(value.trim().to_owned())
                    })
                    .unwrap_or_default()
            }),
        )
        .map_err(js_error)?;
    cookie
        .set("removeCookie", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    globals.set("cookie", cookie).map_err(js_error)?;
    globals.set("content", "").map_err(js_error)
}
