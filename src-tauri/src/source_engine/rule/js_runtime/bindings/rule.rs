fn evaluate_script<'js>(ctx: Ctx<'js>, script: &str) -> Result<JsValue, AppError> {
    let script = normalize_js_statement_boundaries(script);
    let mut sloppy = EvalOptions::default();
    sloppy.strict = false;
    // Most legado rules are expressions (`result.trim()`). Try that first;
    // statement blocks use an explicit `return` and are evaluated second.
    let expression = format!("JSON.stringify(({script}))");
    let serialized = match ctx.eval_with_options::<String, _>(expression.as_str(), sloppy) {
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
            let mut direct_options = EvalOptions::default();
            direct_options.strict = false;
            let direct = ctx
                .eval_with_options::<String, _>(program.as_str(), direct_options)
                .catch(&ctx);
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
                    let mut block_options = EvalOptions::default();
                    block_options.strict = false;
                    ctx.eval_with_options::<String, _>(block.as_str(), block_options)
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
    async fn exposes_legado_src_and_title_aliases() {
        let value = QuickJsRuntime::default()
            .execute(
                "src + '|' + title",
                JsContext {
                    result: "body".into(),
                    src: Some("https://example.test/chapter".into()),
                    title: Some("第一章".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            value,
            JsValue::String("https://example.test/chapter|第一章".into())
        );
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
