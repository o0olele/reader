ALTER TABLE book_sources ADD COLUMN book_url_pattern TEXT;
ALTER TABLE book_sources ADD COLUMN enabled_cookie_jar INTEGER NOT NULL DEFAULT 1 CHECK (enabled_cookie_jar IN (0, 1));
ALTER TABLE book_sources ADD COLUMN info_can_rename TEXT;

