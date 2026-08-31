//! Entity types shared by services, repositories and the IPC layer.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub path: Option<String>,
    pub group_id: Option<i64>,
    pub source_id: Option<i64>,
    pub remote_url: Option<String>,
    pub chapter_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    pub id: i64,
    pub book_id: i64,
    pub title: String,
    pub number: i64,
    pub content: String,
    pub remote_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadingProgress {
    pub book_id: i64,
    pub chapter_id: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookshelfGroup {
    pub id: i64,
    pub name: String,
    pub book_count: i64,
}
