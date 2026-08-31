//! Chapter catalog, online content and reading-progress workflows.

use crate::{
    domain::{Chapter, ReadingProgress},
    error::AppError,
    infrastructure::http::{
        client::build_source_client,
        request::{response_error, send_source_request},
    },
    repository::{
        book::SqliteBookRepository, chapter::SqliteChapterRepository,
        progress::SqliteProgressRepository, source::SqliteSourceRepository, BookRepository,
        ChapterRepository, ProgressRepository, SourceRepository,
    },
    service::settings_service::SettingsService,
    source_engine::selector::{parse_catalog, parse_content},
};

#[derive(Clone)]
pub struct ReaderService {
    chapters: SqliteChapterRepository,
    progress: SqliteProgressRepository,
    books: SqliteBookRepository,
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl ReaderService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            chapters: SqliteChapterRepository::new(pool.clone()),
            progress: SqliteProgressRepository::new(pool.clone()),
            books: SqliteBookRepository::new(pool.clone()),
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    pub async fn cached_content(&self, chapter_id: i64) -> Result<Option<String>, AppError> {
        self.chapters.cached_content(chapter_id).await
    }
    pub async fn cache_content(&self, chapter_id: i64, content: &str) -> Result<(), AppError> {
        self.chapters.save_content(chapter_id, content).await
    }
    pub async fn list_chapters(&self, book_id: i64) -> Result<Vec<Chapter>, AppError> {
        self.chapters.list_for_book(book_id).await
    }
    pub async fn replace_catalog(
        &self,
        book_id: i64,
        catalog: &[(String, String)],
    ) -> Result<(), AppError> {
        self.chapters.replace_catalog(book_id, catalog).await
    }

    pub async fn fetch_online_content(
        &self,
        source_id: i64,
        chapter_url: &str,
        chapter_id: Option<i64>,
    ) -> Result<String, AppError> {
        if let Some(id) = chapter_id {
            if let Some(cached) = self.cached_content(id).await? {
                tracing::debug!(target: "reader", chapter_id = id, "chapter cache hit");
                return Ok(cached);
            }
            tracing::debug!(target: "reader", chapter_id = id, "chapter cache miss");
        }
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let response = send_source_request(&client, chapter_url, &source).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let content = parse_content(&source, &response.text().await.map_err(AppError::network)?)
            .map_err(AppError::parse)?;
        if let Some(id) = chapter_id {
            self.cache_content(id, &content).await?;
        }
        Ok(content)
    }

    pub async fn refresh_catalog(&self, book_id: i64) -> Result<Vec<Chapter>, AppError> {
        let book = self
            .books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::Source("书籍不存在".into()))?;
        let source_id = book
            .source_id
            .ok_or_else(|| AppError::Source("本地书籍没有在线书源".into()))?;
        let book_url = book
            .remote_url
            .ok_or_else(|| AppError::Source("书籍没有远程地址".into()))?;
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let request_url = reqwest::Url::parse(&book_url)
            .or_else(|_| {
                reqwest::Url::parse(&source.base_url).and_then(|base| base.join(&book_url))
            })
            .map_err(|e| AppError::InvalidArgument(format!("目录 URL无效: {e}")))?;
        let response = send_source_request(&client, request_url.as_str(), &source).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let catalog = parse_catalog(&source, &response.text().await.map_err(AppError::network)?)
            .map_err(AppError::parse)?;
        tracing::info!(target: "reader", book_id, chapter_count = catalog.len(), "catalog refreshed");
        if catalog.is_empty() {
            return Err(AppError::Source("书源没有解析出目录".into()));
        }
        self.replace_catalog(book_id, &catalog).await?;
        self.list_chapters(book_id).await
    }

    pub async fn progress(&self, book_id: i64) -> Result<Option<ReadingProgress>, AppError> {
        self.progress.get(book_id).await
    }
    pub async fn save_progress(
        &self,
        book_id: i64,
        chapter_id: i64,
        offset: i64,
    ) -> Result<(), AppError> {
        if offset < 0 {
            return Err(AppError::InvalidArgument("阅读位置不能为负数".into()));
        }
        self.progress
            .save(&ReadingProgress {
                book_id,
                chapter_id,
                offset,
            })
            .await
    }
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
    async fn fetch_online_content_returns_cached_body_before_network_lookup() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'https://example.test/book')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'One', '', 'https://example.test/chapter')")
            .execute(&pool)
            .await
            .unwrap();
        let service = ReaderService::new(pool);
        service.cache_content(1, "cached body").await.unwrap();

        let content = service
            .fetch_online_content(999, "not a URL", Some(1))
            .await
            .unwrap();

        assert_eq!(content, "cached body");
    }

    #[tokio::test]
    async fn service_catalog_operations_preserve_cached_content() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        let service = ReaderService::new(pool);
        service
            .replace_catalog(1, &[("One".into(), "https://example.test/one".into())])
            .await
            .unwrap();
        service.cache_content(1, "cached body").await.unwrap();
        service
            .replace_catalog(
                1,
                &[
                    ("Updated".into(), "https://example.test/one".into()),
                    ("Two".into(), "https://example.test/two".into()),
                ],
            )
            .await
            .unwrap();

        let chapters = service.list_chapters(1).await.unwrap();
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].content, "cached body");
        assert_eq!(chapters[0].title, "Updated");
    }
}
