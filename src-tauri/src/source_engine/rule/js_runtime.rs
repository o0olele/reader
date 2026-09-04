//! Sandboxed JavaScript execution for legado `<js>` rules.
//!
//! The runtime is intentionally kept behind a small trait.  Rule evaluation
//! can therefore use a real QuickJS implementation today while tests and
//! future WebView-backed implementations can provide another runtime.

use crate::error::AppError;
use crate::infrastructure::http::request::evaluate_sign_script;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use md5::{Digest, Md5};
use rquickjs::{Context, Ctx, Function, Object, Runtime};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

/// Credentials and request defaults exposed to a source's JavaScript rules.
/// Network access is deliberately only available through these injected
/// functions; the QuickJS sandbox has no filesystem, process, or environment
/// access.
#[derive(Clone, Debug, Default)]
pub struct JsHttpContext {
    pub base_url: String,
    pub headers: Option<String>,
    pub access_token: Option<String>,
    pub session_cookie: Option<String>,
    pub session_expired: bool,
    pub sign_script: Option<String>,
}

struct JsHttpSession {
    client: reqwest::blocking::Client,
    context: JsHttpContext,
    response: Arc<Mutex<Option<JsHttpResponse>>>,
}

#[derive(Clone, Debug, Default)]
struct JsHttpResponse {
    status: u16,
    headers: HashMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub struct JsContext {
    pub result: String,
    pub url: Option<String>,
    pub key: Option<String>,
    pub base_url: Option<String>,
    pub variables: HashMap<String, String>,
    pub http: Option<JsHttpContext>,
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
        // `reqwest::blocking::Client` owns a private Tokio runtime. Dropping
        // it on a Tokio worker panics (`Cannot drop a runtime in a context
        // where blocking is not allowed`). URL parsing and the rule pipeline
        // are synchronous APIs, so isolate the whole QuickJS session on a
        // plain thread whenever this entry point is reached from async code.
        if tokio::runtime::Handle::try_current().is_ok() {
            let runtime = self.clone();
            let script = script.to_owned();
            return std::thread::spawn(move || runtime.execute_blocking_inner(&script, context))
                .join()
                .map_err(|_| AppError::Source("JavaScript worker panicked".into()))?;
        }
        self.execute_blocking_inner(script, context)
    }

    fn execute_blocking_inner(
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
            .with(|ctx| {
                install_globals(
                    ctx,
                    &values,
                    context.result,
                    context.url,
                    context.key,
                    context.base_url,
                    context.http,
                )
            })
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
    key: Option<String>,
    base_url: Option<String>,
    http: Option<JsHttpContext>,
) -> Result<(), AppError> {
    let globals = ctx.globals();
    globals.set("result", result).map_err(js_error)?;
    globals
        .set("url", url.unwrap_or_default())
        .map_err(js_error)?;
    globals
        .set("key", key.unwrap_or_default())
        .map_err(js_error)?;
    globals
        .set("baseUrl", base_url.unwrap_or_default())
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
    let get_values = Arc::clone(variables);
    let http_session = http.map(build_js_http_session).transpose()?.map(Arc::new);
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
        "hexEncodeToString",
        Function::new(ctx.clone(), |value: String| {
            value
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "hexDecodeToString",
        Function::new(ctx.clone(), |value: String| {
            let bytes = value
                .as_bytes()
                .chunks(2)
                .filter_map(|chunk| {
                    (chunk.len() == 2)
                        .then(|| std::str::from_utf8(chunk).ok())
                        .flatten()
                        .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                })
                .collect::<Vec<_>>();
            String::from_utf8(bytes).unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "md5Encode",
        Function::new(ctx.clone(), |value: String| {
            let mut digest = Md5::new();
            digest.update(value.as_bytes());
            format!("{:x}", digest.finalize())
        }),
    )
    .map_err(js_error)?;
    java.set(
        "strToBytes",
        Function::new(ctx.clone(), |value: String| {
            value
                .as_bytes()
                .iter()
                .map(|byte| byte.to_string())
                .collect::<Vec<_>>()
                .join(",")
        }),
    )
    .map_err(js_error)?;
    java.set(
        "bytesToStr",
        Function::new(ctx.clone(), |value: String| {
            let bytes = value
                .split([',', ' ', '\n'])
                .filter(|part| !part.is_empty())
                .filter_map(|part| part.parse::<u8>().ok())
                .collect::<Vec<_>>();
            String::from_utf8(bytes).unwrap_or_default()
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
        "timeFormat",
        Function::new(ctx.clone(), |epoch: i64, pattern: String| {
            format_epoch(epoch, &pattern)
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
    if let Some(session) = http_session {
        install_http_functions(ctx.clone(), &java, session)?;
    }
    globals.set("java", java).map_err(js_error)
}

fn install_http_functions<'js>(
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
        "ajax",
        Function::new(
            ctx,
            move |url: String, method: Option<String>, body: Option<String>| {
                blocking_http_request(
                    &ajax_http,
                    method.as_deref().unwrap_or("GET"),
                    &url,
                    body.filter(|value| !value.is_empty()),
                )
                .map_err(|error| {
                    rquickjs::Error::new_from_js_message("HTTP", "String", error.to_string())
                })
            },
        ),
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

#[derive(Clone, Debug, Default)]
struct JsHttpRequestOptions {
    method: String,
    body: Option<String>,
    headers: HashMap<String, String>,
    timeout_ms: Option<u64>,
}

fn parse_request_options<'js>(options: Object<'js>) -> Result<JsHttpRequestOptions, String> {
    let method = options
        .get::<_, Option<String>>("method")
        .map_err(|error| error.to_string())?
        .unwrap_or_else(|| "GET".into());
    let body = options
        .get::<_, Option<String>>("body")
        .map_err(|error| error.to_string())?;
    let timeout_ms = options
        .get::<_, Option<u64>>("timeout")
        .map_err(|error| error.to_string())?;
    let headers = options
        .get::<_, Option<Object>>("headers")
        .map_err(|error| error.to_string())?
        .map(|headers| {
            headers
                .props::<String, String>()
                .collect::<Result<HashMap<_, _>, _>>()
                .map_err(|error| error.to_string())
        })
        .transpose()?
        .unwrap_or_default();
    Ok(JsHttpRequestOptions {
        method,
        body,
        headers,
        timeout_ms,
    })
}

fn blocking_http_request_with_options(
    session: &JsHttpSession,
    raw_url: &str,
    options: JsHttpRequestOptions,
) -> Result<String, AppError> {
    let context = &session.context;
    let url = reqwest::Url::parse(raw_url)
        .or_else(|_| reqwest::Url::parse(&context.base_url).and_then(|base| base.join(raw_url)))
        .map_err(AppError::network)?;
    let method = reqwest::Method::from_bytes(options.method.as_bytes())
        .map_err(|error| AppError::InvalidArgument(format!("HTTP method 无效: {error}")))?;
    let mut request = session.client.request(method, url.clone());
    if let Some(timeout_ms) = options.timeout_ms.filter(|value| *value > 0) {
        request = request.timeout(Duration::from_millis(timeout_ms.min(120_000)));
    }
    if !context.session_expired {
        if let Some(token) = context.access_token.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(cookie) = context.session_cookie.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::COOKIE, cookie);
        }
    } else {
        tracing::debug!(target: "network", "JS source session expired; omitting credentials");
    }
    if let Some(script) = context.sign_script.as_deref() {
        if let Some(signature) = evaluate_sign_script(script, url.as_str()) {
            request = request.header("x-signature", signature);
        }
    }
    if let Some(raw) = context.headers.as_deref() {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, JsonValue>>(raw) {
            for (name, value) in headers {
                request = request.header(
                    name,
                    value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string()),
                );
            }
        }
    }
    for (name, value) in options.headers {
        request = request.header(name, value);
    }
    if let Some(body) = options.body {
        request = request.body(body);
    }
    let response = request.send().map_err(AppError::network)?;
    let status = response.status();
    let headers = response
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            Some((
                name.as_str().to_ascii_lowercase(),
                value.to_str().ok()?.to_owned(),
            ))
        })
        .collect::<HashMap<_, _>>();
    if let Ok(mut previous) = session.response.lock() {
        *previous = Some(JsHttpResponse {
            status: status.as_u16(),
            headers,
        });
    }
    let text = response.text().map_err(AppError::network)?;
    if !status.is_success() {
        return Err(AppError::Network(format!("HTTP {status}: {text}")));
    }
    Ok(text)
}

fn build_js_http_session(context: JsHttpContext) -> Result<JsHttpSession, AppError> {
    static CLIENT: OnceLock<Result<reqwest::blocking::Client, String>> = OnceLock::new();
    let client = CLIENT
        .get_or_init(|| {
            reqwest::blocking::Client::builder()
                .user_agent("Reader Desktop/0.1")
                .cookie_store(true)
                .timeout(Duration::from_secs(15))
                .build()
                .map_err(|error| error.to_string())
        })
        .as_ref()
        .map_err(|error| AppError::Network(error.clone()))?
        .clone();
    Ok(JsHttpSession {
        client,
        context,
        response: Arc::new(Mutex::new(None)),
    })
}

fn format_epoch(epoch: i64, pattern: &str) -> String {
    // Keep this dependency-free and deterministic. These are the tokens used
    // by the common legado timeFormat calls; unknown tokens are preserved.
    let seconds = epoch / 1000;
    let days = seconds.div_euclid(86_400);
    let day_seconds = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = day_seconds / 3_600;
    let minute = (day_seconds % 3_600) / 60;
    let second = day_seconds % 60;
    pattern
        .replace("yyyy", &format!("{year:04}"))
        .replace("MM", &format!("{month:02}"))
        .replace("dd", &format!("{day:02}"))
        .replace("HH", &format!("{hour:02}"))
        .replace("mm", &format!("{minute:02}"))
        .replace("ss", &format!("{second:02}"))
}

// Howard Hinnant's Gregorian civil-date conversion, valid for Unix epochs.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    (y + i64::from(m <= 2), m, d)
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
    use std::io::{Read, Write};
    use std::net::TcpListener;

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

    #[tokio::test]
    async fn blocking_execution_with_http_context_is_safe_inside_async_context() {
        let value = QuickJsRuntime::default()
            .execute_blocking(
                "result",
                JsContext {
                    result: "safe".into(),
                    http: Some(JsHttpContext::default()),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(value, JsValue::String("safe".into()));
    }

    #[tokio::test]
    async fn exposes_encoding_and_time_helpers() {
        let runtime = QuickJsRuntime::default();
        let value = runtime
            .execute(
                "java.hexEncodeToString('Hi') + '|' + java.hexDecodeToString('4869') + '|' + java.md5Encode('hello') + '|' + java.strToBytes('Hi') + '|' + java.bytesToStr('72,105') + '|' + java.timeFormat(0, 'yyyy-MM-dd HH:mm:ss')",
                JsContext::default(),
            )
            .await
            .unwrap();
        assert_eq!(
            value,
            JsValue::String(
                "4869|Hi|5d41402abc4b2a76b9719d911017c592|72,105|Hi|1970-01-01 00:00:00".into()
            )
        );
    }

    #[tokio::test]
    async fn exposes_authentication_state_to_source_scripts() {
        let runtime = QuickJsRuntime::default();
        let value = runtime
            .execute(
                "java.sessionState() + ':' + java.isAuthenticated()",
                JsContext {
                    http: Some(JsHttpContext {
                        access_token: Some("token".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("authenticated:true".into()));
    }

    #[tokio::test]
    async fn exposes_expired_state_without_authenticating_scripts() {
        let runtime = QuickJsRuntime::default();
        let value = runtime
            .execute(
                "java.sessionState() + ':' + java.isAuthenticated()",
                JsContext {
                    http: Some(JsHttpContext {
                        access_token: Some("token".into()),
                        session_expired: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("expired:false".into()));
    }

    #[tokio::test]
    async fn request_options_and_response_metadata_are_available_to_scripts() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_millis(500)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut buffer = [0_u8; 4096];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(size) => {
                        bytes.extend_from_slice(&buffer[..size]);
                        let request = String::from_utf8_lossy(&bytes);
                        let expected = request
                            .split("\r\n\r\n")
                            .next()
                            .and_then(|headers| {
                                headers.lines().find_map(|line| {
                                    line.strip_prefix("Content-Length:")?
                                        .trim()
                                        .parse::<usize>()
                                        .ok()
                                })
                            })
                            .unwrap_or_default();
                        if let Some(body) = request.split_once("\r\n\r\n").map(|(_, body)| body) {
                            if body.len() >= expected {
                                break;
                            }
                        }
                    }
                }
            }
            let request = String::from_utf8_lossy(&bytes);
            assert!(request.to_ascii_lowercase().contains("x-test: yes"));
            assert!(request.contains("hello"));
            stream
                .write_all(
                    b"HTTP/1.1 201 Created\r\nX-Trace: runtime\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nok",
                )
                .unwrap();
        });
        let script = format!(
            "java.request('http://{address}', {{method:'POST', body:'hello', headers:{{'X-Test':'yes'}}, timeout:2000}}) + '|' + java.responseStatus() + '|' + java.responseHeader('x-trace') + '|' + JSON.parse(java.responseHeaders())['content-type']"
        );
        let value = QuickJsRuntime::default()
            .execute(
                &script,
                JsContext {
                    http: Some(JsHttpContext::default()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        server.join().unwrap();
        assert_eq!(value, JsValue::String("ok|201|runtime|text/plain".into()));
    }
}
