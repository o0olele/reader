use crate::{domain::BookshelfGroup, error::AppError};

#[derive(Clone)]
pub struct SqliteBookshelfRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteBookshelfRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self { Self { pool } }

    pub async fn list(&self) -> Result<Vec<BookshelfGroup>, AppError> {
        sqlx::query_as::<_, (i64, String, i64)>(
            "SELECT g.id, g.name, COUNT(b.id) FROM bookshelf_groups g LEFT JOIN books b ON b.group_id = g.id GROUP BY g.id ORDER BY g.sort_order, g.id",
        )
        .fetch_all(&self.pool)
        .await
        .map(|rows| rows.into_iter().map(|(id, name, book_count)| BookshelfGroup { id, name, book_count }).collect())
        .map_err(AppError::database)
    }

    pub async fn create(&self, name: &str) -> Result<BookshelfGroup, AppError> {
        let result = sqlx::query("INSERT INTO bookshelf_groups (name) VALUES (?)")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|error| {
                if error.to_string().contains("UNIQUE") { AppError::Source("分组名称已存在".into()) } else { AppError::database(error) }
            })?;
        Ok(BookshelfGroup { id: result.last_insert_rowid(), name: name.to_owned(), book_count: 0 })
    }

    pub async fn move_book(&self, book_id: i64, group_id: i64) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE books SET group_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(group_id).bind(book_id).execute(&self.pool).await.map_err(AppError::database)?;
        if result.rows_affected() == 0 { Err(AppError::Source("书籍不存在".into())) } else { Ok(()) }
    }
}
