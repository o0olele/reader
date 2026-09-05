#[allow(clippy::too_many_arguments)]
fn install_globals<'js>(
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
    globals.set("time", format_epoch(0, "yyyy-MM-dd HH:mm:ss")).map_err(js_error)?;
    let alias_url = base_url_value.clone();
    globals
        .set("getUrl", Function::new(ctx.clone(), move || alias_url.clone()))
        .map_err(js_error)?;
    let alias_uuid = Uuid::new_v4().to_string();
    globals
        .set("uuid", Function::new(ctx.clone(), move || alias_uuid.clone()))
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
    // Common legado aliases used by older source scripts. Keep coercion
    // permissive, matching Rhino's Java helper behavior instead of raising
    // a QuickJS type error for missing or non-string values.
    java.set(
        "decodeURI",
        Function::new(ctx.clone(), |value: String| {
            urlencoding::decode(&value)
                .map(|decoded| decoded.into_owned())
                .unwrap_or(value)
        }),
    )
    .map_err(js_error)?;
    java.set(
        "toInt",
        Function::new(ctx.clone(), |value: String| {
            value.trim().parse::<i64>().unwrap_or_default()
        }),
    )
    .map_err(js_error)?;
    java.set(
        "toBoolean",
        Function::new(ctx.clone(), |value: String| {
            matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "1" | "yes")
        }),
    )
    .map_err(js_error)?;
    java.set(
        "isNull",
        Function::new(ctx.clone(), |value: Option<String>| value.is_none()),
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
