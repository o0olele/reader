ALTER TABLE book_sources ADD COLUMN login_url TEXT;
ALTER TABLE book_sources ADD COLUMN login_method TEXT NOT NULL DEFAULT 'POST';
ALTER TABLE book_sources ADD COLUMN login_body TEXT;
ALTER TABLE book_sources ADD COLUMN token_path TEXT;
ALTER TABLE book_sources ADD COLUMN access_token TEXT;
ALTER TABLE book_sources ADD COLUMN session_cookie TEXT;
ALTER TABLE book_sources ADD COLUMN session_expires_at TEXT;
ALTER TABLE book_sources ADD COLUMN sign_script TEXT;
