use crate::{
    domain::{ReadingHistoryEntry, ReadingRecord, ReadingStats},
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

/// One row per book the user has read: the position from `reading_progress`,
/// the time from `reading_records`, and the newest of the two timestamps as
/// "last read". Both tables are sparse — 换源 clears the position while the
/// time survives, and a session shorter than one timer tick writes time only —
/// so the row set is their union, never a join that would hide either kind.
/// `chapter_id` comes from the joined catalog row so a position whose chapter
/// was removed reads as "no position" instead of a dangling id; `has_progress`
/// keeps that distinguishable from a book that never saved a position at all.
const HISTORY_SQL: &str = "WITH activity AS (
        SELECT book_id, MAX(stamp) AS last_read_at FROM (
            SELECT book_id, updated_at AS stamp FROM reading_progress
            UNION ALL
            SELECT book_id, updated_at AS stamp FROM reading_records
        ) GROUP BY book_id
    )
    SELECT a.book_id, b.title AS book_title, b.author AS book_author, b.cover_data,
           c.id AS chapter_id, c.title AS chapter_title, c.number AS chapter_number,
           p.book_id IS NOT NULL AS has_progress,
           (SELECT COUNT(*) FROM chapters WHERE book_id = a.book_id) AS chapter_count,
           COALESCE(r.duration_seconds, 0) AS duration_seconds, a.last_read_at
    FROM activity a
    JOIN books b ON b.id = a.book_id
    LEFT JOIN reading_progress p ON p.book_id = a.book_id
    LEFT JOIN reading_records r ON r.book_id = a.book_id
    LEFT JOIN chapters c ON c.id = p.chapter_id
    ORDER BY a.last_read_at DESC, b.title ASC";

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

    /// Every book with a saved position or recorded reading time, newest first
    /// (`updated_at` has second resolution, so the title breaks the tie and keeps
    /// the order stable between refreshes).
    pub async fn history(&self) -> Result<Vec<ReadingHistoryEntry>, AppError> {
        sqlx::query_as(HISTORY_SQL)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::database)
    }

    /// Forgets one book's history, or the whole library's when `book_id` is
    /// `None`. The position lives in `reading_progress` but is deleted by the
    /// same act: a reader who asked to forget a book must not resume into an
    /// invisible position it never showed them.
    pub async fn clear(&self, book_id: Option<i64>) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await.map_err(AppError::database)?;
        match book_id {
            Some(book_id) => {
                for table in ["reading_records", "reading_progress"] {
                    sqlx::query(&format!("DELETE FROM {table} WHERE book_id = ?"))
                        .bind(book_id)
                        .execute(&mut *transaction)
                        .await
                        .map_err(AppError::database)?;
                }
            }
            None => {
                // `reading_daily` carries no book id, so only a full clear can
                // drop it. Keeping it would leave today's minutes and the streak
                // alive while every total reads zero.
                for table in ["reading_records", "reading_progress", "reading_daily"] {
                    sqlx::query(&format!("DELETE FROM {table}"))
                        .execute(&mut *transaction)
                        .await
                        .map_err(AppError::database)?;
                }
            }
        }
        transaction.commit().await.map_err(AppError::database)
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
        let last_chapter: i64 =
            sqlx::query_scalar("SELECT id FROM chapters WHERE book_id = 1 AND number = 1")
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

    /// Timestamps are written with second resolution, so tests set them
    /// explicitly instead of racing the clock for an order.
    async fn stamp(pool: &sqlx::SqlitePool, table: &str, book_id: i64, value: &str) {
        sqlx::query(&format!(
            "UPDATE {table} SET updated_at = ? WHERE book_id = ?"
        ))
        .bind(value)
        .bind(book_id)
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn history_unions_position_and_time_rows_ordered_by_last_read() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO books (title, author, path) VALUES ('甲', '作者甲', 'local'), ('乙', NULL, 'local'), ('丙', NULL, 'local')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, '第一章', 'a'), (1, 1, '第二章', 'b'), (3, 0, '开始', 'c')")
            .execute(&pool)
            .await
            .unwrap();
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        // 甲: position + time. 乙: time only (a session under one timer tick).
        // 丙: position only (read, but the flush never landed).
        repository.add_seconds(1, 90).await.unwrap();
        repository.add_seconds(2, 30).await.unwrap();
        let chapter: i64 =
            sqlx::query_scalar("SELECT id FROM chapters WHERE book_id = 1 AND number = 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        sqlx::query("INSERT INTO reading_progress (book_id, chapter_id) VALUES (1, ?), (3, 3)")
            .bind(chapter)
            .execute(&pool)
            .await
            .unwrap();
        stamp(&pool, "reading_progress", 1, "2026-09-16 10:00:00").await;
        stamp(&pool, "reading_records", 1, "2026-09-16 09:00:00").await;
        stamp(&pool, "reading_records", 2, "2026-09-15 20:00:00").await;
        stamp(&pool, "reading_progress", 3, "2026-09-14 08:00:00").await;

        let entries = repository.history().await.unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.book_title.as_str())
                .collect::<Vec<_>>(),
            vec!["甲", "乙", "丙"]
        );

        let first = &entries[0];
        // The position (10:00) is newer than the time flush (09:00).
        assert_eq!(first.last_read_at, "2026-09-16 10:00:00");
        assert_eq!(first.book_author.as_deref(), Some("作者甲"));
        assert_eq!(first.chapter_number, Some(1));
        assert_eq!(first.chapter_title.as_deref(), Some("第二章"));
        assert_eq!(first.chapter_count, 2);
        assert_eq!(first.duration_seconds, 90);
        assert!(first.has_progress);

        assert!(
            !entries[1].has_progress,
            "time-only row never saved a position"
        );
        assert_eq!(entries[1].chapter_id, None, "time-only row has no position");
        assert_eq!(entries[1].chapter_count, 0);
        assert_eq!(entries[1].duration_seconds, 30);

        assert!(entries[2].has_progress);
        assert_eq!(entries[2].duration_seconds, 0);
        assert_eq!(entries[2].last_read_at, "2026-09-14 08:00:00");
    }

    #[tokio::test]
    async fn history_keeps_a_book_whose_chapter_left_the_catalog() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Book', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, 'One', 'a')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO reading_progress (book_id, chapter_id) VALUES (1, 1)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM chapters WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();

        let entries = SqliteReadingRecordRepository::new(pool)
            .history()
            .await
            .unwrap();
        assert_eq!(entries.len(), 1, "the book is still history");
        assert_eq!(entries[0].chapter_id, None);
        assert_eq!(entries[0].chapter_title, None);
        assert!(
            entries[0].has_progress,
            "a lost chapter must stay distinguishable from never having a position"
        );
    }

    #[tokio::test]
    async fn clearing_one_book_leaves_the_others_and_the_daily_totals_alone() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('甲', 'local'), ('乙', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (1, 0, 'One', 'a'), (2, 0, 'One', 'b')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO reading_progress (book_id, chapter_id) VALUES (1, 1), (2, 2)")
            .execute(&pool)
            .await
            .unwrap();
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        repository.add_seconds(1, 60).await.unwrap();
        repository.add_seconds(2, 120).await.unwrap();

        repository.clear(Some(1)).await.unwrap();

        let entries = repository.history().await.unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.book_id)
                .collect::<Vec<_>>(),
            vec![2]
        );
        assert!(repository.get(1).await.unwrap().is_none());
        let progress: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM reading_progress WHERE book_id = 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(progress, 0, "forgetting a book also drops its position");
        // A per-day row cannot be attributed to one book, so it survives.
        assert_eq!(repository.stats(30).await.unwrap().today_seconds, 180);
    }

    #[tokio::test]
    async fn clearing_everything_resets_history_daily_and_stats_but_keeps_the_books() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('甲', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        let repository = SqliteReadingRecordRepository::new(pool.clone());
        repository.add_seconds(1, 300).await.unwrap();

        repository.clear(None).await.unwrap();

        assert!(repository.history().await.unwrap().is_empty());
        let stats = repository.stats(30).await.unwrap();
        assert_eq!(stats.total_seconds, 0);
        assert_eq!(stats.today_seconds, 0);
        assert_eq!(stats.streak_days, 0, "a cleared day must not keep a streak");
        let books: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM books")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(books, 1, "clearing history never removes a book");
    }
}
