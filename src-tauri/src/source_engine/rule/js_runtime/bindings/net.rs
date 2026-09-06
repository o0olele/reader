use super::super::js_error;
use super::super::{
    request_options::{parse_request_options, JsHttpRequestOptions},
    transport::{blocking_http_request_with_options, JsHttpSession},
};
use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use std::sync::Arc;

pub(super) fn install_http_functions<'js>(
    ctx: Ctx<'js>,
    java: &Object<'js>,
    session: Arc<JsHttpSession>,
) -> Result<(), AppError> {
    let request_http = Arc::clone(&session);
    java.set(
        "request",
        Function::new(
            ctx.clone(),
            move |url: String, options: Option<Object<'js>>| {
                let options = options
                    .map(parse_request_options)
                    .transpose()
                    .map_err(|error| {
                        rquickjs::Error::new_from_js_message("RequestOptions", "Object", error)
                    })?
                    .unwrap_or_default();
                blocking_http_request_with_options(&request_http, &url, options).map_err(|error| {
                    rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
                })
            },
        ),
    )
    .map_err(js_error)?;
    let status_http = Arc::clone(&session);
    java.set(
        "responseStatus",
        Function::new(ctx.clone(), move || {
            status_http
                .response
                .lock()
                .ok()
                .and_then(|response| response.as_ref().map(|response| response.status))
                .unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    let headers_http = Arc::clone(&session);
    java.set(
        "responseHeaders",
        Function::new(ctx.clone(), move || {
            headers_http
                .response
                .lock()
                .ok()
                .and_then(|response| {
                    response.as_ref().map(|response| {
                        serde_json::to_string(&response.headers).unwrap_or_else(|_| "{}".into())
                    })
                })
                .unwrap_or_else(|| "{}".into())
        }),
    )
    .map_err(js_error)?;
    let header_http = Arc::clone(&session);
    java.set(
        "responseHeader",
        Function::new(ctx.clone(), move |name: String| {
            let name = name.to_ascii_lowercase();
            header_http
                .response
                .lock()
                .ok()
                .and_then(|response| {
                    response
                        .as_ref()
                        .and_then(|response| response.headers.get(&name).cloned())
                })
                .unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    let post_http = Arc::clone(&session);
    java.set(
        "post",
        Function::new(ctx.clone(), move |url: String, body: Option<String>| {
            blocking_http_request_with_options(
                &post_http,
                &url,
                JsHttpRequestOptions {
                    method: "POST".into(),
                    body,
                    ..Default::default()
                },
            )
            .map_err(|error| {
                rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
            })
        }),
    )
    .map_err(js_error)?;
    let head_http = Arc::clone(&session);
    java.set(
        "head",
        Function::new(ctx.clone(), move |url: String| {
            blocking_http_request_with_options(
                &head_http,
                &url,
                JsHttpRequestOptions {
                    method: "HEAD".into(),
                    ..Default::default()
                },
            )
            .map_err(|error| {
                rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
            })
        }),
    )
    .map_err(js_error)?;
    let connect_http = Arc::clone(&session);
    java.set(
        "connect",
        Function::new(ctx.clone(), move |url: String| {
            blocking_http_request_with_options(
                &connect_http,
                &url,
                JsHttpRequestOptions {
                    method: "GET".into(),
                    ..Default::default()
                },
            )
            .map_err(|error| {
                rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
            })
        }),
    )
    .map_err(js_error)?;
    let ajax_http = session;
    java.set(
        "ajaxRaw",
        Function::new(
            ctx.clone(),
            move |url: String, method: String, body: String| {
                blocking_http_request(
                    &ajax_http,
                    if method.is_empty() {
                        "GET"
                    } else {
                        method.as_str()
                    },
                    &url,
                    (!body.is_empty()).then_some(body),
                )
                .map_err(|error| {
                    rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
                })
            },
        ),
    )
    .map_err(js_error)?;
    // Legado calls java.ajax with one, two, or three arguments. QuickJS
    // enforces the native function's arity, so normalize omitted arguments in
    // a JS wrapper and always call the fixed-arity bridge with three values.
    ctx.eval::<(), _>(
        r#"
        java.ajax = function(url, method, body) {
            if (method && typeof method === 'object') {
                body = method.body;
                method = method.method;
            }
            method = method == null ? 'GET' : String(method);
            body = body == null ? '' : String(body);
            return java.ajaxRaw(String(url), method, body);
        };
        "#,
    )
    .map_err(js_error)
}

fn blocking_http_request(
    session: &JsHttpSession,
    method: &str,
    raw_url: &str,
    body: Option<String>,
) -> Result<String, AppError> {
    blocking_http_request_with_options(
        session,
        raw_url,
        JsHttpRequestOptions {
            method: method.to_owned(),
            body,
            ..Default::default()
        },
    )
}
