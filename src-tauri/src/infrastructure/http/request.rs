use crate::{error::AppError, source::BookSource};
use sha2::{Digest, Sha256};

pub fn source_request(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
) -> Result<reqwest::RequestBuilder, AppError> {
    let referer = reqwest::Url::parse(url).ok().map(|parsed| {
        let mut origin = parsed;
        origin.set_path("/");
        origin.set_query(None);
        origin.set_fragment(None);
        origin.to_string()
    });
    let mut request = client.get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9,en;q=0.8")
        .header(reqwest::header::CACHE_CONTROL, "no-cache");
    if let Some(referer) = referer {
        request = request.header(reqwest::header::REFERER, referer);
    }
    if let Some(token) = source.access_token.as_deref().filter(|v| !v.is_empty()) {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    if let Some(cookie) = source.session_cookie.as_deref().filter(|v| !v.is_empty()) {
        request = request.header(reqwest::header::COOKIE, cookie);
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
                    .ok_or_else(|| AppError::InvalidArgument(format!("非法请求头: {}", name.trim())))?;
            }
        }
    }
    if let Some(script) = source.sign_script.as_deref() {
        if let Some(signature) = evaluate_sign_script(script, url) {
            request = request.header("X-Signature", signature);
        }
    }
    Ok(request)
}

fn evaluate_sign_script(script: &str, url: &str) -> Option<String> {
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

pub async fn send_source_request(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
) -> Result<reqwest::Response, AppError> {
    let mut last_error = String::new();
    for attempt in 0..3 {
        let started = std::time::Instant::now();
        tracing::debug!(target: "network", url = %url, attempt, "sending source request");
        let request = source_request(client, url, source)
            .and_then(|builder| builder.build().map_err(AppError::network))
            .map_err(|e| AppError::Network(format!("请求构造失败，请检查认证 Header: {e}")))?;
        match client.execute(request).await {
            Ok(response) => {
                tracing::debug!(target: "network", url = %url, status = response.status().as_u16(), elapsed_ms = started.elapsed().as_millis() as u64, "source response received");
                return Ok(response);
            }
            Err(error) => {
                tracing::warn!(target: "network", url = %url, attempt, elapsed_ms = started.elapsed().as_millis() as u64, error = %error, "source request failed");
                last_error = if error.is_timeout() {
                    "连接超时，请检查网络或代理".into()
                } else if error.is_connect() {
                    format!("无法连接目标站点（{}），请检查 DNS、防火墙或代理", error)
                } else if error.is_builder() {
                    format!("请求配置无效: {error}")
                } else {
                    error.to_string()
                };
                if attempt < 2 {
                    tokio::time::sleep(std::time::Duration::from_millis(250 * (attempt + 1))).await;
                }
            }
        }
    }
    Err(AppError::Network(last_error))
}
