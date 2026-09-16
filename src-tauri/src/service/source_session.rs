mod cookies;
use cookies::{
    apply_set_cookie_headers, cookie_header, cookie_map, normalized_cookie_header, seed_cookie_jar,
};

use crate::{
    domain::source::{BookInfo, BookSource},
    error::AppError,
    infrastructure::http::{client::build_source_client_with_cookie_jar, request::response_error},
    repository::source::SqliteSourceRepository,
    source_engine::{
        pipeline::{parse_book_info, parse_catalog_page},
        url::{self, FetchedBytes, RequestSpec},
    },
};
use reqwest::cookie::CookieStore;
use std::{collections::HashSet, sync::Arc};

/// Upper bound on the 目录 pages one book is allowed to span; the same ceiling
/// `ReaderService::refresh_catalog` uses so a next-page loop cannot run away.
const MAX_CATALOG_PAGES: usize = 50;

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

    /// One 详情 page parsed through this source's `ruleBookInfo`.
    ///
    /// Read-only: the caller decides whether the result is worth persisting, so
    /// 换源 can inspect a candidate source without touching the shelf.
    pub async fn fetch_book_info(&self, book_url: &str) -> Result<BookInfo, AppError> {
        let source = self.source.read().await.clone();
        let request = url::build(&source, book_url, None, "详情 URL")?;
        let response = self.send(&request).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let html = url::decode_text(response, &request, &source).await?;
        parse_book_info(&source, &html)
    }

    /// The whole 目录 for `book_url`, following `ruleToc.nextTocUrl` until the
    /// source stops asking for another page (or [`MAX_CATALOG_PAGES`] is hit).
    /// Returns `(title, url)` pairs in reading order and never touches the
    /// database — the same list is both a 换源 preview and the committed catalog.
    pub async fn fetch_catalog(&self, book_url: &str) -> Result<Vec<(String, String)>, AppError> {
        let source = self.source.read().await.clone();
        let mut current_rule = book_url.to_owned();
        let mut current_base = source.base_url.clone();
        let mut visited = HashSet::new();
        let mut catalog = Vec::new();
        for _ in 0..MAX_CATALOG_PAGES {
            let request =
                url::build_with_base(&source, &current_base, &current_rule, None, "目录 URL")?;
            let request_key = format!("{} {} {:?}", request.method, request.url, request.body);
            if !visited.insert(request_key) {
                break;
            }
            let response = self.send(&request).await?;
            if !response.status().is_success() {
                return Err(AppError::Network(
                    response_error(response, &source.name).await,
                ));
            }
            let html = url::decode_text(response, &request, &source).await?;
            let (page, next) = parse_catalog_page(&source, &html)?;
            catalog.extend(page);
            let Some(next) = next else {
                break;
            };
            current_base = request.url.to_string();
            current_rule = next;
        }
        Ok(catalog)
    }

    /// Best-effort cover download: a source that will not serve its own cover
    /// must not fail the operation that asked for it.
    pub async fn fetch_cover(&self, cover_url: &str) -> Option<FetchedBytes> {
        let source = self.source.read().await.clone();
        let request = url::build(&source, cover_url, None, "封面 URL").ok()?;
        self.fetch_bytes(&request).await.ok()
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
