ALTER TABLE book_sources ADD COLUMN source_group TEXT;
ALTER TABLE book_sources ADD COLUMN custom_order INTEGER NOT NULL DEFAULT 0;
ALTER TABLE book_sources ADD COLUMN weight INTEGER NOT NULL DEFAULT 0;
ALTER TABLE book_sources ADD COLUMN enabled_explore INTEGER NOT NULL DEFAULT 1 CHECK (enabled_explore IN (0, 1));
ALTER TABLE book_sources ADD COLUMN respond_time INTEGER;
ALTER TABLE book_sources ADD COLUMN last_update_time INTEGER;

CREATE INDEX idx_book_sources_management
  ON book_sources(custom_order, weight DESC, name);
