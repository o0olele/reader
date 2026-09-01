//! Sandboxed JavaScript execution for legado `<js>` rules.
//!
//! The runtime is intentionally kept behind a small trait.  Rule evaluation
//! can therefore use a real QuickJS implementation today while tests and
//! future WebView-backed implementations can provide another runtime.

use crate::error::AppError;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rquickjs::{Context, Ctx, Function, Object, Runtime};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone, Debug, Default)]
pub struct JsContext {
    pub result: String,
    pub url: Option<String>,
    pub variables: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Json(JsonValue),
}

#[async_trait]
pub trait JsRuntime: Send + Sync {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError>;
}

#[derive(Clone, Debug)]
pub struct QuickJsRuntime {
    timeout: Duration,
    memory_limit: usize,
}

impl Default for QuickJsRuntime {
    fn default() -> Self {
        Self::new(Duration::from_secs(5), 16 * 1024 * 1024)
    }
}

impl QuickJsRuntime {
    pub fn new(timeout: Duration, memory_limit: usize) -> Self {
        Self {
            timeout,
            memory_limit,
        }
    }

    pub fn execute_blocking(&self, script: &str, context: JsContext) -> Result<JsValue, AppError> {
        self.execute_blocking_with_context(script, context)
            .map(|(value, _)| value)
    }

    pub fn execute_blocking_with_context(
        &self,
        script: &str,
        context: JsContext,
    ) -> Result<(JsValue, HashMap<String, String>), AppError> {
        let runtime =
            Runtime::new().map_err(|error| AppError::Source(format!("JS runtime: {error}")))?;
        runtime.set_memory_limit(self.memory_limit);
        let deadline = Instant::now() + self.timeout;
        runtime.set_interrupt_handler(Some(Box::new(move || Instant::now() >= deadline)));
        let quick_context = Context::full(&runtime)
            .map_err(|error| AppError::Source(format!("JS context: {error}")))?;
        let values = Arc::new(Mutex::new(context.variables));
        let output = quick_context
            .with(|ctx| install_globals(ctx, &values, context.result, context.url))
            .and_then(|()| quick_context.with(|ctx| evaluate_script(ctx, script)))?;
        let variables = values
            .lock()
            .map(|values| values.clone())
            .unwrap_or_default();
        Ok((output, variables))
    }
}

#[async_trait]
impl JsRuntime for QuickJsRuntime {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError> {
        if script.trim().is_empty() {
            return Err(AppError::InvalidArgument("JavaScript 规则不能为空".into()));
        }
        let runtime = self.clone();
        let script = script.to_owned();
        tokio::time::timeout(
            self.timeout,
            tokio::task::spawn_blocking(move || runtime.execute_blocking(&script, context)),
        )
        .await
        .map_err(|_| {
            AppError::Source(format!(
                "JavaScript 执行超时（{}ms）",
                self.timeout.as_millis()
            ))
        })?
        .map_err(|error| AppError::Source(format!("JavaScript worker failed: {error}")))?
    }
}

fn install_globals<'js>(
    ctx: Ctx<'js>,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    result: String,
    url: Option<String>,
) -> Result<(), AppError> {
    let globals = ctx.globals();
    globals.set("result", result).map_err(js_error)?;
    globals
        .set("url", url.unwrap_or_default())
        .map_err(js_error)?;

    let java = Object::new(ctx.clone()).map_err(js_error)?;
    let get_values = Arc::clone(variables);
    java.set(
        "get",
        Function::new(ctx.clone(), move |key: String| {
            get_values
                .lock()
                .ok()
                .and_then(|values| values.get(&key).cloned())
                .unwrap_or_default()
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
    java.set(
        "base64Encode",
        Function::new(ctx.clone(), |value: String| STANDARD.encode(value)),
    )
    .map_err(js_error)?;
    java.set(
        "base64Decode",
        Function::new(ctx.clone(), |value: String| {
            STANDARD
                .decode(value)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "encodeURI",
        Function::new(ctx.clone(), |value: String| {
            urlencoding::encode(&value).into_owned()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "log",
        Function::new(ctx.clone(), |value: String| {
            tracing::debug!(target: "source", "JS: {value}");
        }),
    )
    .map_err(js_error)?;
    globals.set("java", java).map_err(js_error)
}

fn evaluate_script<'js>(ctx: Ctx<'js>, script: &str) -> Result<JsValue, AppError> {
    // Most legado rules are expressions (`result.trim()`). Try that first;
    // statement blocks use an explicit `return` and are evaluated second.
    let expression = format!("JSON.stringify(({script}))");
    let serialized = ctx
        .eval::<String, _>(expression.as_str())
        .or_else(|_| {
            let block_script = script
                .rsplit_once(';')
                .map(|(body, tail)| format!("{body}; return ({tail});"))
                .unwrap_or_else(|| script.to_owned());
            let block = format!(
                "JSON.stringify((function() {{ {block_script} }})()) || JSON.stringify(result)"
            );
            ctx.eval::<String, _>(block.as_str())
        })
        .map_err(js_error)?;
    let value: JsonValue = serde_json::from_str(&serialized)
        .map_err(|error| AppError::Parse(format!("JavaScript 返回值不是 JSON: {error}")))?;
    Ok(match value {
        JsonValue::String(value) => JsValue::String(value),
        JsonValue::Number(value) => JsValue::Number(value.as_f64().unwrap_or_default()),
        JsonValue::Bool(value) => JsValue::Boolean(value),
        JsonValue::Null => JsValue::Null,
        other => JsValue::Json(other),
    })
}

fn js_error(error: impl std::fmt::Display) -> AppError {
    AppError::Source(format!("JavaScript 执行失败: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn executes_expression_and_java_helpers() {
        let runtime = QuickJsRuntime::default();
        let value = runtime
            .execute(
                "java.base64Encode(result)",
                JsContext {
                    result: "hello".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("aGVsbG8=".into()));
    }

    #[tokio::test]
    async fn reads_and_writes_context_variables() {
        let runtime = QuickJsRuntime::default();
        let mut variables = HashMap::new();
        variables.insert("id".into(), "42".into());
        let value = runtime
            .execute(
                "java.put('id', java.get('id') + '0')",
                JsContext {
                    variables,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("420".into()));
    }

    #[tokio::test]
    async fn rejects_empty_script() {
        let error = QuickJsRuntime::default()
            .execute(" ", JsContext::default())
            .await
            .unwrap_err();
        assert!(matches!(error, AppError::InvalidArgument(_)));
    }
}
