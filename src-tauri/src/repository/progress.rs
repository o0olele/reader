use super::ProgressRepository;
use crate::{domain::ReadingProgress, error::AppError};

#[derive(Clone)]
pub struct SqliteProgressRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteProgressRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

impl ProgressRepository for SqliteProgressRepository {
    async fn get(&self, book_id: i64) -> Result<Option<ReadingProgress>, AppError> {
        sqlx::query_as::<_, (i64, i64, i64, i64, f64)>(
            "SELECT book_id, chapter_id, offset, anchor_index, anchor_ratio FROM reading_progress WHERE book_id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
        .map(|row| {
            row.map(|(book_id, chapter_id, offset, anchor_index, anchor_ratio)| ReadingProgress {
                book_id,
                chapter_id,
                offset,
                anchor_index,
                anchor_ratio,
            })
        })
    }

    async fn save(&self, progress: &ReadingProgress) -> Result<(), AppError> {
        sqlx::query("INSERT INTO reading_progress (book_id, chapter_id, offset, anchor_index, anchor_ratio) VALUES (?, ?, ?, ?, ?) ON CONFLICT(book_id) DO UPDATE SET chapter_id = excluded.chapter_id, offset = excluded.offset, anchor_index = excluded.anchor_index, anchor_ratio = excluded.anchor_ratio, updated_at = CURRENT_TIMESTAMP")
            .bind(progress.book_id)
            .bind(progress.chapter_id)
            .bind(progress.offset)
            .bind(progress.anchor_index)
            .bind(progress.anchor_ratio)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|error| AppError::Database(error.to_string()))
    }
}
