use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DownloadTask {
    pub id: i64,
    pub book_id: i64,
    pub book_title: String,
    pub status: String,
    pub total_chapters: i64,
    pub completed_chapters: i64,
    pub retry_count: i64,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
