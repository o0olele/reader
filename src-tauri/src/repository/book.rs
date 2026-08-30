use super::{BookRepository, NewBook};
use crate::{domain::Book, error::AppError};

#[derive(Clone)]
pub struct SqliteBookRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteBookRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

type BookRow = (
    i64,
    String,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<i64>,
    Option<String>,
    i64,
    String,
);

fn map_book(
    (id, title, author, path, group_id, source_id, remote_url, chapter_count, updated_at): BookRow,
) -> Book {
    Book {
        id,
        title,
        author,
        path,
        group_id,
        source_id,
        remote_url,
        chapter_count,
        updated_at,
    }
}

const BOOK_SELECT: &str = "SELECT b.id, b.title, b.author, b.path, b.group_id, b.source_id, b.remote_url, COUNT(c.id), b.updated_at FROM books b LEFT JOIN chapters c ON c.book_id = b.id";

impl BookRepository for SqliteBookRepository {
    async fn get(&self, id: i64) -> Result<Option<Book>, AppError> {
        sqlx::query_as::<_, BookRow>(&format!("{BOOK_SELECT} WHERE b.id = ? GROUP BY b.id"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| AppError::Database(error.to_string()))
            .map(|row| row.map(map_book))
    }

    async fn list(&self) -> Result<Vec<Book>, AppError> {
        sqlx::query_as::<_, BookRow>(&format!(
            "{BOOK_SELECT} GROUP BY b.id ORDER BY b.updated_at DESC"
        ))
        .fetch_all(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
        .map(|rows| rows.into_iter().map(map_book).collect())
    }

    async fn find_by_path(&self, path: &str) -> Result<Option<Book>, AppError> {
        sqlx::query_as::<_, BookRow>(&format!(
            "{BOOK_SELECT} WHERE b.path = ? GROUP BY b.id LIMIT 1"
        ))
        .bind(path)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
        .map(|row| row.map(map_book))
    }

    async fn create(&self, book: &NewBook<'_>) -> Result<i64, AppError> {
        sqlx::query("INSERT INTO books (title, author, path) VALUES (?, ?, ?)")
            .bind(book.title)
            .bind(book.author)
            .bind(book.path)
            .execute(&self.pool)
            .await
            .map(|result| result.last_insert_rowid())
            .map_err(|error| AppError::Database(error.to_string()))
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|error| AppError::Database(error.to_string()))
    }
}
