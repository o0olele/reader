use crate::{
    domain::Book,
    error::AppError,
    repository::{book::SqliteBookRepository, BookRepository},
    source::BookSearchResult,
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

    pub async fn add_online(&self, result: &BookSearchResult) -> Result<Book, AppError> {
        if let Some(book) = self.books.find_by_path(&result.url).await? { return Ok(book); }
        let id = self.books.create_online(&result.title, result.author.as_deref(), &result.url, result.source_id).await?;
        self.books.get(id).await?.ok_or_else(|| AppError::Database("加入书架后无法读取书籍".into()))
    }
}
