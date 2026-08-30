use crate::{
    domain::Book,
    error::AppError,
    repository::{book::SqliteBookRepository, BookRepository},
};

#[derive(Clone)]
pub struct BookService {
    books: SqliteBookRepository,
}

impl BookService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            books: SqliteBookRepository::new(pool),
        }
    }

    pub async fn list(&self) -> Result<Vec<Book>, AppError> {
        self.books.list().await
    }

    pub async fn delete(&self, book_id: i64) -> Result<(), AppError> {
        self.books.delete(book_id).await
    }
}
