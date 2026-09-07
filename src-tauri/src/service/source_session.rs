use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::client::build_source_client_with_cookie_jar,
    repository::source::SqliteSourceRepository,
    source_engine::url::{self, FetchedBytes, RequestSpec},
};
use reqwest::cookie::CookieStore;
use std::{collections::BTreeMap, sync::Arc};

/// One source-scoped HTTP session whose response cookies survive app restarts.
pub struct SourceSession {
    source: tokio::sync::RwLock<BookSource>,
    sources: SqliteSourceRepository,
    client: reqwest::Client,
    jar: Arc<reqwest::cookie::Jar>,
}

impl SourceSession {
    pub fn new(
        source: BookSource,
        sources: SqliteSourceRepository,
        timeout_secs: u64,
        global_proxy: Option<&str>,
    ) -> Result<Self, AppError> {
        let (client, jar) =
            build_source_client_with_cookie_jar(&source, timeout_secs, global_proxy)?;
        if source.enabled_cookie_jar && !source.session_expired() {
            seed_cookie_jar(&jar, &source.base_url, source.session_cookie.as_deref());
        }
        Ok(Self {
            source: tokio::sync::RwLock::new(source),
            sources,
            client,
            jar,
        })
    }

    pub async fn prepare(&self, spec: &RequestSpec) -> Result<reqwest::Request, AppError> {
        let source = self.source.read().await;
        url::prepare(&self.client, &source, spec)
    }

    pub async fn send(&self, spec: &RequestSpec) -> Result<reqwest::Response, AppError> {
        let source = self.source.read().await.clone();
        if source.enabled_cookie_jar && !source.session_expired() {
            seed_cookie_jar(
                &self.jar,
                spec.url.as_str(),
                source.session_cookie.as_deref(),
            );
        }
        let response = url::send(&self.client, &source, spec).await?;
        self.persist_cookies(&source, spec, &response).await?;
        Ok(response)
    }

    pub async fn fetch_bytes(&self, spec: &RequestSpec) -> Result<FetchedBytes, AppError> {
        let source = self.source.read().await.clone();
        if spec.url.scheme() == "data" {
            return url::fetch_bytes(&self.client, &source, spec).await;
        }
        let response = self.send(spec).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                crate::infrastructure::http::request::response_error(response, &source.name).await,
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

    async fn persist_cookies(
        &self,
        source: &BookSource,
        spec: &RequestSpec,
        response: &reqwest::Response,
    ) -> Result<(), AppError> {
        if !source.enabled_cookie_jar {
            return Ok(());
        }
        let mut cookies = cookie_map(source.session_cookie.as_deref());
        for scope in [&source.base_url, spec.url.as_str(), response.url().as_str()] {
            if let Ok(url) = reqwest::Url::parse(scope) {
                if let Some(value) = self
                    .jar
                    .cookies(&url)
                    .and_then(|value| value.to_str().ok().map(str::to_owned))
                {
                    cookies.extend(cookie_map(Some(&value)));
                }
            }
        }
        apply_set_cookie_headers(&mut cookies, response.headers());
        let value = cookie_header(&cookies);
        if value == normalized_cookie_header(source.session_cookie.as_deref()) {
            return Ok(());
        }
        self.sources
            .update_cookie(source.id, value.as_deref())
            .await?;
        let mut current = self.source.write().await;
        current.session_cookie = value;
        current.session_expires_at = None;
        Ok(())
    }
}

fn seed_cookie_jar(jar: &reqwest::cookie::Jar, scope: &str, header: Option<&str>) {
    let (Ok(url), Some(header)) = (reqwest::Url::parse(scope), header) else {
        return;
    };
    for (name, value) in cookie_map(Some(header)) {
        jar.add_cookie_str(&format!("{name}={value}"), &url);
    }
}

fn apply_set_cookie_headers(
    cookies: &mut BTreeMap<String, String>,
    headers: &reqwest::header::HeaderMap,
) {
    for value in headers.get_all(reqwest::header::SET_COOKIE) {
        let Ok(value) = value.to_str() else { continue };
        let Some(pair) = value.split(';').next() else {
            continue;
        };
        let Some((name, cookie_value)) = pair.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        let remove = cookie_value.trim().is_empty()
            || value.split(';').any(|part| {
                part.trim()
                    .strip_prefix("Max-Age=")
                    .or_else(|| part.trim().strip_prefix("max-age="))
                    .is_some_and(|age| age.trim().parse::<i64>().is_ok_and(|age| age <= 0))
            });
        if remove {
            cookies.remove(name);
        } else {
            cookies.insert(name.to_owned(), cookie_value.trim().to_owned());
        }
    }
}

fn cookie_map(header: Option<&str>) -> BTreeMap<String, String> {
    header
        .into_iter()
        .flat_map(|value| value.split(';'))
        .filter_map(|part| {
            let (name, value) = part.trim().split_once('=')?;
            (!name.is_empty()).then(|| (name.to_owned(), value.to_owned()))
        })
        .collect()
}

fn cookie_header(cookies: &BTreeMap<String, String>) -> Option<String> {
    (!cookies.is_empty()).then(|| {
        cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn normalized_cookie_header(header: Option<&str>) -> Option<String> {
    cookie_header(&cookie_map(header))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::source::{CatalogRule, InfoRule, RawSourceRules, SearchRule};
    use crate::repository::SourceRepository;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    #[test]
    fn response_cookies_replace_and_remove_existing_values() {
        let mut cookies = cookie_map(Some("old=1; remove=1"));
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::SET_COOKIE,
            "old=2; Path=/".parse().unwrap(),
        );
        headers.append(
            reqwest::header::SET_COOKIE,
            "remove=; Max-Age=0; Path=/".parse().unwrap(),
        );
        apply_set_cookie_headers(&mut cookies, &headers);
        assert_eq!(cookie_header(&cookies).as_deref(), Some("old=2"));
    }

    #[tokio::test]
    async fn ordinary_response_cookie_is_persisted() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request).unwrap();
            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\nSet-Cookie: sid=fresh; Path=/; HttpOnly\r\nContent-Length: 2\r\n\r\nok",
                )
                .unwrap();
        });
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let sources = SqliteSourceRepository::new(pool);
        let source = BookSource {
            id: 0,
            name: "cookie".into(),
            base_url: format!("http://{address}/"),
            search_url: format!("http://{address}/search"),
            explore_url: None,
            book_url_pattern: None,
            enabled_cookie_jar: true,
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
            session_cookie: Some("old=1".into()),
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            source_group: None,
            custom_order: 0,
            weight: 0,
            enabled_explore: true,
            respond_time: None,
            last_update_time: None,
            raw_rules: RawSourceRules::default(),
        };
        let id = sources.upsert(&source).await.unwrap();
        sources
            .update_session(id, None, Some("old=1"), None)
            .await
            .unwrap();
        let source = sources.get(id).await.unwrap().unwrap();
        let session = SourceSession::new(source.clone(), sources.clone(), 5, None).unwrap();
        let spec =
            crate::source_engine::url::build(&source, &source.search_url, None, "test").unwrap();
        let response = session.send(&spec).await.unwrap();
        assert_eq!(response.text().await.unwrap(), "ok");
        server.join().unwrap();
        let saved = sources.get(id).await.unwrap().unwrap();
        assert_eq!(saved.session_cookie.as_deref(), Some("old=1; sid=fresh"));
        assert!(saved.session_expires_at.is_none());
    }
}
