use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use super::super::js_error;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use super::super::JsHttpContext;
use serde_json::Value as JsonValue;
use crate::source_engine::rule::jsoup::Extraction;
use super::rule::{nested_rule_values, rule_js_error};

pub(super) fn install_source_compat<'js>(
    ctx: Ctx<'js>,
    globals: &Object<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    base_url: &str,
    http: Option<JsHttpContext>,
) -> Result<(), AppError> {
    let source = Object::new(ctx.clone()).map_err(js_error)?;
    let get_values = Arc::clone(variables);
    source
        .set(
            "getVariable",
            Function::new(ctx.clone(), move |key: Option<String>| {
                let values = get_values.lock().ok();
                let raw = values
                    .as_ref()
                    .and_then(|values| values.get("source"))
                    .cloned()
                    .unwrap_or_default();
                match key.as_deref().filter(|key| !key.is_empty()) {
                    None => raw,
                    Some(key) => values
                        .as_ref()
                        .and_then(|values| values.get(key))
                        .cloned()
                        .or_else(|| {
                            serde_json::from_str::<JsonValue>(&raw)
                                .ok()
                                .and_then(|value| value.get(key).map(|value| value.to_string()))
                        })
                        .unwrap_or_default(),
                }
            }),
        )
        .map_err(js_error)?;
    let set_values = Arc::clone(variables);
    source
        .set(
            "setVariable",
            Function::new(ctx.clone(), move |value: String| {
                if let Ok(mut values) = set_values.lock() {
                    values.insert("source".into(), value.clone());
                }
                value
            }),
        )
        .map_err(js_error)?;
    let get_key = base_url.to_owned();
    source
        .set(
            "getKey",
            Function::new(ctx.clone(), move || get_key.clone()),
        )
        .map_err(js_error)?;
    let login_header = Arc::new(Mutex::new(
        http.as_ref()
            .and_then(|context| context.headers.clone())
            .unwrap_or_default(),
    ));
    let get_header = Arc::clone(&login_header);
    source
        .set(
            "getLoginHeader",
            Function::new(ctx.clone(), move || {
                get_header
                    .lock()
                    .map(|header| header.clone())
                    .unwrap_or_default()
            }),
        )
        .map_err(js_error)?;
    let put_header = Arc::clone(&login_header);
    source
        .set(
            "putLoginHeader",
            Function::new(ctx.clone(), move |header: String| {
                if let Ok(mut value) = put_header.lock() {
                    *value = header.clone();
                }
                header
            }),
        )
        .map_err(js_error)?;
    let clear_header = Arc::clone(&login_header);
    source
        .set(
            "removeLoginHeader",
            Function::new(ctx.clone(), move || {
                if let Ok(mut value) = clear_header.lock() {
                    value.clear();
                }
            }),
        )
        .map_err(js_error)?;
    source.set("key", base_url.to_owned()).map_err(js_error)?;
    source
        .set("loginUrl", base_url.to_owned())
        .map_err(js_error)?;
    globals.set("source", source).map_err(js_error)?;

    // Legado returns Java Map-like objects from these methods. A small JS
    // wrapper gives source rules both property access and `.get(key)` without
    // exposing Rust implementation details to QuickJS.
    ctx.eval::<(), _>(
        r#"
        source.getLoginHeaderMap = function() {
            var raw = String(source.getLoginHeader() || '').replace(/^#/, '');
            var map = {};
            try { map = JSON.parse(raw || '{}') || {}; } catch (e) {}
            map.get = function(key) { return map[key]; };
            return map;
        };
        source.getLoginInfoMap = function() {
            var map = {};
            map.get = function(key) { return map[key]; };
            return map;
        };
        "#,
    )
    .map_err(js_error)?;

    let s_values = Arc::clone(variables);
    let s_input = globals
        .get::<_, String>("result")
        .map_err(js_error)
        .unwrap_or_default();
    let s_http = http.clone();
    globals
        .set(
            "S",
            Function::new(ctx.clone(), move |rule: String| {
                nested_rule_values(
                    &rule,
                    &s_input,
                    Extraction::Values,
                    &s_values,
                    s_http.clone(),
                )
                .map(|values| values.into_iter().next().unwrap_or_default())
                .map_err(rule_js_error)
            }),
        )
        .map_err(js_error)?;

    super::objects::install(ctx, globals, variables, http)
}
