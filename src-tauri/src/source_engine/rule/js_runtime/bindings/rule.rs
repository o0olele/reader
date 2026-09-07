use super::super::js_error;
use super::super::JsHttpContext;
use crate::error::AppError;
use crate::source_engine::rule::{engine::evaluate, jsoup::Extraction, model::RuleContext};
use rquickjs::{Ctx, Function, Object};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub(super) fn install<'js>(
    ctx: Ctx<'js>,
    java: &Object<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    rule_input: String,
    http: Option<JsHttpContext>,
) -> Result<(), AppError> {
    let get_string_values = Arc::clone(variables);
    let get_string_http = http.clone();
    let get_string_input = rule_input.clone();
    java.set(
        "getString",
        Function::new(ctx.clone(), move |rule: String| {
            nested_rule_values(
                &rule,
                &get_string_input,
                Extraction::Values,
                &get_string_values,
                get_string_http.clone(),
            )
            .map(|values| values.into_iter().next().unwrap_or_default())
            .map_err(rule_js_error)
        }),
    )
    .map_err(js_error)?;
    let get_elements_values = Arc::clone(variables);
    let get_elements_http = http.clone();
    let get_elements_input = rule_input.clone();
    java.set(
        "getElements",
        Function::new(ctx.clone(), move |rule: String| {
            nested_rule_values(
                &rule,
                &get_elements_input,
                Extraction::Nodes,
                &get_elements_values,
                get_elements_http.clone(),
            )
            .map_err(rule_js_error)
        }),
    )
    .map_err(js_error)?;
    let get_element_values = Arc::clone(variables);
    let get_element_http = http.clone();
    let get_element_input = rule_input.clone();
    java.set(
        "getElement",
        Function::new(ctx.clone(), move |rule: String| {
            nested_rule_values(
                &rule,
                &get_element_input,
                Extraction::Nodes,
                &get_element_values,
                get_element_http.clone(),
            )
            .map(|values| values.into_iter().next().unwrap_or_default())
            .map_err(rule_js_error)
        }),
    )
    .map_err(js_error)?;
    let list_values = Arc::clone(variables);
    java.set(
        "getStringList",
        Function::new(ctx.clone(), move |rule: String| {
            nested_rule_values(
                &rule,
                &rule_input,
                Extraction::Values,
                &list_values,
                http.clone(),
            )
            .map_err(rule_js_error)
        }),
    )
    .map_err(js_error)?;
    Ok(())
}

pub(super) fn nested_rule_values(
    rule: &str,
    input: &str,
    want: Extraction,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    http: Option<JsHttpContext>,
) -> Result<Vec<String>, AppError> {
    let snapshot = variables
        .lock()
        .map(|values| values.clone())
        .unwrap_or_default();
    let mut context = RuleContext::new(snapshot);
    if let Some(http) = http {
        context.with_http(http);
    }
    let values = evaluate(rule, input, want, &mut context).map_err(|error| {
        AppError::Source(format!("nested rule `{rule}` is not executable: {error}"))
    })?;
    if let Ok(mut shared) = variables.lock() {
        shared.extend(context.snapshot());
    }
    Ok(values)
}

pub(super) fn rule_js_error(error: AppError) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Rule", "String", error.to_string())
}
