use super::SourceRepository;
use crate::{
    domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule},
    error::AppError,
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
        sqlx::query(
            "INSERT INTO book_sources (name, base_url, search_url, explore_url, search_item_selector, title_selector, author_selector, cover_selector, url_selector, enabled, info_title_selector, info_author_selector, info_intro_selector, info_cover_selector, catalog_item_selector, catalog_title_selector, catalog_url_selector, content_selector, next_toc_url_selector, next_content_url_selector, header, login_url, login_method, login_body, token_path, sign_script, proxy_url, concurrent_rate, rule_search, rule_book_info, rule_toc, rule_content, rule_explore) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(name) DO UPDATE SET base_url=excluded.base_url, search_url=excluded.search_url, explore_url=excluded.explore_url, search_item_selector=excluded.search_item_selector, title_selector=excluded.title_selector, author_selector=excluded.author_selector, cover_selector=excluded.cover_selector, url_selector=excluded.url_selector, enabled=excluded.enabled, info_title_selector=excluded.info_title_selector, info_author_selector=excluded.info_author_selector, info_intro_selector=excluded.info_intro_selector, info_cover_selector=excluded.info_cover_selector, catalog_item_selector=excluded.catalog_item_selector, catalog_title_selector=excluded.catalog_title_selector, catalog_url_selector=excluded.catalog_url_selector, content_selector=excluded.content_selector, next_toc_url_selector=excluded.next_toc_url_selector, next_content_url_selector=excluded.next_content_url_selector, header=excluded.header, login_url=excluded.login_url, login_method=excluded.login_method, login_body=excluded.login_body, token_path=excluded.token_path, sign_script=excluded.sign_script, proxy_url=excluded.proxy_url, concurrent_rate=excluded.concurrent_rate, rule_search=excluded.rule_search, rule_book_info=excluded.rule_book_info, rule_toc=excluded.rule_toc, rule_content=excluded.rule_content, rule_explore=excluded.rule_explore, updated_at=CURRENT_TIMESTAMP",
        )
            .bind(&source.name).bind(&source.base_url).bind(&source.search_url).bind(&source.explore_url)
            .bind(&source.search_rule.item).bind(&source.search_rule.title).bind(&source.search_rule.author).bind(&source.search_rule.cover).bind(&source.search_rule.url)
            .bind(source.enabled as i64).bind(&source.info_rule.title).bind(&source.info_rule.author).bind(&source.info_rule.intro).bind(&source.info_rule.cover)
            .bind(&source.catalog_rule.item).bind(&source.catalog_rule.title).bind(&source.catalog_rule.url).bind(&source.content_selector)
            .bind(&source.next_toc_url_selector).bind(&source.next_content_url_selector)
            .bind(&source.header).bind(&source.login_url).bind(&source.login_method).bind(&source.login_body).bind(&source.token_path).bind(&source.sign_script).bind(&source.proxy_url).bind(&source.concurrent_rate)
            .bind(&source.raw_rules.search).bind(&source.raw_rules.book_info).bind(&source.raw_rules.toc).bind(&source.raw_rules.content).bind(&source.raw_rules.explore)
            .execute(&self.pool).await.map_err(|error| AppError::Database(error.to_string()))?;
        sqlx::query_scalar("SELECT id FROM book_sources WHERE name = ?")
            .bind(&source.name)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
    }

    pub async fn update_session(
        &self,
        source_id: i64,
        access_token: Option<&str>,
        session_cookie: Option<&str>,
        session_expires_at: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query("UPDATE book_sources SET access_token = ?, session_cookie = ?, session_expires_at = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(access_token)
            .bind(session_cookie)
            .bind(session_expires_at)
            .bind(source_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(AppError::database)
    }

    pub async fn clear_session(&self, source_id: i64) -> Result<(), AppError> {
        self.update_session(source_id, None, None, None).await
    }

    pub async fn update_raw_rules(
        &self,
        source_id: i64,
        rules: &RawSourceRules,
    ) -> Result<(), AppError> {
        sqlx::query("UPDATE book_sources SET rule_search = ?, rule_book_info = ?, rule_toc = ?, rule_content = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(&rules.search).bind(&rules.book_info).bind(&rules.toc).bind(&rules.content).bind(source_id)
            .execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    /// Keeps the credentials for diagnostics/re-authentication, but marks the
    /// session as unusable after a server rejects it.
    pub async fn mark_session_expired(&self, source_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE book_sources SET session_expires_at = '0', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(source_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(AppError::database)
    }
}

#[derive(sqlx::FromRow)]
struct SourceRow {
    id: i64,
    name: String,
    base_url: String,
    search_url: String,
    explore_url: Option<String>,
    search_item_selector: String,
    title_selector: String,
    author_selector: Option<String>,
    cover_selector: Option<String>,
    url_selector: String,
    info_title_selector: Option<String>,
    info_author_selector: Option<String>,
    info_intro_selector: Option<String>,
    info_cover_selector: Option<String>,
    catalog_item_selector: Option<String>,
    catalog_title_selector: Option<String>,
    catalog_url_selector: Option<String>,
    content_selector: Option<String>,
    next_toc_url_selector: Option<String>,
    next_content_url_selector: Option<String>,
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
    concurrent_rate: Option<String>,
    rule_search: Option<String>,
    rule_book_info: Option<String>,
    rule_toc: Option<String>,
    rule_content: Option<String>,
    rule_explore: Option<String>,
}

fn map_source(row: SourceRow) -> BookSource {
    BookSource {
        id: row.id,
        name: row.name,
        base_url: row.base_url,
        search_url: row.search_url,
        explore_url: row.explore_url,
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
            cover: row.info_cover_selector,
            kind: None,
            latest_chapter: None,
        },
        catalog_rule: CatalogRule {
            item: row.catalog_item_selector.unwrap_or_else(|| "a".into()),
            title: row.catalog_title_selector.unwrap_or_else(|| "a".into()),
            url: row
                .catalog_url_selector
                .unwrap_or_else(|| "a::attr(href)".into()),
            next_url: row.next_toc_url_selector.clone(),
        },
        content_selector: row.content_selector.unwrap_or_else(|| "body".into()),
        next_toc_url_selector: row.next_toc_url_selector,
        next_content_url_selector: row.next_content_url_selector,
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
        concurrent_rate: row.concurrent_rate,
        enabled: row.enabled != 0,
        raw_rules: RawSourceRules {
            search: row.rule_search,
            book_info: row.rule_book_info,
            toc: row.rule_toc,
            content: row.rule_content,
            explore: row.rule_explore,
        },
    }
}

const SOURCE_SELECT: &str = "SELECT id, name, base_url, search_url, explore_url, search_item_selector, title_selector, author_selector, cover_selector, url_selector, info_title_selector, info_author_selector, info_intro_selector, info_cover_selector, catalog_item_selector, catalog_title_selector, catalog_url_selector, content_selector, next_toc_url_selector, next_content_url_selector, enabled, header, login_url, login_method, login_body, token_path, access_token, session_cookie, session_expires_at, sign_script, proxy_url, concurrent_rate, rule_search, rule_book_info, rule_toc, rule_content, rule_explore FROM book_sources";

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
    async fn upsert_round_trips_request_settings() {
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
            explore_url: Some("热门::/hot".into()),
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a".into(),
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
            concurrent_rate: Some("5/1000".into()),
            enabled: true,
            raw_rules: RawSourceRules {
                explore: Some(r#"{"bookList":".book"}"#.into()),
                ..Default::default()
            },
        };
        let id = repository.upsert(&source).await.unwrap();
        assert_eq!(
            repository
                .get(id)
                .await
                .unwrap()
                .unwrap()
                .proxy_url
                .as_deref(),
            Some("socks5://127.0.0.1:1080")
        );
        assert_eq!(
            repository
                .get(id)
                .await
                .unwrap()
                .unwrap()
                .concurrent_rate
                .as_deref(),
            Some("5/1000")
        );
        let saved = repository.get(id).await.unwrap().unwrap();
        assert_eq!(saved.explore_url.as_deref(), Some("热门::/hot"));
        assert_eq!(
            saved.raw_rules.explore.as_deref(),
            Some(r#"{"bookList":".book"}"#)
        );
    }

    #[tokio::test]
    async fn marking_a_session_expired_preserves_credentials() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let repository = SqliteSourceRepository::new(pool);
        let source = BookSource {
            id: 0,
            name: "expired-test".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com/?q={{key}}".into(),
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
                url: "a::attr(href)".into(),
                next_url: None,
            },
            content_selector: "body".into(),
            next_toc_url_selector: None,
            next_content_url_selector: None,
            header: None,
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: Some("token".into()),
            session_cookie: Some("sid=1".into()),
            session_expires_at: None,
            sign_script: None,
            proxy_url: None,
            concurrent_rate: None,
            enabled: true,
            raw_rules: Default::default(),
        };
        let id = repository.upsert(&source).await.unwrap();
        repository
            .update_session(id, Some("token"), Some("sid=1"), None)
            .await
            .unwrap();
        repository.mark_session_expired(id).await.unwrap();
        let current = repository.get(id).await.unwrap().unwrap();
        assert_eq!(current.access_token.as_deref(), Some("token"));
        assert_eq!(current.session_cookie.as_deref(), Some("sid=1"));
        assert_eq!(current.session_state(), "expired");
    }
}
