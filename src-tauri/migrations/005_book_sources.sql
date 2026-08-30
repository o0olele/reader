CREATE TABLE IF NOT EXISTS book_sources (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  base_url TEXT NOT NULL,
  search_url TEXT NOT NULL,
  search_item_selector TEXT NOT NULL,
  title_selector TEXT NOT NULL,
  author_selector TEXT,
  cover_selector TEXT,
  url_selector TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_book_sources_name ON book_sources(name);
