//! Chapter catalog, content and progress orchestration boundary.

use crate::{
    error::AppError,
    repository::chapter::SqliteChapterRepository,
};

pub const MODULE: &str = "reader";

#[derive(Clone)]
pub struct ReaderService {
    chapters: SqliteChapterRepository,
}

impl ReaderService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            chapters: SqliteChapterRepository::new(pool),
        }
    }

    pub async fn cached_content(&self, chapter_id: i64) -> Result<Option<String>, AppError> {
        self.chapters.cached_content(chapter_id).await
    }

    pub async fn cache_content(&self, chapter_id: i64, content: &str) -> Result<(), AppError> {
        self.chapters.save_content(chapter_id, content).await
    }
}
