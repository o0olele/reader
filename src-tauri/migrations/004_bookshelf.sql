CREATE TABLE IF NOT EXISTS bookshelf_groups (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO bookshelf_groups (name, sort_order) VALUES ('默认书架', 0);
ALTER TABLE books ADD COLUMN group_id INTEGER REFERENCES bookshelf_groups(id);
UPDATE books SET group_id = (SELECT id FROM bookshelf_groups WHERE name = '默认书架') WHERE group_id IS NULL;
CREATE INDEX IF NOT EXISTS idx_books_group ON books(group_id);
