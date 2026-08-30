use super::SourceRepository;
use crate::{
    error::AppError,
    source::{BookSource, CatalogRule, InfoRule, SearchRule},
};

#[derive(Clone)]
pub struct SqliteSourceRepository {
    pool: sqlx::SqlitePool,
}
impl SqliteSourceRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, source: &BookSource) -> Result<i64, AppError> {
        let result = sqlx::query("INSERT INTO book_sources (name, base_url, search_url, search_item_selector, title_selector, author_selector, cover_selector, url_selector, enabled, info_title_selector, info_author_selector, info_intro_selector, catalog_item_selector, catalog_title_selector, catalog_url_selector, content_selector, header, login_url, login_method, login_body, token_path, sign_script, proxy_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(name) DO UPDATE SET base_url=excluded.base_url, search_url=excluded.search_url, search_item_selector=excluded.search_item_selector, title_selector=excluded.title_selector, author_selector=excluded.author_selector, cover_selector=excluded.cover_selector, url_selector=excluded.url_selector, enabled=excluded.enabled, info_title_selector=excluded.info_title_selector, info_author_selector=excluded.info_author_selector, info_intro_selector=excluded.info_intro_selector, catalog_item_selector=excluded.catalog_item_selector, catalog_title_selector=excluded.catalog_title_selector, catalog_url_selector=excluded.catalog_url_selector, content_selector=excluded.content_selector, header=excluded.header, login_url=excluded.login_url, login_method=excluded.login_method, login_body=excluded.login_body, token_path=excluded.token_path, sign_script=excluded.sign_script, proxy_url=excluded.proxy_url, updated_at=CURRENT_TIMESTAMP")
            .bind(&source.name).bind(&source.base_url).bind(&source.search_url)
            .bind(&source.search_rule.item).bind(&source.search_rule.title).bind(&source.search_rule.author).bind(&source.search_rule.cover).bind(&source.search_rule.url)
            .bind(source.enabled as i64).bind(&source.info_rule.title).bind(&source.info_rule.author).bind(&source.info_rule.intro)
            .bind(&source.catalog_rule.item).bind(&source.catalog_rule.title).bind(&source.catalog_rule.url).bind(&source.content_selector)
            .bind(&source.header).bind(&source.login_url).bind(&source.login_method).bind(&source.login_body).bind(&source.token_path).bind(&source.sign_script).bind(&source.proxy_url)
            .execute(&self.pool).await.map_err(|error| AppError::Database(error.to_string()))?;
        let _ = result;
        sqlx::query_scalar("SELECT id FROM book_sources WHERE name = ?")
            .bind(&source.name)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
    }
}

#[derive(sqlx::FromRow)]
struct SourceRow {
    id: i64,
    name: String,
    base_url: String,
    search_url: String,
    search_item_selector: String,
    title_selector: String,
    author_selector: Option<String>,
    cover_selector: Option<String>,
    url_selector: String,
    info_title_selector: Option<String>,
    info_author_selector: Option<String>,
    info_intro_selector: Option<String>,
    catalog_item_selector: Option<String>,
    catalog_title_selector: Option<String>,
    catalog_url_selector: Option<String>,
    content_selector: Option<String>,
    enabled: i64,
    header: Option<String>,
    login_url: Option<String>,
    login_method: String,
    login_body: Option<String>,
    token_path: Option<String>,
    access_token: Option<String>,
    session_cookie: Option<String>,
    session_expires_at: Option<String>,
    sign_script: Option<String>,
    proxy_url: Option<String>,
}

fn map_source(row: SourceRow) -> BookSource {
    BookSource {
        id: row.id,
        name: row.name,
        base_url: row.base_url,
        search_url: row.search_url,
        search_rule: SearchRule {
            item: row.search_item_selector,
            title: row.title_selector,
            author: row.author_selector,
            cover: row.cover_selector,
            url: row.url_selector,
        },
        info_rule: InfoRule {
            title: row.info_title_selector,
            author: row.info_author_selector,
            intro: row.info_intro_selector,
        },
        catalog_rule: CatalogRule {
            item: row.catalog_item_selector.unwrap_or_else(|| "a".into()),
            title: row.catalog_title_selector.unwrap_or_else(|| "a".into()),
            url: row
                .catalog_url_selector
                .unwrap_or_else(|| "a::attr(href)".into()),
        },
        content_selector: row.content_selector.unwrap_or_else(|| "body".into()),
        header: row.header,
        login_url: row.login_url,
        login_method: row.login_method,
        login_body: row.login_body,
        token_path: row.token_path,
        access_token: row.access_token,
        session_cookie: row.session_cookie,
        session_expires_at: row.session_expires_at,
        sign_script: row.sign_script,
        proxy_url: row.proxy_url,
        enabled: row.enabled != 0,
    }
}

const SOURCE_SELECT: &str = "SELECT id, name, base_url, search_url, search_item_selector, title_selector, author_selector, cover_selector, url_selector, info_title_selector, info_author_selector, info_intro_selector, catalog_item_selector, catalog_title_selector, catalog_url_selector, content_selector, enabled, header, login_url, login_method, login_body, token_path, access_token, session_cookie, session_expires_at, sign_script, proxy_url FROM book_sources";

impl SourceRepository for SqliteSourceRepository {
    async fn list(&self) -> Result<Vec<BookSource>, AppError> {
        sqlx::query_as::<_, SourceRow>(&format!("{SOURCE_SELECT} ORDER BY name"))
            .fetch_all(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
            .map(|rows| rows.into_iter().map(map_source).collect())
    }
    async fn get(&self, id: i64) -> Result<Option<BookSource>, AppError> {
        sqlx::query_as::<_, SourceRow>(&format!("{SOURCE_SELECT} WHERE id = ?"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
            .map(|row| row.map(map_source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn upsert_round_trips_proxy_url() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let repository = SqliteSourceRepository::new(pool);
        let source = BookSource {
            id: 0,
            name: "proxy-test".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com/?q={{key}}".into(),
            search_rule: SearchRule { item: ".book".into(), title: ".title".into(), author: None, cover: None, url: "a".into() },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule { item: "a".into(), title: "a".into(), url: "a::attr(href)".into() },
            content_selector: "body".into(),
            header: None,
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: None,
            proxy_url: Some("socks5://127.0.0.1:1080".into()),
            enabled: true,
        };
        let id = repository.upsert(&source).await.unwrap();
        assert_eq!(repository.get(id).await.unwrap().unwrap().proxy_url.as_deref(), Some("socks5://127.0.0.1:1080"));
    }
}
