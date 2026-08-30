//! Chapter catalog, content and progress orchestration boundary.

use crate::{
    domain::Chapter,
    error::AppError,
    repository::{chapter::SqliteChapterRepository, progress::SqliteProgressRepository, ChapterRepository, ProgressRepository},
    domain::ReadingProgress,
};

pub const MODULE: &str = "reader";

#[derive(Clone)]
pub struct ReaderService {
    chapters: SqliteChapterRepository,
    progress: SqliteProgressRepository,
}

impl ReaderService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            chapters: SqliteChapterRepository::new(pool.clone()),
            progress: SqliteProgressRepository::new(pool),
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

    pub async fn progress(&self, book_id: i64) -> Result<Option<ReadingProgress>, AppError> {
        self.progress.get(book_id).await
    }

    pub async fn save_progress(&self, book_id: i64, chapter_id: i64, offset: i64) -> Result<(), AppError> {
        if offset < 0 { return Err(AppError::InvalidArgument("阅读位置不能为负数".into())); }
        self.progress.save(&ReadingProgress { book_id, chapter_id, offset }).await
    }
}
