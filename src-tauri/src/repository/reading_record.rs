use crate::{
    domain::{ReadingRecord, ReadingStats},
    error::AppError,
};

/// SQLite expression for "today" in the user's local timezone. Computed by
/// SQLite so the backend never needs a date library.
const TODAY: &str = "date('now','localtime')";

/// Counts consecutive days with reading time, ending today. `n` starts at 1
/// only when today has time, so a gap immediately ends the streak.
const STREAK_SQL: &str = "WITH RECURSIVE streak(day, n) AS (
        SELECT date('now','localtime'),
               (SELECT CASE WHEN EXISTS(SELECT 1 FROM reading_daily WHERE day = date('now','localtime') AND seconds > 0) THEN 1 ELSE 0 END)
        UNION ALL
        SELECT date(streak.day, '-1 day'), streak.n + 1
        FROM streak
        WHERE streak.n > 0
          AND EXISTS (SELECT 1 FROM reading_daily d WHERE d.day = date(streak.day, '-1 day') AND d.seconds > 0)
    )
    SELECT COALESCE(MAX(n), 0) FROM streak";

const FINISHED_SQL: &str = "SELECT COUNT(*) FROM reading_progress p
    JOIN chapters c ON c.id = p.chapter_id
    WHERE c.number = (SELECT MAX(number) FROM chapters WHERE book_id = p.book_id)";

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
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        let written = sqlx::query(
            "INSERT INTO reading_records (book_id, duration_seconds) SELECT id, ? FROM books WHERE id = ? ON CONFLICT(book_id) DO UPDATE SET duration_seconds = reading_records.duration_seconds + excluded.duration_seconds, updated_at = CURRENT_TIMESTAMP",
        )
        .bind(seconds)
        .bind(book_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;
        // Unknown books must not create a phantom day of reading time.
        if written.rows_affected() > 0 {
            sqlx::query(
                "INSERT INTO reading_daily (day, seconds) VALUES (date('now','localtime'), ?) ON CONFLICT(day) DO UPDATE SET seconds = reading_daily.seconds + excluded.seconds, updated_at = CURRENT_TIMESTAMP",
            )
            .bind(seconds)
            .execute(&mut *transaction)
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        }
        transaction
            .commit()
            .await
            .map_err(|error| AppError::Database(error.to_string()))
    }

    pub async fn stats(&self, daily_goal_minutes: i64) -> Result<ReadingStats, AppError> {
        let total_seconds: i64 =
            sqlx::query_scalar("SELECT COALESCE(SUM(duration_seconds), 0) FROM reading_records")
                .fetch_one(&self.pool)
                .await
                .map_err(|error| AppError::Database(error.to_string()))?;
        let today_seconds: i64 = sqlx::query_scalar::<_, i64>(&format!(
            "SELECT COALESCE(seconds, 0) FROM reading_daily WHERE day = {TODAY}"
        ))
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?
        .unwrap_or(0);
        let streak_days: i64 = sqlx::query_scalar(STREAK_SQL)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        let finished_books: i64 = sqlx::query_scalar(FINISHED_SQL)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;

        Ok(ReadingStats {
            total_seconds,
            today_seconds,
            daily_goal_minutes,
            streak_days,
            finished_books,
        })
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
    async fn accumulates_reading_time_and_survives_reopening_repository() {
        let pool = pool().await;
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
        let pool = pool().await;
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        repository.add_seconds(404, 10).await.unwrap();
        repository.add_seconds(404, 0).await.unwrap();
        assert!(repository.get(404).await.unwrap().is_none());
        let days: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reading_daily")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(days, 0, "unknown books must not create a reading day");
    }

    #[tokio::test]
    async fn stats_report_totals_today_streak_and_finished_books() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, 'One', 'a'), (1, 1, 'Two', 'b')")
            .execute(&pool)
            .await
            .unwrap();
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        repository.add_seconds(1, 300).await.unwrap();
        // A gap before yesterday must not extend the streak.
        sqlx::query("INSERT INTO reading_daily (day, seconds) VALUES (date('now','localtime','-1 day'), 60), (date('now','localtime','-3 day'), 60)")
            .execute(&pool)
            .await
            .unwrap();
        let last_chapter: i64 = sqlx::query_scalar("SELECT id FROM chapters WHERE book_id = 1 AND number = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO reading_progress (book_id, chapter_id) VALUES (1, ?)")
            .bind(last_chapter)
            .execute(&pool)
            .await
            .unwrap();

        let stats = repository.stats(30).await.unwrap();
        assert_eq!(stats.total_seconds, 300);
        assert_eq!(stats.today_seconds, 300);
        assert_eq!(stats.streak_days, 2);
        assert_eq!(stats.daily_goal_minutes, 30);
        assert_eq!(stats.finished_books, 1);
    }

    #[tokio::test]
    async fn streak_is_zero_without_reading_today() {
        let pool = pool().await;
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        sqlx::query("INSERT INTO reading_daily (day, seconds) VALUES (date('now','localtime','-1 day'), 60)")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(repository.stats(20).await.unwrap().streak_days, 0);
    }
}
