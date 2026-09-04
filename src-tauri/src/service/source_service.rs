//! Book-source CRUD, legado import and authentication.
//!
//! Search and source probing live in [`crate::service::search_service`].

use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::{
        client::build_source_client_with_cookie_jar, request::user_agent, url::resolve_url,
    },
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::{
        compat::{raw_unsupported_source_names, source_has_unsupported_rules},
        import::parse_sources_json,
    },
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use reqwest::cookie::CookieStore;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct SourceService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl SourceService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }
    pub async fn list(&self) -> Result<Vec<BookSource>, AppError> {
        self.sources.list().await
    }
    pub async fn get(&self, id: i64) -> Result<BookSource, AppError> {
        self.sources
            .get(id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))
    }
    pub async fn upsert(&self, source: &BookSource) -> Result<i64, AppError> {
        self.sources.upsert(source).await
    }

    pub async fn import_json(&self, input: &str) -> Result<SourceImportReport, AppError> {
        let sources = parse_sources_json(input)?;
        let raw_partial = raw_unsupported_source_names(input);
        let mut report = SourceImportReport::default();
        for source in sources {
            let partial =
                raw_partial.contains(&source.name) || source_has_unsupported_rules(&source);
            match self.upsert(&BookSource::from_import(&source)).await {
                Ok(_) => {
                    report.imported += 1;
                    if partial {
                        report.partial.push(source.name);
                    }
                }
                Err(error) => report.failed.push(format!("{}: {error}", source.name)),
            }
        }
        tracing::info!(target: "source", imported = report.imported, failed = report.failed.len(), partial = report.partial.len(), "legado source import finished");
        Ok(report)
    }

    pub async fn import_url(&self, raw_url: &str) -> Result<SourceImportReport, AppError> {
        let url = reqwest::Url::parse(raw_url.trim())
            .map_err(|_| AppError::InvalidArgument("书源 URL 无效".into()))?;
        // Deliberately plain: no cookie jar, and reqwest's default redirect
        // policy. Routing this through `build_shared_client` would change both.
        let client = reqwest::Client::builder()
            .user_agent(user_agent())
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(AppError::network)?;
        let response = client.get(url).send().await.map_err(AppError::network)?;
        if !response.status().is_success() {
            return Err(format!("书源 URL 返回 HTTP {}", response.status()).into());
        }
        self.import_json(&response.text().await.map_err(AppError::network)?)
            .await
    }

    pub async fn login(&self, input: SourceLoginInput) -> Result<SourceLoginResult, AppError> {
        let source = self.get(input.source_id).await?;
        let login_url = source.login_url.as_deref().ok_or("该书源未配置登录 URL")?;
        let login_url = login_url
            .replace("{{username}}", &input.username)
            .replace("{{password}}", &input.password);
        let url = resolve_url(&source.base_url, &login_url, "登录 URL")?;
        let (client, cookie_jar) = build_source_client_with_cookie_jar(
            &source,
            20,
            self.settings.proxy_url().await?.as_deref(),
        )?;
        if let Some(cookie) = source.session_cookie.as_deref() {
            cookie_jar.add_cookie_str(cookie, &url);
        }
        let body = source
            .login_body
            .as_deref()
            .unwrap_or("{\"username\":\"{{username}}\",\"password\":\"{{password}}\"}")
            .replace("{{username}}", &input.username)
            .replace("{{password}}", &input.password);
        let mut request = match source.login_method.as_str() {
            "GET" => client.get(url.clone()),
            "PUT" => client.put(url.clone()),
            _ => client.post(url.clone()),
        };
        request = apply_headers(request, source.header.as_deref());
        request = if body.trim_start().starts_with('{') {
            request.header(reqwest::header::CONTENT_TYPE, "application/json")
        } else {
            request.header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
        }
        .body(body);
        let response = request.send().await.map_err(AppError::network)?;
        if !response.status().is_success() {
            return Err(format!("登录返回 HTTP {}", response.status()).into());
        }
        let response_url = response.url().clone();
        let mut cookie_expiry = None;
        let cookies = response
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .inspect(|value| {
                if let Some(max_age) = cookie_max_age(value) {
                    cookie_expiry = Some(now_epoch().saturating_add(max_age));
                }
            })
            .filter_map(|v| v.split(';').next())
            .collect::<Vec<_>>()
            .join("; ");
        let jar_cookies = [&response_url, &url]
            .into_iter()
            .filter_map(|scope| cookie_jar.cookies(scope))
            .filter_map(|value| value.to_str().ok().map(str::to_owned))
            .collect::<Vec<_>>()
            .join("; ");
        let cookies = merge_cookies(
            source.session_cookie.as_deref(),
            Some(&format!("{cookies}; {jar_cookies}")),
        );
        let response_text = response.text().await.map_err(AppError::network)?;
        let token = source.token_path.as_deref().and_then(|path| {
            serde_json::from_str::<serde_json::Value>(&response_text)
                .ok()
                .and_then(|value| json_path(&value, path))
        });
        let session_expires_at = session_expiry(response_text.as_str(), token.as_deref())
            .or_else(|| cookie_expiry.map(|value| value.to_string()));
        self.sources
            .update_session(
                input.source_id,
                token.as_deref(),
                cookies.as_deref(),
                session_expires_at.as_deref(),
            )
            .await?;
        Ok(SourceLoginResult {
            source_id: input.source_id,
            authenticated: token.is_some() || cookies.is_some(),
            has_token: token.is_some(),
            has_cookie: cookies.is_some(),
            session_expires_at,
        })
    }

    pub async fn session_status(&self, source_id: i64) -> Result<SourceSessionStatus, AppError> {
        let source = self.get(source_id).await?;
        Ok(SourceSessionStatus {
            source_id,
            state: source.session_state().to_owned(),
            has_token: source.access_token.is_some(),
            has_cookie: source.session_cookie.is_some(),
            expires_at: source.session_expires_at,
        })
    }

    /// Refreshes a protocol session using the source's configured login flow.
    /// Credentials are intentionally supplied per call and are never stored.
    pub async fn refresh_session(
        &self,
        input: SourceLoginInput,
    ) -> Result<SourceLoginResult, AppError> {
        self.login(input).await
    }

    pub async fn clear_session(&self, source_id: i64) -> Result<(), AppError> {
        self.sources.clear_session(source_id).await
    }

    /// Persists cookies collected from the source's embedded browser window.
    /// Existing token credentials are retained because browser challenges often
    /// supplement, rather than replace, an API token.
    pub async fn save_browser_cookies(
        &self,
        source_id: i64,
        cookies: &str,
    ) -> Result<SourceLoginResult, AppError> {
        let source = self.get(source_id).await?;
        let cookies = cookies.trim();
        if cookies.is_empty() {
            return Err(AppError::InvalidArgument(
                "浏览器中没有可保存的 Cookie".into(),
            ));
        }
        if cloudflare_challenge_unsolved(cookies) {
            return Err(AppError::InvalidArgument(
                "Cloudflare 验证尚未完成：浏览器只返回了 __cf_bm，没有 cf_clearance。请在认证窗口里等待验证通过（页面显示出书源内容）后再读取会话".into(),
            ));
        }
        let merged_cookies = merge_cookies(source.session_cookie.as_deref(), Some(cookies))
            .ok_or_else(|| AppError::InvalidArgument("浏览器中没有可保存的有效 Cookie".into()))?;
        let session_expires_at = (!source.session_expired())
            .then(|| source.session_expires_at.clone())
            .flatten();
        self.sources
            .update_session(
                source_id,
                source.access_token.as_deref(),
                Some(merged_cookies.as_str()),
                // A 401/403 marks the previous protocol session expired. A
                // successful browser challenge supersedes that marker; keep
                // it would make request builders silently omit these cookies.
                session_expires_at.as_deref(),
            )
            .await?;
        Ok(SourceLoginResult {
            source_id,
            authenticated: true,
            has_token: source.access_token.is_some(),
            has_cookie: true,
            session_expires_at,
        })
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SourceImportReport {
    pub imported: usize,
    pub failed: Vec<String>,
    pub partial: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct SourceLoginInput {
    pub source_id: i64,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SourceLoginResult {
    pub source_id: i64,
    pub authenticated: bool,
    pub has_token: bool,
    pub has_cookie: bool,
    pub session_expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SourceSessionStatus {
    pub source_id: i64,
    pub state: String,
    pub has_token: bool,
    pub has_cookie: bool,
    pub expires_at: Option<String>,
}

fn session_expiry(response: &str, token: Option<&str>) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(response) {
        if let Some(expiry) = value.get("expires_at").and_then(expiry_value) {
            return Some(expiry);
        }
        if let Some(seconds) = value.get("expires_in").and_then(|v| v.as_u64()) {
            return Some((now_epoch().saturating_add(seconds)).to_string());
        }
    }
    let token = token?;
    let payload = token.split('.').nth(1)?;
    let bytes = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    value.get("exp").and_then(expiry_value)
}

fn expiry_value(value: &serde_json::Value) -> Option<String> {
    value
        .as_u64()
        .map(|value| value.to_string())
        .or_else(|| value.as_str().map(str::to_owned))
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default()
}

fn cookie_max_age(cookie: &str) -> Option<u64> {
    cookie.split(';').find_map(|attribute| {
        let (name, value) = attribute.trim().split_once('=')?;
        name.eq_ignore_ascii_case("max-age")
            .then(|| value.trim().parse::<i64>().ok())
            .flatten()
            .map(|seconds| seconds.max(0) as u64)
    })
}

/// Merge Cookie header fragments using the same name-wins semantics as
/// legado's CookieManager. Empty/malformed fragments are ignored, and the
/// newest value wins when a browser challenge refreshes an existing cookie.
/// Whether a cookie set proves the user is still stuck on the challenge page.
///
/// Cloudflare hands `__cf_bm` (and the `cf_chl_*` scratch cookies) to every
/// visitor, including one who has only just loaded "Just a moment…". Solving
/// the challenge is what mints `cf_clearance`. Accepting the former as proof
/// is what made the source report "已认证" while every later request still got
/// bounced — the failure this whole flow exists to fix, reported as a success.
///
/// Sources with no Cloudflare marker at all are ordinary logins and pass
/// through untouched.
fn cloudflare_challenge_unsolved(cookies: &str) -> bool {
    let mut saw_marker = false;
    for name in cookies
        .split(';')
        .filter_map(|pair| pair.trim().split_once('='))
        .map(|(name, _)| name.trim())
    {
        if name.eq_ignore_ascii_case("cf_clearance") {
            return false;
        }
        saw_marker |= name.eq_ignore_ascii_case("__cf_bm")
            || name.to_ascii_lowercase().starts_with("cf_chl")
            || name.to_ascii_lowercase().starts_with("_cf_chl");
    }
    saw_marker
}

fn merge_cookies(values: Option<&str>, extra: Option<&str>) -> Option<String> {
    let mut merged = std::collections::BTreeMap::new();
    for raw in values.into_iter().chain(extra) {
        for pair in raw.split(';') {
            let Some((name, value)) = pair.trim().split_once('=') else {
                continue;
            };
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            merged.insert(name.to_owned(), value.trim().to_owned());
        }
    }
    (!merged.is_empty()).then(|| {
        merged
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
    })
}
fn apply_headers(
    mut request: reqwest::RequestBuilder,
    raw: Option<&str>,
) -> reqwest::RequestBuilder {
    if let Some(raw) = raw {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(raw)
        {
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
    request
}
fn json_path(value: &serde_json::Value, path: &str) -> Option<String> {
    let mut current = value;
    for segment in path
        .trim_matches('/')
        .split(&['.', '/'][..])
        .filter(|s| !s.is_empty())
    {
        current = current.get(segment)?;
    }
    current
        .as_str()
        .map(str::to_owned)
        .or_else(|| current.as_i64().map(|v| v.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn pool() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn imports_partial_source_and_persists_it() {
        let service = SourceService::new(pool().await);
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article","name":".name","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;

        let report = service.import_json(input).await.unwrap();

        assert_eq!(report.imported, 1);
        assert_eq!(report.failed, Vec::<String>::new());
        assert_eq!(report.partial, vec!["XPath source"]);

        // The raw legado rules must survive the round trip through the
        // database, not just the write: the engine reads them back on search.
        let sources = service.list().await.unwrap();
        assert_eq!(sources.len(), 1);
        let raw = &sources[0].raw_rules;
        let search: serde_json::Value = serde_json::from_str(raw.search.as_ref().unwrap()).unwrap();
        assert_eq!(search["bookList"], "@XPath://article");
        assert_eq!(search["bookUrl"], "a");
        assert_eq!(raw.content.as_deref(), Some("\".content\""));
        assert!(raw.book_info.is_none());
    }

    #[tokio::test]
    async fn browser_cookies_replace_expired_session_marker() {
        let service = SourceService::new(pool().await);
        service
            .import_json(
                r#"[{"bookSourceName":"Browser source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":".book","name":".name","bookUrl":"a"}}]"#,
            )
            .await
            .unwrap();
        let source = service.list().await.unwrap().remove(0);
        service
            .sources
            .mark_session_expired(source.id)
            .await
            .unwrap();
        let result = service
            .save_browser_cookies(source.id, "cf_clearance=ok")
            .await
            .unwrap();
        assert!(result.authenticated);
        assert_eq!(result.session_expires_at, None);
        let saved = service.get(source.id).await.unwrap();
        assert_eq!(saved.session_state(), "authenticated");
        assert_eq!(saved.session_cookie.as_deref(), Some("cf_clearance=ok"));
    }

    /// The window can be read while "Just a moment…" is still on screen. Back
    /// then that stored `__cf_bm` alone and reported 已认证, so the source
    /// looked fixed and every later request still bounced.
    #[tokio::test]
    async fn browser_cookies_reject_an_unfinished_cloudflare_challenge() {
        let service = SourceService::new(pool().await);
        service
            .import_json(
                r#"[{"bookSourceName":"CF source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":".book","name":".name","bookUrl":"a"}}]"#,
            )
            .await
            .unwrap();
        let source = service.list().await.unwrap().remove(0);
        let error = service
            .save_browser_cookies(source.id, "__cf_bm=abc; cf_chl_rc_i=1")
            .await
            .unwrap_err();
        assert!(format!("{error}").contains("cf_clearance"), "{error}");
        let saved = service.get(source.id).await.unwrap();
        assert_eq!(saved.session_state(), "anonymous");
    }

    #[test]
    fn clearance_beside_the_challenge_markers_is_accepted() {
        assert!(!cloudflare_challenge_unsolved(
            "__cf_bm=abc; cf_clearance=ok"
        ));
    }

    /// Sources behind an ordinary login have no Cloudflare cookies at all and
    /// must not be caught by the gate.
    #[test]
    fn non_cloudflare_cookies_pass_through() {
        assert!(!cloudflare_challenge_unsolved("PHPSESSID=abc; remember=1"));
    }

    #[tokio::test]
    async fn browser_cookies_keep_a_live_token_expiry() {
        let service = SourceService::new(pool().await);
        service
            .import_json(
                r#"[{"bookSourceName":"Browser token","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","token":"token","tokenExpire":"4102444800","ruleSearch":{"bookList":".book","name":".name","bookUrl":"a"}}]"#,
            )
            .await
            .unwrap();
        let source = service.list().await.unwrap().remove(0);
        service
            .sources
            .update_session(source.id, Some("token"), None, Some("4102444800"))
            .await
            .unwrap();
        service
            .save_browser_cookies(source.id, "cf_clearance=ok")
            .await
            .unwrap();
        let saved = service.get(source.id).await.unwrap();
        assert_eq!(saved.session_expires_at.as_deref(), Some("4102444800"));
    }

    #[test]
    fn extracts_expiry_from_jwt_payload() {
        let token = "eyJhbGciOiJub25lIn0.eyJleHAiOjQyMDB9.signature";
        assert_eq!(session_expiry("{}", Some(token)).as_deref(), Some("4200"));
    }

    #[test]
    fn extracts_cookie_max_age_for_session_expiry() {
        assert_eq!(cookie_max_age("sid=1; Max-Age=3600; HttpOnly"), Some(3600));
        assert_eq!(cookie_max_age("sid=1; max-age=0"), Some(0));
        assert_eq!(cookie_max_age("sid=1; Path=/"), None);
    }

    #[test]
    fn merges_browser_cookies_without_dropping_existing_login_state() {
        assert_eq!(
            merge_cookies(
                Some("sid=old; theme=dark"),
                Some("cf_clearance=ok; sid=new")
            )
            .as_deref(),
            Some("cf_clearance=ok; sid=new; theme=dark")
        );
    }

    #[test]
    fn ignores_malformed_cookie_fragments() {
        assert_eq!(
            merge_cookies(Some("invalid; sid=1"), Some(" ")).as_deref(),
            Some("sid=1")
        );
    }
}
