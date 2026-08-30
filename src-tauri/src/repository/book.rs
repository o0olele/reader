use super::BookRepository;
use crate::{domain::Book, error::AppError};

#[derive(Clone)]
pub struct SqliteBookRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteBookRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_online(
        &self,
        title: &str,
        author: Option<&str>,
        url: &str,
        source_id: i64,
    ) -> Result<i64, AppError> {
        sqlx::query("INSERT INTO books (title, author, path, source_id, remote_url, group_id) VALUES (?, ?, ?, ?, ?, (SELECT id FROM bookshelf_groups WHERE name = '默认书架' LIMIT 1))")
            .bind(title).bind(author).bind(url).bind(source_id).bind(url)
            .execute(&self.pool).await.map(|result| result.last_insert_rowid()).map_err(AppError::database)
    }

    /// Inserts a locally imported book and its chapters in one transaction, so
    /// a failure part-way through leaves no half-imported book on the shelf.
    pub async fn create_local_with_chapters(
        &self,
        title: &str,
        author: Option<&str>,
        path: &str,
        chapters: &[(String, String)],
    ) -> Result<i64, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::database)?;
        let inserted = sqlx::query("INSERT INTO books (title, author, path, group_id) VALUES (?, ?, ?, (SELECT id FROM bookshelf_groups WHERE name = '默认书架' LIMIT 1))")
            .bind(title).bind(author).bind(path)
            .execute(&mut *tx).await.map_err(AppError::database)?;
        let book_id = inserted.last_insert_rowid();
        for (number, (chapter_title, content)) in chapters.iter().enumerate() {
            sqlx::query(
                "INSERT INTO chapters (book_id, number, title, content) VALUES (?, ?, ?, ?)",
            )
            .bind(book_id)
            .bind(number as i64)
            .bind(chapter_title)
            .bind(content)
            .execute(&mut *tx)
            .await
            .map_err(AppError::database)?;
        }
        tx.commit().await.map_err(AppError::database)?;
        Ok(book_id)
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

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|error| AppError::Database(error.to_string()))
    }
}
