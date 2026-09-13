use crate::{
    domain::{Bookmark, BookmarkEntry},
    error::AppError,
};

pub struct SqliteBookmarkRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteBookmarkRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, book_id: i64, chapter_id: i64) -> Result<Option<Bookmark>, AppError> {
        sqlx::query_as("SELECT book_id, chapter_id, offset, mode FROM bookmarks WHERE book_id = ? AND chapter_id = ?")
            .bind(book_id).bind(chapter_id).fetch_optional(&self.pool).await.map_err(AppError::database)
    }

    pub async fn save(&self, bookmark: &Bookmark) -> Result<(), AppError> {
        if bookmark.offset < 0 || !matches!(bookmark.mode.as_str(), "scroll" | "paged") {
            return Err(AppError::InvalidArgument("无效的书签位置或阅读模式".into()));
        }
        let result = sqlx::query("INSERT INTO bookmarks (book_id, chapter_id, offset, mode) SELECT book_id, id, ?, ? FROM chapters WHERE book_id = ? AND id = ? ON CONFLICT(chapter_id) DO UPDATE SET offset = excluded.offset, mode = excluded.mode, updated_at = CURRENT_TIMESTAMP")
            .bind(bookmark.offset).bind(&bookmark.mode).bind(bookmark.book_id).bind(bookmark.chapter_id)
            .execute(&self.pool).await.map_err(AppError::database)?;
        if result.rows_affected() == 0 {
            return Err(AppError::InvalidArgument(
                "章节不属于此书籍或已被删除".into(),
            ));
        }
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, chapter_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM bookmarks WHERE book_id = ? AND chapter_id = ?")
            .bind(book_id)
            .bind(chapter_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::database)?;
        Ok(())
    }

    /// Every bookmark across the library, newest first. `updated_at` has
    /// second resolution, so title and chapter number break the tie and keep
    /// the order stable between refreshes.
    pub async fn list(&self) -> Result<Vec<BookmarkEntry>, AppError> {
        sqlx::query_as(
            "SELECT b.book_id, k.title AS book_title, k.author AS book_author,
                    b.chapter_id, c.title AS chapter_title, c.number AS chapter_number,
                    b.offset, b.mode, b.updated_at
             FROM bookmarks b
             JOIN books k ON k.id = b.book_id
             JOIN chapters c ON c.id = b.chapter_id
             ORDER BY b.updated_at DESC, k.title ASC, c.number ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::database)
    }
}

#[cfg(test)]
#[path = "bookmark_tests.rs"]
mod tests;
