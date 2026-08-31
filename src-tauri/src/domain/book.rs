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
    pub intro: Option<String>,
    pub kind: Option<String>,
    pub latest_chapter: Option<String>,
    pub cover_url: Option<String>,
    pub cover_data: Option<String>,
    pub chapter_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookshelfGroup {
    pub id: i64,
    pub name: String,
    pub book_count: i64,
}
