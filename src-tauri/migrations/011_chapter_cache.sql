-- Keep downloaded online chapter bodies separate from the catalog metadata.
-- This allows catalog rows to exist without forcing an empty NOT NULL body.
CREATE TABLE IF NOT EXISTS chapter_contents (
  chapter_id INTEGER PRIMARY KEY REFERENCES chapters(id) ON DELETE CASCADE,
  content TEXT NOT NULL,
  cached_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_chapter_contents_cached_at
  ON chapter_contents(cached_at);
