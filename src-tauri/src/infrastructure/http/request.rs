use crate::{domain::source::BookSource, error::AppError};
use sha2::{Digest, Sha256};

/// Builds a GET carrying the browser-ish headers, stored session credentials,
/// per-source custom headers and signature that book sources expect.
#[cfg(test)]
fn source_request(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
) -> Result<reqwest::RequestBuilder, AppError> {
    source_request_with_method(client, url, source, reqwest::Method::GET, None)
}

pub fn source_request_with_method(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
    method: reqwest::Method,
    body: Option<String>,
) -> Result<reqwest::RequestBuilder, AppError> {
    let referer = reqwest::Url::parse(url).ok().map(|parsed| {
        let mut origin = parsed;
        origin.set_path("/");
        origin.set_query(None);
        origin.set_fragment(None);
        origin.to_string()
    });
    let mut request = client.request(method, url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9,en;q=0.8")
        .header(reqwest::header::CACHE_CONTROL, "no-cache");
    if let Some(referer) = referer {
        request = request.header(reqwest::header::REFERER, referer);
    }
    if !source.session_expired() {
        if let Some(token) = source.access_token.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(cookie) = source.session_cookie.as_deref().filter(|v| !v.is_empty()) {
            request = request.header(reqwest::header::COOKIE, cookie);
        }
    } else {
        tracing::debug!(target: "network", source = %source.name, "source session expired; omitting credentials");
    }
    if let Some(raw) = source.header.as_deref() {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(raw)
        {
            for (name, value) in headers {
                request = request.header(
                    &name,
                    value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string()),
                );
            }
        } else {
            for line in raw
                .split(&['\n', '&'][..])
                .filter(|line| !line.trim().is_empty())
            {
                let Some((name, value)) = line.split_once(':') else {
                    continue;
                };
                request = request
                    .header(name.trim(), value.trim())
                    .try_clone()
                    .ok_or_else(|| {
                        AppError::InvalidArgument(format!("非法请求头: {}", name.trim()))
                    })?;
            }
        }
    }
    if let Some(script) = source.sign_script.as_deref() {
        if let Some(signature) = evaluate_sign_script(script, url) {
            request = request.header("X-Signature", signature);
        }
    }
    if let Some(body) = body {
        request = request.body(body);
    }
    Ok(request)
}

pub fn evaluate_sign_script(script: &str, url: &str) -> Option<String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();
    let expression = script
        .replace("{{url}}", url)
        .replace("{{timestamp}}", &timestamp);
    let inner = expression
        .trim()
        .trim_start_matches("return")
        .trim()
        .trim_end_matches(';')
        .trim()
        .strip_prefix("sha256(")?
        .strip_suffix(')')?;
    let mut hasher = Sha256::new();
    hasher.update(inner.as_bytes());
    Some(format!("{:x}", hasher.finalize()))
}

pub async fn response_error(response: reqwest::Response, source_name: &str) -> String {
    let status = response.status();
    let cloudflare_headers = if matches!(status.as_u16(), 403 | 503) {
        response
            .headers()
            .get("server")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.to_ascii_lowercase().contains("cloudflare"))
            || response.headers().contains_key("cf-mitigated")
            || response.headers().contains_key("cf-ray")
    } else {
        false
    };
    let detail = response.text().await.unwrap_or_default();
    let detail = detail.split_whitespace().collect::<Vec<_>>().join(" ");
    let lower = detail.to_ascii_lowercase();
    if cloudflare_headers
        || lower.contains("just a moment")
        || lower.contains("cf-chl-")
        || lower.contains("cloudflare")
        || lower.contains("challenge-platform")
        || lower.contains("enable javascript and cookies")
    {
        return format!("{source_name} 需要浏览器执行 JavaScript 验证（Cloudflare challenge），HTTP 客户端无法直接通过");
    }
    let detail = detail.chars().take(180).collect::<String>();
    if detail.is_empty() {
        format!("{source_name} 返回 HTTP {status}")
    } else {
        format!("{source_name} 返回 HTTP {status}: {detail}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{CatalogRule, InfoRule, SearchRule};

    fn test_source(sign_script: Option<&str>, header: Option<&str>) -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com/?q={{key}}".into(),
            explore_url: None,
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a::attr(href)".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a::attr(href)".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: header.map(str::to_owned),
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: sign_script.map(str::to_owned),
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            raw_rules: Default::default(),
        }
    }

    fn build(source: &BookSource, url: &str) -> reqwest::Request {
        source_request(&reqwest::Client::new(), url, source)
            .unwrap()
            .build()
            .unwrap()
    }

    /// B4 regression: signing used to sit after an early return in the `header`
    /// branch, so sources without custom headers were silently never signed.
    #[test]
    fn signs_requests_without_custom_headers() {
        let request = build(
            &test_source(Some("sha256({{url}})"), None),
            "https://example.com/chapter",
        );
        assert!(request.headers().contains_key("x-signature"));
    }

    #[test]
    fn signs_requests_that_also_carry_custom_headers() {
        let request = build(
            &test_source(Some("sha256({{url}})"), Some(r#"{"X-Api-Key":"abc"}"#)),
            "https://example.com/chapter",
        );
        assert!(request.headers().contains_key("x-signature"));
        assert_eq!(request.headers()["x-api-key"], "abc");
    }

    #[test]
    fn omits_the_signature_header_when_no_script_is_configured() {
        let request = build(&test_source(None, None), "https://example.com/chapter");
        assert!(!request.headers().contains_key("x-signature"));
    }

    #[test]
    fn derives_the_referer_from_the_request_origin() {
        let request = build(&test_source(None, None), "https://example.com/a/b?c=1");
        assert_eq!(
            request.headers()[reqwest::header::REFERER],
            "https://example.com/"
        );
    }

    #[test]
    fn sends_the_stored_session_credentials() {
        let mut source = test_source(None, None);
        source.access_token = Some("tok".into());
        source.session_cookie = Some("sid=1".into());
        let request = build(&source, "https://example.com/chapter");
        assert_eq!(
            request.headers()[reqwest::header::AUTHORIZATION],
            "Bearer tok"
        );
        assert_eq!(request.headers()[reqwest::header::COOKIE], "sid=1");
    }

    #[test]
    fn omits_expired_session_credentials() {
        let mut source = test_source(None, None);
        source.access_token = Some("tok".into());
        source.session_cookie = Some("sid=1".into());
        source.session_expires_at = Some("0".into());
        let request = build(&source, "https://example.com/chapter");
        assert!(!request
            .headers()
            .contains_key(reqwest::header::AUTHORIZATION));
        assert!(!request.headers().contains_key(reqwest::header::COOKIE));
    }

    #[tokio::test]
    async fn identifies_cloudflare_by_response_headers() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            stream
                .write_all(
                    b"HTTP/1.1 403 Forbidden\r\nServer: cloudflare\r\nContent-Length: 0\r\n\r\n",
                )
                .unwrap();
        });
        let response = reqwest::Client::new()
            .get(format!("http://{address}/"))
            .send()
            .await
            .unwrap();
        let reason = response_error(response, "test").await;
        server.join().unwrap();
        assert!(reason.contains("Cloudflare challenge"));
        assert!(!AppError::Network(reason).requires_authentication());
    }
}
