//! Sandboxed JavaScript execution for legado `<js>` rules.
//!
//! The runtime is intentionally kept behind a small trait.  Rule evaluation
//! can therefore use a real QuickJS implementation today while tests and
//! future WebView-backed implementations can provide another runtime.

use super::{engine::evaluate, jsoup::Extraction, model::RuleContext};
use crate::error::AppError;
use crate::infrastructure::http::request::{evaluate_sign_script, user_agent};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac as HmacMac};
use md5::{Digest, Md5};
use rquickjs::{CatchResultExt, Context, Ctx, Function, Object, Runtime};
use serde_json::Value as JsonValue;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};
use uuid::Uuid;

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
    let get_values = Arc::clone(variables);
    let http_session = http
        .clone()
        .map(build_js_http_session)
        .transpose()?
        .map(Arc::new);
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
    let get_element_input = rule_input;
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
        "digestHex",
        Function::new(ctx.clone(), |data: String, algorithm: String| {
            digest_bytes(&algorithm, data.as_bytes())
                .map(|bytes| bytes_to_hex(&bytes))
                .map_err(|error| rquickjs::Error::new_from_js_message("Digest", "String", error))
        }),
    )
    .map_err(js_error)?;
    java.set(
        "HMacHex",
        Function::new(
            ctx.clone(),
            |data: String, algorithm: String, key: String| {
                hmac_bytes(&algorithm, key.as_bytes(), data.as_bytes())
                    .map(|bytes| bytes_to_hex(&bytes))
                    .map_err(|error| rquickjs::Error::new_from_js_message("HMac", "String", error))
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "HMacBase64",
        Function::new(
            ctx.clone(),
            |data: String, algorithm: String, key: String| {
                hmac_bytes(&algorithm, key.as_bytes(), data.as_bytes())
                    .map(|bytes| STANDARD.encode(bytes))
                    .map_err(|error| rquickjs::Error::new_from_js_message("HMac", "String", error))
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "randomUUID",
        Function::new(ctx.clone(), || Uuid::new_v4().to_string()),
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
        "timeFormatRaw",
        Function::new(ctx.clone(), |epoch: f64, pattern: String| {
            format_epoch(epoch as i64, &pattern)
        }),
    )
    .map_err(js_error)?;
    java.set(
        "timeFormatUtcRaw",
        Function::new(
            ctx.clone(),
            |epoch: f64, pattern: String, offset_hours: i32| {
                let offset_ms = i64::from(offset_hours) * 3_600 * 1_000;
                format_epoch(epoch as i64 + offset_ms, &pattern)
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "toNumChapter",
        Function::new(ctx.clone(), |value: String| {
            normalize_chapter_numbers(&value)
        }),
    )
    .map_err(js_error)?;
    java.set("toast", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("longToast", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set(
        "getStringList",
        Function::new(ctx.clone(), |rule: String| vec![rule]),
    )
    .map_err(js_error)?;
    java.set("t2s", Function::new(ctx.clone(), |value: String| value))
        .map_err(js_error)?;
    java.set(
        "htmlFormat",
        Function::new(ctx.clone(), |value: String| value),
    )
    .map_err(js_error)?;
    java.set("startBrowser", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("startBrowserAwait", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("openUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("setContent", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshBookUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshTocUrl", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set("refreshExplore", Function::new(ctx.clone(), || {}))
        .map_err(js_error)?;
    java.set(
        "log",
        Function::new(ctx.clone(), |value: String| {
            tracing::debug!(target: "source", "JS: {value}");
        }),
    )
    .map_err(js_error)?;
    install_source_compat(
        ctx.clone(),
        &globals,
        variables,
        &base_url_value,
        http.clone(),
    )?;
    // Make the object visible while HTTP compatibility wrappers are installed;
    // the wrappers are ordinary JavaScript functions that delegate to the
    // fixed-arity native entry points below.
    globals.set("java", java.clone()).map_err(js_error)?;
    if let Some(session) = http_session {
        install_http_functions(ctx.clone(), &java, session)?;
    }
    // Legado overloads these helpers (one, two, or three arguments). Native
    // QuickJS functions are fixed-arity, so expose small JS shims that coerce
    // omitted/string arguments before calling the typed bridge.
    ctx.eval::<(), _>(
        r#"
        java.timeFormat = function(epoch, pattern) {
            epoch = Number(epoch);
            if (!isFinite(epoch)) epoch = 0;
            pattern = pattern == null ? 'yyyy-MM-dd HH:mm:ss' : String(pattern);
            return java.timeFormatRaw(epoch, pattern);
        };
        java.timeFormatUTC = function(epoch, pattern, offset) {
            epoch = Number(epoch);
            if (!isFinite(epoch)) epoch = 0;
            pattern = pattern == null ? 'yyyy-MM-dd HH:mm:ss' : String(pattern);
            offset = offset == null ? 0 : Number(offset);
            if (!isFinite(offset)) offset = 0;
            return java.timeFormatUtcRaw(epoch, pattern, offset);
        };
        "#,
    )
    .map_err(js_error)?;
    globals.set("java", java).map_err(js_error)
}

fn install_source_compat<'js>(
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

fn nested_rule_values(
    rule: &str,
    input: &str,
    want: Extraction,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    http: Option<JsHttpContext>,
) -> Result<Vec<String>, String> {
    let snapshot = variables
        .lock()
        .map(|values| values.clone())
        .unwrap_or_default();
    let mut context = RuleContext::new(snapshot);
    if let Some(http) = http {
        context.with_http(http);
    }
    let values = evaluate(rule, input, want, &mut context).map_err(|error| error.to_string())?;
    if let Ok(mut shared) = variables.lock() {
        shared.extend(context.snapshot());
    }
    Ok(values)
}

fn rule_js_error(error: String) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Rule", "String", error)
}

fn stable_android_id(base_url: &str) -> String {
    let mut digest = Md5::new();
    digest.update(b"reader-desktop/android-id/");
    digest.update(base_url.as_bytes());
    format!("{:x}", digest.finalize())[..16].to_owned()
}

fn canonical_algorithm(algorithm: &str) -> String {
    algorithm
        .trim()
        .to_ascii_uppercase()
        .replace(['-', '_', '/'], "")
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn digest_bytes(algorithm: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    match canonical_algorithm(algorithm).as_str() {
        "MD5" => Ok(Md5::digest(data).to_vec()),
        "SHA1" => Ok(Sha1::digest(data).to_vec()),
        "SHA256" => Ok(Sha256::digest(data).to_vec()),
        "SHA512" => Ok(Sha512::digest(data).to_vec()),
        other => Err(format!("unsupported digest algorithm: {other}")),
    }
}

fn hmac_bytes(algorithm: &str, key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let normalized = canonical_algorithm(algorithm);
    let digest_name = normalized.strip_prefix("HMAC").unwrap_or(&normalized);
    match digest_name {
        "MD5" => {
            let mut mac = Hmac::<Md5>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA1" => {
            let mut mac = Hmac::<Sha1>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA256" => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA512" => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        other => Err(format!("unsupported HMAC algorithm: {other}")),
    }
}

fn normalize_chapter_numbers(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chinese = String::new();
    let flush = |output: &mut String, chinese: &mut String| {
        if chinese.is_empty() {
            return;
        }
        if let Some(number) = chinese_number(chinese) {
            output.push_str(&number.to_string());
        } else {
            output.push_str(chinese);
        }
        chinese.clear();
    };
    for character in value.chars() {
        if chinese_digit(character).is_some() || chinese_unit(character).is_some() {
            chinese.push(character);
        } else {
            flush(&mut output, &mut chinese);
            output.push(character);
        }
    }
    flush(&mut output, &mut chinese);
    output
}

fn chinese_number(value: &str) -> Option<u64> {
    let has_unit = value
        .chars()
        .any(|character| chinese_unit(character).is_some());
    if !has_unit {
        return value.chars().try_fold(0_u64, |number, character| {
            Some(number * 10 + chinese_digit(character)?)
        });
    }
    let mut total = 0_u64;
    let mut section = 0_u64;
    let mut digit = 0_u64;
    for character in value.chars() {
        if let Some(value) = chinese_digit(character) {
            digit = value;
            continue;
        }
        let unit = chinese_unit(character)?;
        if unit == 10_000 {
            section = (section + digit) * unit;
            total += section;
            section = 0;
        } else {
            section += digit.max(1) * unit;
        }
        digit = 0;
    }
    Some(total + section + digit)
}

fn chinese_digit(character: char) -> Option<u64> {
    match character {
        '零' | '〇' => Some(0),
        '一' => Some(1),
        '二' | '两' => Some(2),
        '三' => Some(3),
        '四' => Some(4),
        '五' => Some(5),
        '六' => Some(6),
        '七' => Some(7),
        '八' => Some(8),
        '九' => Some(9),
        _ => None,
    }
}

fn chinese_unit(character: char) -> Option<u64> {
    match character {
        '十' => Some(10),
        '百' => Some(100),
        '千' => Some(1_000),
        '万' => Some(10_000),
        _ => None,
    }
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
    // Set per request rather than on the cached client: the client is a
    // `OnceLock` and would otherwise freeze whatever UA was configured at the
    // first `java.ajax` call. Source-level and call-level headers below can
    // still override it.
    let mut request = session
        .client
        .request(method, url.clone())
        .header(reqwest::header::USER_AGENT, user_agent());
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
    let script = normalize_js_statement_boundaries(script);
    // Most legado rules are expressions (`result.trim()`). Try that first;
    // statement blocks use an explicit `return` and are evaluated second.
    let expression = format!("JSON.stringify(({script}))");
    let serialized = match ctx.eval::<String, _>(expression.as_str()) {
        Ok(serialized) => serialized,
        Err(_) => {
            // Expression failure is expected for statement-style rules. A
            // direct eval preserves JavaScript's completion value, including
            // scripts whose statements are separated only by newlines (a
            // common Legado style). The old function wrapper tried to infer
            // the final expression from semicolons and turned scripts such as
            // `headerSign=...\nparamSign=...\nurl` into an invalid single
            // `return (...)` expression.
            let _ = ctx.catch();
            let source = serde_json::to_string(&script)
                .map_err(|error| AppError::Parse(format!("JavaScript 编码失败: {error}")))?;
            let program = format!(
                "JSON.stringify((function() {{ return eval({source}); }})()) || JSON.stringify(result)"
            );
            let direct = ctx.eval::<String, _>(program.as_str()).catch(&ctx);
            match direct {
                Ok(serialized) => serialized,
                Err(_direct_error) if has_top_level_return(&script) => {
                    // A script containing an explicit top-level `return`
                    // cannot run through eval. Keep the function-wrapper
                    // fallback for those exports and report its real error.
                    let block_script = split_last_statement(&script)
                        .map(|(body, tail)| format!("{body}; return ({tail});"))
                        .unwrap_or_else(|| script.to_owned());
                    let block = format!(
                        "JSON.stringify((function() {{ {block_script} }})()) || JSON.stringify(result)"
                    );
                    ctx.eval::<String, _>(block.as_str())
                        .catch(&ctx)
                        .map_err(js_error)?
                }
                Err(direct_error) => return Err(js_error(direct_error)),
            }
        }
    };
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

/// Legado exports frequently omit semicolons between assignment statements,
/// relying on Android's JavaScript engine to insert them at line boundaries.
/// QuickJS is stricter for a few otherwise-valid continuations (notably
/// `headerSign=...` followed by `paramSign=...`). Add terminators only where a
/// new declaration/assignment clearly starts, leaving multiline expressions,
/// control headers, and object/function bodies untouched.
fn normalize_js_statement_boundaries(script: &str) -> String {
    let script = declare_implicit_assignments(script);
    let mut lines: Vec<String> = script.lines().map(declare_implicit_assignment).collect();
    if lines.len() < 2 {
        return script;
    }
    for index in 1..lines.len() {
        if should_insert_statement_semicolon(&lines[index - 1], &lines[index]) {
            let previous = lines[index - 1].trim_end();
            if !previous.ends_with(';') {
                lines[index - 1].push(';');
            }
        }
    }
    lines.join("\n")
}

fn declare_implicit_assignments(script: &str) -> String {
    let mut names = Vec::new();
    let bytes = script.as_bytes();
    let mut index = 0;
    let mut quote = None;
    let mut escaped = false;
    let mut statement_start = true;
    while index < bytes.len() {
        let character = bytes[index] as char;
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if let Some(active) = quote {
            if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            index += 1;
            continue;
        }
        if matches!(character, '\'' | '"' | '`') {
            quote = Some(character);
            index += 1;
            continue;
        }
        if character == '/' && bytes.get(index + 1) == Some(&b'/') {
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            statement_start = true;
            continue;
        }
        if character == ';' || character == '\n' || character == '{' {
            statement_start = true;
            index += 1;
            continue;
        }
        if character.is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if statement_start
            && (character == '_' || character == '$' || character.is_ascii_alphabetic())
        {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index] == b'_'
                    || bytes[index] == b'$'
                    || bytes[index].is_ascii_alphanumeric())
            {
                index += 1;
            }
            let name = &script[start..index];
            let mut cursor = index;
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            let declared = matches!(
                name,
                "var" | "let" | "const" | "function" | "if" | "for" | "while" | "return"
            );
            let known_global = matches!(
                name,
                "result"
                    | "url"
                    | "key"
                    | "baseUrl"
                    | "page"
                    | "java"
                    | "source"
                    | "book"
                    | "chapter"
                    | "content"
            );
            if !declared
                && !known_global
                && bytes.get(cursor) == Some(&b'=')
                && bytes.get(cursor + 1) != Some(&b'=')
                && bytes.get(cursor + 1) != Some(&b'>')
                && !names.iter().any(|existing| existing == name)
            {
                names.push(name.to_owned());
            }
            statement_start = false;
            continue;
        }
        statement_start = false;
        index += 1;
    }
    if names.is_empty() {
        script.to_owned()
    } else {
        format!("var {};\n{script}", names.join(", "))
    }
}

fn declare_implicit_assignment(line: &str) -> String {
    let trimmed = line.trim_start();
    let Some(equal) = trimmed.find('=') else {
        return line.to_owned();
    };
    if trimmed
        .as_bytes()
        .get(equal + 1)
        .is_some_and(|next| matches!(next, b'=' | b'>'))
    {
        return line.to_owned();
    }
    let name = trimmed[..equal].trim_end();
    if name.ends_with(['!', '<', '>']) {
        return line.to_owned();
    }
    let known_global = matches!(name, "result" | "url" | "key" | "baseUrl" | "page" | "java");
    let identifier = !name.is_empty()
        && name.chars().enumerate().all(|(index, character)| {
            character == '_'
                || character == '$'
                || character.is_ascii_alphanumeric() && (index > 0 || !character.is_ascii_digit())
        });
    if !known_global && identifier {
        let indent = &line[..line.len() - trimmed.len()];
        format!("{indent}var {trimmed}")
    } else {
        line.to_owned()
    }
}

fn should_insert_statement_semicolon(previous: &str, current: &str) -> bool {
    let previous = previous.trim();
    let current = current.trim_start();
    if previous.is_empty()
        || previous.starts_with("//")
        || previous.ends_with([
            '{', '(', '[', ',', '.', ':', '?', '=', '+', '-', '*', '/', '&', '|',
        ])
        || ["else", "catch", "finally"]
            .iter()
            .any(|keyword| current.starts_with(keyword))
    {
        return false;
    }
    if is_control_header(previous) {
        return false;
    }
    starts_assignment_or_declaration(current)
        || matches!(current.chars().next(), Some('\'' | '"' | '`'))
}

fn is_control_header(line: &str) -> bool {
    ["if", "for", "while", "switch", "with", "catch"]
        .iter()
        .any(|keyword| {
            line.strip_prefix(keyword)
                .is_some_and(|rest| rest.trim_start().starts_with('('))
        })
}

fn starts_assignment_or_declaration(line: &str) -> bool {
    let line = line.trim_start();
    if ["var ", "let ", "const "]
        .iter()
        .any(|prefix| line.starts_with(prefix))
    {
        return true;
    }
    let Some(equal) = line.find('=') else {
        return false;
    };
    let lhs = line[..equal].trim_end();
    if lhs.is_empty() || lhs.ends_with(['=', '!', '<', '>']) {
        return false;
    }
    lhs.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '$' | '.' | '[' | ']' | '\'' | '"')
            || character.is_ascii_whitespace()
    })
}

fn split_last_statement(script: &str) -> Option<(&str, &str)> {
    let mut quote = None;
    let mut escaped = false;
    let mut depth = 0_i32;
    let mut split = None;
    for (index, character) in script.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote.is_some() {
            if character == '\\' {
                escaped = true;
            } else if Some(character) == quote {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' | '`' => quote = Some(character),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ';' if depth == 0 => split = Some(index),
            _ => {}
        }
    }
    let index = split?;
    let tail = script[index + 1..].trim();
    (!tail.is_empty()).then_some((&script[..index], tail))
}

fn has_top_level_return(script: &str) -> bool {
    let mut depth = 0_i32;
    let mut quote = None;
    let mut escaped = false;
    let bytes = script.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let character = bytes[index] as char;
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if let Some(active) = quote {
            if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            index += 1;
            continue;
        }
        if matches!(character, '\'' | '"' | '`') {
            quote = Some(character);
            index += 1;
            continue;
        }
        match character {
            '{' | '(' | '[' => depth += 1,
            '}' | ')' | ']' => depth -= 1,
            _ => {}
        }
        if depth == 0
            && (index == 0 || bytes[index - 1].is_ascii_whitespace() || bytes[index - 1] == b';')
            && script[index..].starts_with("return")
            && script[index + 6..]
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_whitespace() || next == ';')
        {
            return true;
        }
        index += 1;
    }
    false
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
    async fn reports_the_javascript_exception_message() {
        let error = QuickJsRuntime::default()
            .execute("missingHelper()", JsContext::default())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("missingHelper"), "{error}");
        assert!(!error.contains("Exception generated by QuickJS"), "{error}");
    }

    #[tokio::test]
    async fn executes_newline_separated_statement_scripts() {
        let script = r#"
signKey='secret'
headers={'platform':'android'}
params={'page':page,'wd':key}
var encode = function(values) {
  return Object.keys(values).map(k=>k+'='+values[k]).join('&')
};
headerSign=java.md5Encode(encode(headers)+signKey)
paramSign=java.md5Encode(encode(params)+signKey)
headers['sign']=headerSign
params['sign']=paramSign
'/search?'+encode(params)+','+JSON.stringify({'headers':headers})
"#;
        let value = QuickJsRuntime::default()
            .execute(
                script,
                JsContext {
                    key: Some("斗破苍穹".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        match value {
            JsValue::String(value) => {
                assert!(value.starts_with("/search?page=1&wd="), "{value}");
                assert!(value.contains("headers"), "{value}");
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[tokio::test]
    async fn executes_real_qimao_search_script() {
        let script = r#"
sign_key='d3dGiJc651gSQ8w1'

headers={'app-version':'51110','platform':'android','reg':'0','AUTHORIZATION':'','application-id':'com.****.reader','net-env':'1','channel':'unknown','qm-params':''}

params={'gender':'3','imei_ip':'2937357107','page':page,'wd':key}

var urlEncode = function (param, key, encode) {
  if(param==null) return '';
  var paramStr = '';
  var t = typeof (param);
  if (t == 'string' || t == 'number' || t == 'boolean') {
    paramStr += '&' + key + '=' + ((encode==null||encode) ? encodeURIComponent(param) : param);
  } else {
    for (var i in param) {
      var k = key == null ? i : key + (param instanceof Array ? '[' + i + ']' : '.' + i);
      paramStr += urlEncode(param[i], k, encode);
    }
  }
  return paramStr;
};

headerSign=String(java.md5Encode(Object.keys(headers).sort().reduce((pre,n)=>pre+n+'='+headers[n],'' )+sign_key))
paramSign=String(java.md5Encode(Object.keys(params).sort().reduce((pre,n)=>pre+n+'='+params[n],'' )+sign_key))
headers['sign']=headerSign
params['sign']=paramSign
body=urlEncode(params)

"/api/v5/search/words?" +body+","+java.put("headers",JSON.stringify({"headers":headers}))"#;
        let value = QuickJsRuntime::default()
            .execute(
                script,
                JsContext {
                    key: Some("斗破苍穹".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(
            matches!(value, JsValue::String(value) if value.starts_with("/api/v5/search/words?"))
        );
    }

    #[tokio::test]
    async fn executes_statement_scripts_starting_with_control_flow() {
        let value = QuickJsRuntime::default()
            .execute(
                "if (key) { result = key; }\nresult",
                JsContext {
                    key: Some("ok".into()),
                    result: "before".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("ok".into()));
    }

    #[tokio::test]
    async fn executes_legado_control_flow_after_function_declarations() {
        let value = QuickJsRuntime::default()
            .execute(
                r#"
var category = function () { return 'category'; };
var tag = function () { return 'tag'; };
if (baseUrl.match(/category/)) {
  category()
} else {
  tag()
}
"#,
                JsContext {
                    base_url: Some("https://example.test/tag".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("tag".into()));
    }

    #[tokio::test]
    async fn accepts_legado_one_argument_ajax_calls() {
        let error = QuickJsRuntime::default()
            .execute(
                "java.ajax('not a valid URL')",
                JsContext {
                    http: Some(JsHttpContext::default()),
                    ..Default::default()
                },
            )
            .await
            .unwrap_err()
            .to_string();
        assert!(!error.contains("argument(s)"), "{error}");
        assert!(error.contains("network error"), "{error}");
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
        assert_eq!(
            QuickJsRuntime::default()
                .execute("java.timeFormat(0)", JsContext::default())
                .await
                .unwrap(),
            JsValue::String("1970-01-01 00:00:00".into())
        );
        assert_eq!(
            QuickJsRuntime::default()
                .execute(
                    "java.timeFormatUTC(0, 'yyyy-MM-dd HH:mm:ss', 8)",
                    JsContext::default()
                )
                .await
                .unwrap(),
            JsValue::String("1970-01-01 08:00:00".into())
        );
    }

    #[tokio::test]
    async fn exposes_digest_hmac_and_uuid_helpers() {
        let value = QuickJsRuntime::default()
            .execute(
                "java.digestHex('hello', 'SHA-256') + '|' + java.HMacHex('hello', 'HmacSHA256', 'key') + '|' + java.HMacBase64('hello', 'HmacSHA1', 'key') + '|' + java.randomUUID()",
                JsContext::default(),
            )
            .await
            .unwrap();
        let JsValue::String(value) = value else {
            panic!("expected string result")
        };
        let pieces = value.split('|').collect::<Vec<_>>();
        assert_eq!(
            pieces[0],
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(pieces[1].len(), 64);
        assert_eq!(pieces[2].len(), 28);
        assert!(Uuid::parse_str(pieces[3]).is_ok());
    }

    #[tokio::test]
    async fn exposes_nested_rule_helpers() {
        let runtime = QuickJsRuntime::default();
        let value = runtime
            .execute(
                "java.getString('$.book.name')",
                JsContext {
                    result: r#"{"book":{"name":"斗破苍穹"}}"#.into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("斗破苍穹".into()));

        let value = runtime
            .execute(
                "java.getElements('tag.li').length + ':' + java.getElement('tag.li')",
                JsContext {
                    result: "<ul><li>一</li><li>二</li></ul>".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(
            matches!(value, JsValue::String(value) if value.starts_with("2:") && value.contains("<li>一</li>"))
        );
    }

    #[tokio::test]
    async fn normalizes_chapter_numbers_and_ignores_android_toasts() {
        let value = QuickJsRuntime::default()
            .execute(
                "java.toast('ignored'); java.longToast('ignored'); java.toNumChapter('第一百二十三章')",
                JsContext::default(),
            )
            .await
            .unwrap();
        assert_eq!(value, JsValue::String("第123章".into()));
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
    async fn exposes_a_stable_desktop_android_id_compatibility_helper() {
        let runtime = QuickJsRuntime::default();
        let first = runtime
            .execute(
                "java.androidId()",
                JsContext {
                    base_url: Some("https://example.test".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let second = runtime
            .execute(
                "java.androidId()",
                JsContext {
                    base_url: Some("https://example.test".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(first, second);
        assert!(matches!(first, JsValue::String(value) if value.len() == 16));
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
