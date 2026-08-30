//! Persistence contracts used by application services.
//!
//! The current SQLite queries still live in the legacy command module. These
//! traits define the Step 0 boundary so implementations can be moved without
//! changing IPC payloads.

use crate::{
    domain::{Book, Chapter, ReadingProgress},
    error::AppError,
};

pub mod book;
pub mod chapter;
pub mod bookshelf;
pub mod progress;
pub mod source;

pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: Option<&'a str>,
    pub path: Option<&'a str>,
}

pub trait BookRepository: Send + Sync {
    async fn get(&self, id: i64) -> Result<Option<Book>, AppError>;
    async fn list(&self) -> Result<Vec<Book>, AppError>;
    async fn find_by_path(&self, path: &str) -> Result<Option<Book>, AppError>;
    async fn create(&self, book: &NewBook<'_>) -> Result<i64, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

pub trait ChapterRepository: Send + Sync {
    async fn list_for_book(&self, book_id: i64) -> Result<Vec<Chapter>, AppError>;
}

pub trait ProgressRepository: Send + Sync {
    async fn get(&self, book_id: i64) -> Result<Option<ReadingProgress>, AppError>;
    async fn save(&self, progress: &ReadingProgress) -> Result<(), AppError>;
}

pub trait SourceRepository: Send + Sync {
    async fn get(&self, id: i64) -> Result<Option<crate::source::BookSource>, AppError>;
    async fn list(&self) -> Result<Vec<crate::source::BookSource>, AppError>;
}
