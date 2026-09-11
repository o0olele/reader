//! Chapter catalog, online content and reading-progress workflows.

mod cache;
mod online;
mod prefetch;
mod processing;
#[cfg(test)]
mod processing_tests;
use cache::memory_cache;
pub(crate) use prefetch::cancel_prefetch;

use crate::{
    domain::{Chapter, ReadingProgress, ReadingRecord, ReadingStats},
    error::AppError,
    repository::{
        book::SqliteBookRepository, chapter::SqliteChapterRepository,
        progress::SqliteProgressRepository, reading_record::SqliteReadingRecordRepository,
        source::SqliteSourceRepository, BookRepository, ChapterRepository, ProgressRepository,
        SourceRepository,
    },
    service::settings_service::SettingsService,
};

#[derive(Clone)]
pub struct ReaderService {
    pool: sqlx::SqlitePool,
    chapters: SqliteChapterRepository,
    progress: SqliteProgressRepository,
    reading_records: SqliteReadingRecordRepository,
    books: SqliteBookRepository,
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl ReaderService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
            chapters: SqliteChapterRepository::new(pool.clone()),
            progress: SqliteProgressRepository::new(pool.clone()),
            reading_records: SqliteReadingRecordRepository::new(pool.clone()),
            books: SqliteBookRepository::new(pool.clone()),
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    pub async fn cached_content(&self, chapter_id: i64) -> Result<Option<String>, AppError> {
        if let Some(value) = memory_cache()
            .lock()
            .map_err(|_| AppError::Database("阅读缓存锁不可用".into()))?
            .get(chapter_id)
        {
            return Ok(Some(value));
        }
        self.chapters.cached_content(chapter_id).await
    }
    pub async fn cache_content(&self, chapter_id: i64, content: &str) -> Result<(), AppError> {
        crate::service::cache_service::CacheService::new(self.pool.clone())
            .store(chapter_id, content)
            .await?;
        memory_cache()
            .lock()
            .map_err(|_| AppError::Database("阅读缓存锁不可用".into()))?
            .put(chapter_id, content.to_owned());
        Ok(())
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
                anchor_index: 0,
                anchor_ratio: 0.0,
            })
            .await
    }

    pub async fn save_progress_anchor(
        &self,
        book_id: i64,
        chapter_id: i64,
        offset: i64,
        anchor_index: i64,
        anchor_ratio: f64,
    ) -> Result<(), AppError> {
        if offset < 0 || anchor_index < 0 || !(0.0..=1.0).contains(&anchor_ratio) {
            return Err(AppError::InvalidArgument("阅读位置无效".into()));
        }
        self.progress
            .save(&ReadingProgress {
                book_id,
                chapter_id,
                offset,
                anchor_index,
                anchor_ratio,
            })
            .await
    }

    pub async fn reading_record(&self, book_id: i64) -> Result<Option<ReadingRecord>, AppError> {
        self.reading_records.get(book_id).await
    }

    pub async fn add_reading_time(
        &self,
        book_id: i64,
        duration_seconds: i64,
    ) -> Result<(), AppError> {
        if duration_seconds < 0 {
            return Err(AppError::InvalidArgument("阅读时长不能为负数".into()));
        }
        self.reading_records
            .add_seconds(book_id, duration_seconds)
            .await
    }

    /// Home-dashboard aggregate: totals, today, streak, goal and finished books.
    pub async fn reading_stats(&self) -> Result<ReadingStats, AppError> {
        let goal = self.settings.reading_goal_minutes().await?;
        self.reading_records.stats(goal).await
    }

    pub async fn set_reading_goal(&self, minutes: i64) -> Result<ReadingStats, AppError> {
        self.settings.save_reading_goal_minutes(minutes).await?;
        self.reading_records.stats(minutes).await
    }
}

pub(crate) fn clear_memory_cache() -> Result<(), AppError> {
    memory_cache()
        .lock()
        .map_err(|_| AppError::Database("阅读缓存锁不可用".into()))?
        .clear();
    Ok(())
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

    #[tokio::test]
    async fn semantic_progress_anchor_round_trips() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, 'One', 'body')",
        )
        .execute(&pool)
        .await
        .unwrap();
        let service = ReaderService::new(pool);
        service
            .save_progress_anchor(1, 1, 480, 12, 0.375)
            .await
            .unwrap();
        let progress = service.progress(1).await.unwrap().unwrap();
        assert_eq!(progress.anchor_index, 12);
        assert!((progress.anchor_ratio - 0.375).abs() < f64::EPSILON);
    }
}
