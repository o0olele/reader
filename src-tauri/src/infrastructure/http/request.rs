use crate::{domain::source::BookSource, error::AppError};
use sha2::{Digest, Sha256};
use std::sync::RwLock;

/// Keep the browser challenge window and every follow-up HTTP request on one
/// User-Agent. Cloudflare binds its clearance cookie to the UA, so any
/// mismatch between the embedded webview and reqwest invalidates the cookie
/// the user just spent a captcha earning.
///
/// Only a fallback for the first launch, before the frontend has reported what
/// the embedded webview actually is. Forcing a made-up UA onto the webview is
/// the wrong direction — the webview keeps sending its own real client hints
/// regardless, so a forced UA just recreates the mismatch one layer down.
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

static USER_AGENT_OVERRIDE: RwLock<Option<String>> = RwLock::new(None);
static DETECTED_USER_AGENT: RwLock<Option<String>> = RwLock::new(None);

fn read(slot: &'static RwLock<Option<String>>) -> Option<String> {
    slot.read().ok().and_then(|value| value.clone())
}

fn write(slot: &'static RwLock<Option<String>>, value: Option<&str>) {
    let value = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    if let Ok(mut slot) = slot.write() {
        *slot = value;
    }
}

/// The User-Agent shared by the auth webview, the rule engine's HTTP calls and
/// every service client. Process-global on purpose: these live in unrelated
/// modules and a per-caller value is exactly the drift this is here to prevent.
///
/// Precedence: explicit user setting, then whatever the embedded webview
/// reported about itself, then the compiled-in fallback.
pub fn user_agent() -> String {
    resolve_user_agent(read(&USER_AGENT_OVERRIDE), read(&DETECTED_USER_AGENT))
}

/// Split out from [`user_agent`] so the precedence can be asserted without
/// writing to process-global state that the other tests read concurrently.
fn resolve_user_agent(configured: Option<String>, detected: Option<String>) -> String {
    configured
        .or(detected)
        .unwrap_or_else(|| DEFAULT_USER_AGENT.to_owned())
}

/// The explicit user setting, if any. The auth window consults this to decide
/// whether to override the webview's own UA — when the user has *not* set one,
/// leaving the webview alone is what keeps its UA and its client hints
/// mutually consistent.
pub fn user_agent_override() -> Option<String> {
    read(&USER_AGENT_OVERRIDE)
}

/// Installs the user-configured User-Agent. `None` or blank restores the
/// detected/default value. Called at startup and whenever settings are saved.
pub fn set_user_agent(value: Option<&str>) {
    write(&USER_AGENT_OVERRIDE, value);
}

/// Installs the `navigator.userAgent` the main window reported. This is the
/// real WebView2/WebKitGTK identity that the auth window will present to
/// Cloudflare, so replaying it verbatim from reqwest is what makes the
/// clearance cookie survive the handoff.
pub fn set_detected_user_agent(value: Option<&str>) {
    write(&DETECTED_USER_AGENT, value);
}

/// Chrome sends the three low-entropy client hints on every secure request, so
/// claiming to be Chrome while sending none is itself an inconsistency. The
/// high-entropy hints (`-Arch`, `-Bitness`, `-Full-Version-List`, …) are only
/// sent *after* the server asks via `Accept-CH`; volunteering them unprompted
/// is a bot signal, which is why they are deliberately absent here.
///
/// Returns nothing for a non-Chrome UA — the hints must agree with whatever UA
/// is actually configured rather than with a hardcoded Chrome build.
fn client_hints(user_agent: &str) -> Option<[(&'static str, String); 3]> {
    let major = user_agent
        .split("Chrome/")
        .nth(1)?
        .split(['.', ' '])
        .next()
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))?;
    // Android must be tested before Linux: Android UAs contain both.
    let platform = if user_agent.contains("Android") {
        "Android"
    } else if user_agent.contains("Windows") {
        "Windows"
    } else if user_agent.contains("Macintosh") || user_agent.contains("Mac OS X") {
        "macOS"
    } else if user_agent.contains("Linux") || user_agent.contains("X11") {
        "Linux"
    } else {
        return None;
    };
    let mobile = if user_agent.contains("Mobile") {
        "?1"
    } else {
        "?0"
    };
    // WebView2 identifies as Edge, and Edge's hints say so. Claiming Chrome
    // next to an `Edg/` UA is the same class of contradiction this function
    // exists to remove.
    let brand = if user_agent.contains("Edg/") {
        "Microsoft Edge"
    } else {
        "Google Chrome"
    };
    Some([
        (
            "Sec-CH-UA",
            format!(r#""{brand}";v="{major}", "Chromium";v="{major}", "Not_A Brand";v="24""#),
        ),
        ("Sec-CH-UA-Mobile", mobile.to_owned()),
        ("Sec-CH-UA-Platform", format!("\"{platform}\"")),
    ])
}

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
    let user_agent = user_agent();
    let mut request = client
        .request(method, url)
        .header(reqwest::header::USER_AGENT, &user_agent)
        // Chrome's exact document Accept. A lookalike is worth no more than an
        // honest one, so match it verbatim rather than approximately.
        .header(
            reqwest::header::ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        )
        .header(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9,en;q=0.8")
        .header(reqwest::header::CACHE_CONTROL, "no-cache")
        // A browser-shaped navigation. Their absence next to a Chrome UA is as
        // much of a mismatch as a malformed UA would be.
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Sec-Fetch-User", "?1")
        .header(reqwest::header::UPGRADE_INSECURE_REQUESTS, "1");
    for (name, value) in client_hints(&user_agent).into_iter().flatten() {
        request = request.header(name, value);
    }
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

/// Whether a response is Cloudflare demanding a browser, as opposed to merely
/// having passed through Cloudflare.
///
/// The distinction matters because `Server: cloudflare` and `cf-ray` are on
/// **every** response Cloudflare proxies, successes included. Treating them as
/// challenge evidence turned every 403 from every Cloudflare-fronted site —
/// WAF blocks, rate limits, geo blocks, plain origin 403s — into "需要浏览器
/// 验证", which hides the real status text and sends debugging down a path
/// that cannot work. 69shuba, for instance, answers a 14-byte `page not found`
/// 403 that no browser challenge would fix.
///
/// Only two things are real evidence: Cloudflare's own `cf-mitigated:
/// challenge` header, and the interstitial's markup.
pub fn is_challenge_response<'a>(
    headers: impl IntoIterator<Item = (&'a str, &'a str)>,
    body: &str,
) -> bool {
    for (name, value) in headers {
        if name.eq_ignore_ascii_case("cf-mitigated")
            && value.to_ascii_lowercase().contains("challenge")
        {
            return true;
        }
    }
    let body = body.to_ascii_lowercase();
    // `_cf_chl_opt` is the variable the interstitial defines and the same thing
    // legado polls for in its verification webview.
    body.contains("_cf_chl_opt")
        || body.contains("challenge-platform")
        || body.contains("just a moment")
        || body.contains("enable javascript and cookies")
}

pub async fn response_error(response: reqwest::Response, source_name: &str) -> String {
    let status = response.status();
    let headers = response
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            Some((name.as_str().to_owned(), value.to_str().ok()?.to_owned()))
        })
        .collect::<Vec<_>>();
    let detail = response.text().await.unwrap_or_default();
    let detail = detail.split_whitespace().collect::<Vec<_>>().join(" ");
    if is_challenge_response(
        headers
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str())),
        &detail,
    ) {
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
    fn uses_the_browser_user_agent_for_authenticated_requests() {
        let request = build(&test_source(None, None), "https://example.com/chapter");
        assert_eq!(
            request.headers()[reqwest::header::USER_AGENT],
            user_agent().as_str()
        );
    }

    /// The default UA used to omit `(KHTML, like Gecko)` and carry a two-part
    /// Chrome version. Cloudflare binds `cf_clearance` to the UA, so a UA no
    /// real Chrome would send makes the cookie useless the moment reqwest
    /// replays it.
    #[test]
    fn the_default_user_agent_is_a_well_formed_chrome_ua() {
        assert!(DEFAULT_USER_AGENT.contains("(KHTML, like Gecko)"));
        let version = DEFAULT_USER_AGENT
            .split("Chrome/")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .expect("UA advertises a Chrome version");
        assert_eq!(version.split('.').count(), 4, "got {version}");
    }

    /// Regression: the hints were written as raw strings containing `\"`, so
    /// every value went out with literal backslashes — a malformed structured
    /// header, and a far louder bot signal than sending nothing at all.
    #[test]
    fn client_hints_are_valid_structured_headers() {
        let hints = client_hints(DEFAULT_USER_AGENT).expect("chrome UA yields hints");
        for (name, value) in &hints {
            assert!(
                !value.contains('\\'),
                "{name} still escapes quotes: {value}"
            );
        }
        assert_eq!(
            hints[0].1,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#
        );
        assert_eq!(hints[1].1, "?0");
        assert_eq!(hints[2].1, "\"Windows\"");
    }

    /// The hints have to track the configured UA. Emitting Chrome 131/Windows
    /// hints next to a Firefox or Android UA reintroduces the mismatch this
    /// whole path exists to avoid.
    #[test]
    fn client_hints_follow_the_configured_user_agent() {
        let android = "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36";
        let hints = client_hints(android).expect("chrome UA yields hints");
        assert!(hints[0].1.contains(r#"v="120""#), "{}", hints[0].1);
        assert_eq!(hints[1].1, "?1");
        assert_eq!(hints[2].1, "\"Android\"");

        let firefox =
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0";
        assert!(client_hints(firefox).is_none());
    }

    /// The detected webview UA must win over the compiled-in guess, and an
    /// explicit user setting over both. Getting this order wrong is what would
    /// silently keep sending the placeholder UA forever.
    #[test]
    fn the_detected_webview_agent_outranks_the_default_but_not_the_user() {
        let detected = "Mozilla/5.0 WebView2/1.0".to_owned();
        let configured = "Custom/1.0".to_owned();
        assert_eq!(resolve_user_agent(None, None), DEFAULT_USER_AGENT);
        assert_eq!(
            resolve_user_agent(None, Some(detected.clone())),
            detected.as_str()
        );
        assert_eq!(
            resolve_user_agent(Some(configured.clone()), Some(detected)),
            configured.as_str()
        );
    }

    /// WebView2 — the runtime the auth window actually runs on — reports as
    /// Edge. Its own hints say "Microsoft Edge", so reqwest replaying the same
    /// UA has to say the same thing or the pair contradicts itself.
    #[test]
    fn client_hints_name_edge_for_a_webview2_user_agent() {
        let webview2 = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.2903.86";
        let hints = client_hints(webview2).expect("chromium UA yields hints");
        assert_eq!(
            hints[0].1,
            r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#
        );
    }

    #[test]
    fn sends_browser_navigation_headers() {
        let request = build(&test_source(None, None), "https://example.com/chapter");
        assert_eq!(request.headers()["sec-fetch-mode"], "navigate");
        assert_eq!(request.headers()["sec-ch-ua-platform"], "\"Windows\"");
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

    async fn reason_for(raw_response: &'static [u8]) -> String {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            stream.write_all(raw_response).unwrap();
        });
        let response = reqwest::Client::new()
            .get(format!("http://{address}/"))
            .send()
            .await
            .unwrap();
        let reason = response_error(response, "test").await;
        server.join().unwrap();
        reason
    }

    #[tokio::test]
    async fn identifies_a_challenge_by_the_cf_mitigated_header() {
        let reason = reason_for(
            b"HTTP/1.1 403 Forbidden\r\nServer: cloudflare\r\ncf-mitigated: challenge\r\nContent-Length: 0\r\n\r\n",
        )
        .await;
        assert!(reason.contains("Cloudflare challenge"), "{reason}");
        assert!(!AppError::Network(reason).requires_authentication());
    }

    /// Measured against the real 69shuba: a 14-byte `page not found` 403 with
    /// `Server: cloudflare` and a `cf-ray`, and no challenge markup anywhere.
    /// This used to be reported as "需要浏览器执行 JavaScript 验证", which both
    /// hid the actual status text and pointed debugging at a captcha that does
    /// not exist. The body must survive into the message.
    #[tokio::test]
    async fn a_plain_403_from_behind_cloudflare_reports_its_real_body() {
        let reason = reason_for(
            b"HTTP/1.1 403 Forbidden\r\nServer: cloudflare\r\ncf-ray: a35b5e2c5a90033f-HKG\r\nContent-Length: 14\r\n\r\npage not found",
        )
        .await;
        assert!(!reason.contains("Cloudflare challenge"), "{reason}");
        assert!(reason.contains("403"), "{reason}");
        assert!(reason.contains("page not found"), "{reason}");
    }

    /// The interstitial is recognisable from its markup even when the headers
    /// are unremarkable.
    #[tokio::test]
    async fn identifies_a_challenge_by_its_interstitial_markup() {
        let reason = reason_for(
            b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 46\r\n\r\n<title>Just a moment...</title><div id=x></div>",
        )
        .await;
        assert!(reason.contains("Cloudflare challenge"), "{reason}");
    }
}
