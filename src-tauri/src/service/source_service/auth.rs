//! Protocol login and session-state commands for book sources.

use crate::{
    error::AppError,
    infrastructure::http::{
        client::build_source_client_with_cookie_jar, url::resolve_url,
    },
    service::source_service::{
        cookies::{cookie_max_age, merge_cookies, session_expiry},
        SourceLoginInput, SourceLoginResult, SourceService, SourceSessionStatus,
    },
};
use reqwest::cookie::CookieStore;

impl SourceService {
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
        .or_else(|| current.as_f64().map(|v| v.to_string()))
        .or_else(|| current.as_bool().map(|v| v.to_string()))
}

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::SourceService;
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
}
