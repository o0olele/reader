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

