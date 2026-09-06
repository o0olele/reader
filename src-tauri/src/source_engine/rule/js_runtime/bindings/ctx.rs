use super::super::js_error;
use super::super::JsHttpContext;
use super::super::{time::format_epoch, transport::build_js_http_session};
use super::crypto::stable_android_id;
use crate::error::AppError;
use rquickjs::{Ctx, Function, Object};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub(in super::super) fn install_globals<'js>(
    ctx: Ctx<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    result: String,
    url: Option<String>,
    key: Option<String>,
    base_url: Option<String>,
    http: Option<JsHttpContext>,
    title: Option<String>,
    src: Option<String>,
) -> Result<(), AppError> {
    let globals = ctx.globals();
    let rule_input = result.clone();
    let base_url_value = base_url.unwrap_or_default();
    globals.set("result", result).map_err(js_error)?;
    globals
        .set("url", url.unwrap_or_default())
        .map_err(js_error)?;
    globals
        .set("key", key.unwrap_or_default())
        .map_err(js_error)?;
    globals
        .set("baseUrl", base_url_value.clone())
        .map_err(js_error)?;
    // Legado exposes pagination as a built-in even when the URL rule does not
    // explicitly declare it.  Source exports commonly reference `page`
    // directly while constructing signed API requests.
    globals.set("page", "1").map_err(js_error)?;
    globals
        .set("title", title.unwrap_or_default())
        .map_err(js_error)?;
    globals
        .set("src", src.unwrap_or_else(|| rule_input.clone()))
        .map_err(js_error)?;
    // Rhino-era sources often use these unqualified aliases. Keep them
    // harmless and deterministic in QuickJS so a missing convenience helper
    // does not abort the whole rule.
    globals.set("org", rule_input.clone()).map_err(js_error)?;
    globals.set("run", rule_input.clone()).map_err(js_error)?;
    globals
        .set("time", format_epoch(0, "yyyy-MM-dd HH:mm:ss"))
        .map_err(js_error)?;
    let alias_url = base_url_value.clone();
    globals
        .set(
            "getUrl",
            Function::new(ctx.clone(), move || alias_url.clone()),
        )
        .map_err(js_error)?;
    let alias_uuid = Uuid::new_v4().to_string();
    globals
        .set(
            "uuid",
            Function::new(ctx.clone(), move || alias_uuid.clone()),
        )
        .map_err(js_error)?;

    let java = Object::new(ctx.clone()).map_err(js_error)?;
    let session_state = http
        .as_ref()
        .map(|value| {
            if value.session_expired {
                "expired"
            } else if value.access_token.is_some() || value.session_cookie.is_some() {
                "authenticated"
            } else {
                "anonymous"
            }
        })
        .unwrap_or("anonymous")
        .to_owned();
    let authenticated = session_state == "authenticated";
    java.set(
        "isAuthenticated",
        Function::new(ctx.clone(), move || authenticated),
    )
    .map_err(js_error)?;
    let state_for_js = session_state.clone();
    java.set(
        "sessionState",
        Function::new(ctx.clone(), move || state_for_js.clone()),
    )
    .map_err(js_error)?;
    // A number of legacy sources use the Android-only device identifier when
    // constructing request headers.  Desktop has no Android ID, but exposing
    // a stable, non-empty identifier keeps those scripts executable and gives
    // remote APIs the same per-installation shape they expect.  Derive it from
    // the source base URL so different sources do not accidentally share a
    // credential-like value while remaining deterministic across requests.
    let android_id = stable_android_id(&base_url_value);
    java.set(
        "androidId",
        Function::new(ctx.clone(), move || android_id.clone()),
    )
    .map_err(js_error)?;
    let http_session = http
        .clone()
        .map(build_js_http_session)
        .transpose()?
        .map(Arc::new);
    super::context::install(ctx.clone(), &java, variables, http_session.clone())?;
    super::rule::install(ctx.clone(), &java, variables, rule_input, http.clone())?;
    super::codec::install(ctx.clone(), &java)?;
    super::crypto::install(ctx.clone(), &java)?;
    super::source::install_source_compat(ctx.clone(), &globals, variables, &base_url_value, http)?;
    globals.set("java", java.clone()).map_err(js_error)?;
    super::utility::install(ctx.clone(), &java)?;
    if let Some(session) = http_session {
        super::net::install_http_functions(ctx, &java, session)?;
    }
    Ok(())
}
