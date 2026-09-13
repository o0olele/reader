use serde::Serialize;

/// One saved position inside a chapter. The chapter id is unique, so a book
/// holds at most one bookmark per chapter (`018_bookmarks.sql`).
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Bookmark {
    pub book_id: i64,
    pub chapter_id: i64,
    pub offset: i64,
    pub mode: String,
}

/// A bookmark plus the book and chapter it points at, for the cross-book
/// 书签 page (`list_bookmarks`). Display fields only: the reader still reads a
/// bookmark through `get_bookmark`.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct BookmarkEntry {
    pub book_id: i64,
    pub book_title: String,
    pub book_author: Option<String>,
    pub chapter_id: i64,
    pub chapter_title: String,
    pub chapter_number: i64,
    pub offset: i64,
    pub mode: String,
    pub updated_at: String,
}
