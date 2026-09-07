use crate::{domain::DownloadTask, error::AppError};

#[derive(Clone)]
pub struct SqliteDownloadRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteDownloadRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, book_id: i64, chapter_ids: Option<&[i64]>) -> Result<i64, AppError> {
        let ids = chapter_ids
            .map(serde_json::to_string)
            .transpose()
            .map_err(AppError::parse)?;
        sqlx::query("INSERT INTO download_tasks (book_id, chapter_ids) VALUES (?, ?)")
            .bind(book_id)
            .bind(ids)
            .execute(&self.pool)
            .await
            .map(|result| result.last_insert_rowid())
            .map_err(AppError::database)
    }

    pub async fn list(&self) -> Result<Vec<DownloadTask>, AppError> {
        sqlx::query_as("SELECT d.id, d.book_id, b.title AS book_title, d.status, d.total_chapters, d.completed_chapters, d.retry_count, d.error, d.created_at, d.updated_at FROM download_tasks d JOIN books b ON b.id = d.book_id ORDER BY d.created_at DESC")
            .fetch_all(&self.pool).await.map_err(AppError::database)
    }

    pub async fn chapter_ids(&self, task_id: i64) -> Result<Option<Vec<i64>>, AppError> {
        let value = sqlx::query_scalar::<_, Option<String>>(
            "SELECT chapter_ids FROM download_tasks WHERE id = ?",
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database)?
        .flatten();
        value
            .map(|value| serde_json::from_str(&value).map_err(AppError::parse))
            .transpose()
    }

    pub async fn status(&self, task_id: i64) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT status FROM download_tasks WHERE id = ?")
            .bind(task_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::database)
    }

    pub async fn set_status(&self, task_id: i64, status: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE download_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(status)
        .bind(task_id)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(AppError::database)
    }

    pub async fn claim(&self, task_id: i64) -> Result<bool, AppError> {
        sqlx::query("UPDATE download_tasks SET status = 'running', error = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'pending'")
            .bind(task_id).execute(&self.pool).await.map(|r| r.rows_affected() == 1).map_err(AppError::database)
    }

    pub async fn set_total(&self, task_id: i64, total: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE download_tasks SET total_chapters = ?, completed_chapters = 0, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(total).bind(task_id).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    pub async fn advance(&self, task_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE download_tasks SET completed_chapters = completed_chapters + 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(task_id).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    pub async fn fail(&self, task_id: i64, error: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE download_tasks SET status = 'failed', retry_count = retry_count + 1, error = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'running'")
            .bind(error).bind(task_id).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    pub async fn incomplete_ids(&self) -> Result<Vec<i64>, AppError> {
        sqlx::query_scalar("SELECT id FROM download_tasks WHERE status IN ('pending', 'running') ORDER BY created_at")
            .fetch_all(&self.pool).await.map_err(AppError::database)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn tasks_persist_progress_and_do_not_overwrite_cancellation_with_failure() {
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
        let repository = SqliteDownloadRepository::new(pool);
        let id = repository.create(1, Some(&[2, 3])).await.unwrap();
        assert_eq!(repository.chapter_ids(id).await.unwrap(), Some(vec![2, 3]));
        assert!(repository.claim(id).await.unwrap());
        repository.set_total(id, 2).await.unwrap();
        repository.advance(id).await.unwrap();
        repository.set_status(id, "cancelled").await.unwrap();
        repository.fail(id, "late error").await.unwrap();
        let task = repository.list().await.unwrap().remove(0);
        assert_eq!(task.status, "cancelled");
        assert_eq!(task.completed_chapters, 1);
        assert_eq!(task.retry_count, 0);
    }
}
