use serde::Serialize;

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
