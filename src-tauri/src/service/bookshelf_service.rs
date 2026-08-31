use crate::{
    domain::BookshelfGroup, error::AppError, repository::bookshelf::SqliteBookshelfRepository,
};

#[derive(Clone)]
pub struct BookshelfService {
    groups: SqliteBookshelfRepository,
}

impl BookshelfService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            groups: SqliteBookshelfRepository::new(pool),
        }
    }
    pub async fn list(&self) -> Result<Vec<BookshelfGroup>, AppError> {
        self.groups.list().await
    }
    pub async fn create(&self, name: &str) -> Result<BookshelfGroup, AppError> {
        self.groups.create(name).await
    }
    pub async fn move_book(&self, book_id: i64, group_id: i64) -> Result<(), AppError> {
        self.groups.move_book(book_id, group_id).await
    }
}
