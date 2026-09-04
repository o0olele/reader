use super::RequestSpec;
use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::request::{response_error, source_request_with_method},
    source_engine::rule::{JsContext, JsValue, QuickJsRuntime},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use encoding_rs::Encoding;

pub struct FetchedBytes {
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
}

pub fn prepare(
    client: &reqwest::Client,
    source: &BookSource,
    spec: &RequestSpec,
) -> Result<reqwest::Request, AppError> {
    let mut request = source_request_with_method(
        client,
        spec.url.as_str(),
        source,
        spec.method.clone(),
        spec.body.clone(),
    )?;
    if let Some(referer) = spec.origin.as_deref() {
        request = request.header(reqwest::header::REFERER, referer);
    }
    for (name, value) in &spec.headers {
        request = request.header(name, value);
    }
    request
        .build()
        .map_err(|error| AppError::Network(format!("请求构造失败: {error}")))
}

pub async fn send(
    client: &reqwest::Client,
    source: &BookSource,
    spec: &RequestSpec,
) -> Result<reqwest::Response, AppError> {
    super::rate_limit::wait(source).await;
    let mut last_error = String::new();
    for attempt in 0..=spec.retry {
        let started = std::time::Instant::now();
        tracing::debug!(target: "network", url = %spec.url, attempt, "sending source request");
        let request = prepare(client, source, spec)?;
        match client.execute(request).await {
            Ok(response) => {
                tracing::debug!(target: "network", url = %spec.url, status = response.status().as_u16(), elapsed_ms = started.elapsed().as_millis() as u64, "source response received");
                return Ok(response);
            }
            Err(error) => {
                tracing::warn!(target: "network", url = %spec.url, attempt, elapsed_ms = started.elapsed().as_millis() as u64, error = %error, "source request failed");
                last_error = if error.is_timeout() {
                    "连接超时，请检查网络或代理".into()
                } else if error.is_connect() {
                    format!("无法连接目标站点（{error}），请检查 DNS、防火墙或代理")
                } else if error.is_builder() {
                    format!("请求配置无效: {error}")
                } else {
                    error.to_string()
                };
                if attempt < spec.retry {
                    tokio::time::sleep(std::time::Duration::from_millis(
                        250 * (attempt as u64 + 1),
                    ))
                    .await;
                }
            }
        }
    }
    Err(AppError::Network(last_error))
}

pub async fn decode_text(
    response: reqwest::Response,
    spec: &RequestSpec,
    source: &BookSource,
) -> Result<String, AppError> {
    let bytes = response.bytes().await.map_err(AppError::network)?;
    decode_text_bytes(bytes.as_ref(), spec, source)
}

/// Decodes a response body obtained outside reqwest (for example from the
/// authenticated WebView) using the same charset and legado body script rules.
pub fn decode_text_bytes(
    bytes: &[u8],
    spec: &RequestSpec,
    source: &BookSource,
) -> Result<String, AppError> {
    let text = match spec.charset.as_deref() {
        Some(label) => Encoding::for_label(label.as_bytes())
            .ok_or_else(|| AppError::Parse(format!("不支持的响应字符集: {label}")))?
            .decode(bytes)
            .0
            .into_owned(),
        None => String::from_utf8(bytes.to_vec())
            .map_err(|error| AppError::Parse(format!("响应不是 UTF-8: {error}")))?,
    };
    apply_body_script(text, spec, source)
}

/// Applies a legado body script to text that has already been decoded by the
/// browser. WebView `XMLHttpRequest.responseText` is always a Unicode string,
/// so applying the configured charset a second time would corrupt GBK pages.
pub fn decode_text_string(
    text: String,
    spec: &RequestSpec,
    source: &BookSource,
) -> Result<String, AppError> {
    apply_body_script(text, spec, source)
}

fn apply_body_script(
    text: String,
    spec: &RequestSpec,
    source: &BookSource,
) -> Result<String, AppError> {
    let Some(script) = spec.body_js.as_deref() else {
        return Ok(text);
    };
    let value = QuickJsRuntime::default().execute_blocking(
        script,
        JsContext {
            result: text,
            url: Some(spec.url.to_string()),
            base_url: Some(source.base_url.clone()),
            http: Some(source.http_context()),
            ..Default::default()
        },
    )?;
    Ok(match value {
        JsValue::String(value) => value,
        JsValue::Number(value) => value.to_string(),
        JsValue::Boolean(value) => value.to_string(),
        JsValue::Null => String::new(),
        JsValue::Json(value) => value.to_string(),
    })
}

pub async fn fetch_bytes(
    client: &reqwest::Client,
    source: &BookSource,
    spec: &RequestSpec,
) -> Result<FetchedBytes, AppError> {
    if spec.url.scheme() == "data" {
        return decode_data_url(spec.url.as_str());
    }
    let response = send(client, source, spec).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(';').next().unwrap_or(value).to_owned());
    let bytes = response.bytes().await.map_err(AppError::network)?.to_vec();
    Ok(FetchedBytes {
        bytes,
        content_type,
    })
}

fn decode_data_url(value: &str) -> Result<FetchedBytes, AppError> {
    let (metadata, payload) = value
        .strip_prefix("data:")
        .and_then(|value| value.split_once(','))
        .ok_or_else(|| AppError::Parse("data URL 格式无效".into()))?;
    let encoded = metadata
        .split(';')
        .any(|part| part.eq_ignore_ascii_case("base64"));
    let bytes = if encoded {
        STANDARD
            .decode(payload)
            .map_err(|error| AppError::Parse(format!("data URL Base64 无效: {error}")))?
    } else {
        urlencoding::decode(payload)
            .map_err(|error| AppError::Parse(format!("data URL 内容无效: {error}")))?
            .as_bytes()
            .to_vec()
    };
    let content_type = metadata
        .split(';')
        .next()
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    Ok(FetchedBytes {
        bytes,
        content_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{BookSource, CatalogRule, InfoRule, SearchRule};
    use std::io::{Read, Write};
    use std::net::TcpListener;

    fn source(base_url: String) -> BookSource {
        BookSource {
            id: 1,
            name: "transport".into(),
            base_url,
            search_url: String::new(),
            explore_url: None,
            search_rule: SearchRule {
                item: "a".into(),
                title: "a".into(),
                author: None,
                cover: None,
                url: "a".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "GET".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            raw_rules: Default::default(),
        }
    }

    #[test]
    fn decodes_base64_data_urls() {
        let fetched = decode_data_url("data:text/plain;base64,aGVsbG8=").unwrap();
        assert_eq!(fetched.bytes, b"hello");
        assert_eq!(fetched.content_type.as_deref(), Some("text/plain"));
    }

    #[tokio::test]
    async fn sends_options_and_applies_body_javascript() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .unwrap();
            let mut bytes = [0_u8; 4096];
            let size = stream.read(&mut bytes).unwrap();
            let request = String::from_utf8_lossy(&bytes[..size]);
            assert!(request.starts_with("POST /chapter HTTP/1.1"));
            assert!(request.to_ascii_lowercase().contains("x-stage: content"));
            assert!(request
                .to_ascii_lowercase()
                .contains("referer: https://origin.example/"));
            assert!(request.contains("page=2"));
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello")
                .unwrap();
        });
        let source = source(format!("http://{address}/"));
        let spec = super::super::build(
            &source,
            r#"chapter,{"method":"POST","body":"page=2","headers":{"X-Stage":"content"},"origin":"https://origin.example/","bodyJs":"result.toUpperCase()"}"#,
            None,
            "正文 URL",
        )
        .unwrap();
        let response = send(&reqwest::Client::new(), &source, &spec).await.unwrap();
        let text = decode_text(response, &spec, &source).await.unwrap();
        server.join().unwrap();
        assert_eq!(text, "HELLO");
    }
}
