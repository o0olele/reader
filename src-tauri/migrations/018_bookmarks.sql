CREATE TABLE bookmarks (
  chapter_id INTEGER PRIMARY KEY REFERENCES chapters(id) ON DELETE CASCADE,
  book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
  offset INTEGER NOT NULL CHECK (offset >= 0),
  mode TEXT NOT NULL CHECK (mode IN ('scroll', 'paged')),
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_bookmarks_book ON bookmarks(book_id);
