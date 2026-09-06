use super::super::js_error;
use super::super::{
    request_options::JsHttpRequestOptions,
    transport::{blocking_http_request_with_options, JsHttpSession},
};
use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub(super) fn install<'js>(
    ctx: Ctx<'js>,
    java: &Object<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    http_session: Option<Arc<JsHttpSession>>,
) -> Result<(), AppError> {
    let get_values = Arc::clone(variables);
    let get_session = http_session.clone();
    java.set(
        "get",
        Function::new(ctx.clone(), move |key: String| {
            if let Some(session) = get_session.as_ref().filter(|_| {
                key.starts_with("http://") || key.starts_with("https://") || key.starts_with('/')
            }) {
                return blocking_http_request_with_options(
                    session,
                    &key,
                    JsHttpRequestOptions {
                        method: "GET".into(),
                        ..Default::default()
                    },
                )
                .map_err(|error| {
                    rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
                });
            }
            Ok(get_values
                .lock()
                .ok()
                .and_then(|values| values.get(&key).cloned())
                .unwrap_or_default())
        }),
    )
    .map_err(js_error)?;
    let put_values = Arc::clone(variables);
    java.set(
        "put",
        Function::new(ctx.clone(), move |key: String, value: String| {
            if let Ok(mut values) = put_values.lock() {
                values.insert(key, value.clone());
            }
            value
        }),
    )
    .map_err(js_error)?;
    Ok(())
}
