use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReadingRecord {
    pub book_id: i64,
    pub duration_seconds: i64,
}

/// One row of the 历史 page: a book the user has read, where they stopped and
/// how long they spent on it (`list_reading_history`). A row exists as soon as
/// the book has a saved position **or** recorded reading time, so a book read
/// for less than one timer tick is still listed instead of silently missing.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReadingHistoryEntry {
    pub book_id: i64,
    pub book_title: String,
    pub book_author: Option<String>,
    pub cover_data: Option<String>,
    /// `None` once the saved chapter left the catalog (目录收缩 / 换源): the book
    /// stays in the history, but it no longer has a position to show. The three
    /// chapter fields are always set or unset together.
    pub chapter_id: Option<i64>,
    pub chapter_title: Option<String>,
    pub chapter_number: Option<i64>,
    /// Whether a position row exists at all. It separates "read but never saved a
    /// position" (`false`) from "the saved position is gone" (`true` with
    /// `chapter_id: None`), which would otherwise look identical.
    pub has_progress: bool,
    /// Catalog size, for the progress percentage; `0` before a catalog exists.
    pub chapter_count: i64,
    pub duration_seconds: i64,
    /// Newest of the position and reading-time timestamps, UTC as SQLite writes
    /// it — the 历史 page orders on this and renders it in local time.
    pub last_read_at: String,
}

/// Aggregated reading statistics for the home dashboard (ROADMAP-v3 E1).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReadingStats {
    pub total_seconds: i64,
    pub today_seconds: i64,
    pub daily_goal_minutes: i64,
    /// Consecutive days with reading time, ending today.
    pub streak_days: i64,
    /// Books whose saved progress sits on the last chapter of their catalog.
    pub finished_books: i64,
}
