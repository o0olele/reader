use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReadingRecord {
    pub book_id: i64,
    pub duration_seconds: i64,
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
