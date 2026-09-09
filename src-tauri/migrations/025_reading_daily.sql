-- Per-day reading time, so the home dashboard can show today's minutes and a
-- streak without scanning cumulative per-book totals (ROADMAP-v3 E1).
CREATE TABLE IF NOT EXISTS reading_daily (
  day TEXT PRIMARY KEY,
  seconds INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
