use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReadingRecord {
    pub book_id: i64,
    pub duration_seconds: i64,
}
