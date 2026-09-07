CREATE TABLE IF NOT EXISTS download_tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
  status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'running', 'paused', 'completed', 'failed', 'cancelled')),
  chapter_ids TEXT,
  total_chapters INTEGER NOT NULL DEFAULT 0,
  completed_chapters INTEGER NOT NULL DEFAULT 0,
  retry_count INTEGER NOT NULL DEFAULT 0,
  error TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_download_tasks_status ON download_tasks(status, updated_at);
CREATE INDEX IF NOT EXISTS idx_download_tasks_book ON download_tasks(book_id, created_at DESC);
