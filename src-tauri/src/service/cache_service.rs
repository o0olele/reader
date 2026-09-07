use crate::{
    error::AppError, repository::chapter::SqliteChapterRepository,
    service::settings_service::SettingsService,
};
use serde::Serialize;
use std::sync::OnceLock;

const DEFAULT_QUOTA_MB: i64 = 1024;
const MIN_QUOTA_MB: i64 = 64;
const MAX_QUOTA_MB: i64 = 102_400;
static CACHE_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
pub struct CacheStats {
    pub used_bytes: i64,
    pub quota_bytes: i64,
    pub cached_chapters: i64,
}

#[derive(Clone)]
pub struct CacheService {
    pool: sqlx::SqlitePool,
    chapters: SqliteChapterRepository,
    settings: SettingsService,
}

impl CacheService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            chapters: SqliteChapterRepository::new(pool.clone()),
            settings: SettingsService::new(pool.clone()),
            pool,
        }
    }

    pub async fn stats(&self) -> Result<CacheStats, AppError> {
        let (used_bytes, cached_chapters) = sqlx::query_as::<_, (i64, i64)>(
            "SELECT COALESCE(SUM(length(CAST(content AS BLOB))), 0), COUNT(*) FROM chapter_contents",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::database)?;
        Ok(CacheStats {
            used_bytes,
            quota_bytes: self.quota_bytes().await?,
            cached_chapters,
        })
    }

    pub async fn set_quota(&self, megabytes: i64) -> Result<CacheStats, AppError> {
        if !(MIN_QUOTA_MB..=MAX_QUOTA_MB).contains(&megabytes) {
            return Err(AppError::InvalidArgument(format!(
                "缓存上限须在 {MIN_QUOTA_MB} MB 到 {MAX_QUOTA_MB} MB 之间"
            )));
        }
        let _guard = cache_lock().lock().await;
        self.settings.save_cache_quota_mb(megabytes).await?;
        self.trim(None).await?;
        self.stats().await
    }

    pub async fn clear(&self) -> Result<CacheStats, AppError> {
        let _guard = cache_lock().lock().await;
        sqlx::query("DELETE FROM chapter_contents")
            .execute(&self.pool)
            .await
            .map_err(AppError::database)?;
        crate::service::reader_service::clear_memory_cache()?;
        self.stats().await
    }

    pub async fn store(&self, chapter_id: i64, content: &str) -> Result<(), AppError> {
        let _guard = cache_lock().lock().await;
        self.chapters.save_content(chapter_id, content).await?;
        let book_id = sqlx::query_scalar::<_, i64>("SELECT book_id FROM chapters WHERE id = ?")
            .bind(chapter_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::database)?;
        if self.trim(book_id).await? > 0 {
            crate::service::reader_service::clear_memory_cache()?;
        }
        Ok(())
    }

    async fn quota_bytes(&self) -> Result<i64, AppError> {
        Ok(self
            .settings
            .cache_quota_mb()
            .await?
            .unwrap_or(DEFAULT_QUOTA_MB)
            .clamp(MIN_QUOTA_MB, MAX_QUOTA_MB)
            * 1024
            * 1024)
    }

    /// Keeps the active book intact and evicts the oldest cached chapters from
    /// other books first. A deliberately downloaded book may temporarily make
    /// the cache exceed the soft quota rather than becoming incomplete.
    async fn trim(&self, protected_book_id: Option<i64>) -> Result<usize, AppError> {
        let quota = self.quota_bytes().await?;
        let mut used = self.stats().await?.used_bytes;
        if used <= quota {
            return Ok(0);
        }
        let candidates = sqlx::query_as::<_, (i64, i64)>(
            "SELECT cc.chapter_id, length(CAST(cc.content AS BLOB)) FROM chapter_contents cc JOIN chapters c ON c.id = cc.chapter_id WHERE (? IS NULL OR c.book_id != ?) ORDER BY cc.cached_at, cc.chapter_id",
        )
        .bind(protected_book_id)
        .bind(protected_book_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::database)?;
        let mut removed = 0;
        for (chapter_id, bytes) in candidates {
            if used <= quota {
                break;
            }
            sqlx::query("DELETE FROM chapter_contents WHERE chapter_id = ?")
                .bind(chapter_id)
                .execute(&self.pool)
                .await
                .map_err(AppError::database)?;
            used = used.saturating_sub(bytes);
            removed += 1;
        }
        Ok(removed)
    }
}

fn cache_lock() -> &'static tokio::sync::Mutex<()> {
    CACHE_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn clear_preserves_local_chapters_and_removes_online_cache() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, 'One', 'local body')").execute(&pool).await.unwrap();
        let service = CacheService::new(pool.clone());
        service.store(1, "cached body").await.unwrap();
        assert_eq!(service.stats().await.unwrap().cached_chapters, 1);
        service.clear().await.unwrap();
        assert_eq!(service.stats().await.unwrap().cached_chapters, 0);
        let local: String = sqlx::query_scalar("SELECT content FROM chapters WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(local, "local body");
    }
}
