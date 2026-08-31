use super::ChapterRepository;
use crate::{domain::Chapter, error::AppError};

#[derive(Clone)]
pub struct SqliteChapterRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteChapterRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn cached_content(&self, chapter_id: i64) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT content FROM chapter_contents WHERE chapter_id = ?")
            .bind(chapter_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
    }

    pub async fn save_content(&self, chapter_id: i64, content: &str) -> Result<(), AppError> {
        sqlx::query("INSERT INTO chapter_contents (chapter_id, content, cached_at) VALUES (?, ?, CURRENT_TIMESTAMP) ON CONFLICT(chapter_id) DO UPDATE SET content = excluded.content, cached_at = CURRENT_TIMESTAMP")
            .bind(chapter_id).bind(content).execute(&self.pool).await
            .map(|_| ()).map_err(|error| AppError::Database(error.to_string()))
    }

    pub async fn replace_catalog(
        &self,
        book_id: i64,
        catalog: &[(String, String)],
    ) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        sqlx::query("DELETE FROM chapters WHERE book_id = ? AND number >= ?")
            .bind(book_id)
            .bind(catalog.len() as i64)
            .execute(&mut *tx)
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        for (number, (title, url)) in catalog.iter().enumerate() {
            sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (?, ?, ?, '', ?) ON CONFLICT(book_id, number) DO UPDATE SET title = excluded.title, remote_url = excluded.remote_url")
                .bind(book_id).bind(number as i64).bind(title).bind(url)
                .execute(&mut *tx).await.map_err(|error| AppError::Database(error.to_string()))?;
        }
        tx.commit()
            .await
            .map_err(|error| AppError::Database(error.to_string()))
    }
}

impl ChapterRepository for SqliteChapterRepository {
    async fn list_for_book(&self, book_id: i64) -> Result<Vec<Chapter>, AppError> {
        sqlx::query_as::<_, (i64, i64, String, i64, String, Option<String>)>(
            "SELECT c.id, c.book_id, c.title, c.number, COALESCE(cc.content, c.content), c.remote_url FROM chapters c LEFT JOIN chapter_contents cc ON cc.chapter_id = c.id WHERE c.book_id = ? ORDER BY c.number",
        )
        .bind(book_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
        .map(|rows| {
            rows.into_iter()
                .map(|(id, book_id, title, number, content, remote_url)| Chapter {
                    id,
                    book_id,
                    title,
                    number,
                    content,
                    remote_url,
                })
                .collect()
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
            .expect("in-memory sqlite");
        sqlx::query("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT NOT NULL);")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE chapters (id INTEGER PRIMARY KEY AUTOINCREMENT, book_id INTEGER NOT NULL, number INTEGER NOT NULL, title TEXT NOT NULL, content TEXT NOT NULL, remote_url TEXT, UNIQUE(book_id, number));")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE chapter_contents (chapter_id INTEGER PRIMARY KEY, content TEXT NOT NULL, cached_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);")
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn chapter_content_cache_round_trips() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (id, title) VALUES (1, 'Book')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'One', '', 'https://example.test/1')")
            .execute(&pool).await.unwrap();
        let repository = SqliteChapterRepository::new(pool);
        repository.save_content(1, "cached body").await.unwrap();
        assert_eq!(
            repository.cached_content(1).await.unwrap().as_deref(),
            Some("cached body")
        );
    }

    #[tokio::test]
    async fn catalog_refresh_preserves_existing_content_cache() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (id, title) VALUES (1, 'Book')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'Old', '', 'https://example.test/1')")
            .execute(&pool).await.unwrap();
        let repository = SqliteChapterRepository::new(pool);
        repository.save_content(1, "cached body").await.unwrap();
        repository
            .replace_catalog(
                1,
                &[
                    ("Updated".into(), "https://example.test/1".into()),
                    ("New".into(), "https://example.test/2".into()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(
            repository.cached_content(1).await.unwrap().as_deref(),
            Some("cached body")
        );
        assert_eq!(repository.list_for_book(1).await.unwrap().len(), 2);
    }
}
