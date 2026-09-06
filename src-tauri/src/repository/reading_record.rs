use crate::{domain::ReadingRecord, error::AppError};

#[derive(Clone)]
pub struct SqliteReadingRecordRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteReadingRecordRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, book_id: i64) -> Result<Option<ReadingRecord>, AppError> {
        sqlx::query_as::<_, (i64, i64)>(
            "SELECT book_id, duration_seconds FROM reading_records WHERE book_id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map(|row| {
            row.map(|(book_id, duration_seconds)| ReadingRecord {
                book_id,
                duration_seconds,
            })
        })
        .map_err(|error| AppError::Database(error.to_string()))
    }

    pub async fn add_seconds(&self, book_id: i64, seconds: i64) -> Result<(), AppError> {
        if seconds <= 0 {
            return Ok(());
        }
        sqlx::query(
            "INSERT INTO reading_records (book_id, duration_seconds) SELECT id, ? FROM books WHERE id = ? ON CONFLICT(book_id) DO UPDATE SET duration_seconds = reading_records.duration_seconds + excluded.duration_seconds, updated_at = CURRENT_TIMESTAMP",
        )
        .bind(seconds)
        .bind(book_id)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|error| AppError::Database(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn accumulates_reading_time_and_survives_reopening_repository() {
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

        let repository = SqliteReadingRecordRepository::new(pool.clone());
        repository.add_seconds(1, 12).await.unwrap();
        repository.add_seconds(1, 8).await.unwrap();
        assert_eq!(
            repository.get(1).await.unwrap().unwrap().duration_seconds,
            20
        );

        let reopened = SqliteReadingRecordRepository::new(pool);
        assert_eq!(reopened.get(1).await.unwrap().unwrap().duration_seconds, 20);
    }

    #[tokio::test]
    async fn ignores_unknown_books_and_non_positive_durations() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let repository = SqliteReadingRecordRepository::new(pool);
        repository.add_seconds(404, 10).await.unwrap();
        repository.add_seconds(404, 0).await.unwrap();
        assert!(repository.get(404).await.unwrap().is_none());
    }
}
