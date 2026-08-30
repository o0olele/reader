ALTER TABLE books ADD COLUMN source_id INTEGER REFERENCES book_sources(id);
ALTER TABLE books ADD COLUMN remote_url TEXT;
ALTER TABLE chapters ADD COLUMN remote_url TEXT;

ALTER TABLE book_sources ADD COLUMN info_title_selector TEXT;
ALTER TABLE book_sources ADD COLUMN info_author_selector TEXT;
ALTER TABLE book_sources ADD COLUMN info_intro_selector TEXT;
ALTER TABLE book_sources ADD COLUMN catalog_item_selector TEXT;
ALTER TABLE book_sources ADD COLUMN catalog_title_selector TEXT;
ALTER TABLE book_sources ADD COLUMN catalog_url_selector TEXT;
ALTER TABLE book_sources ADD COLUMN content_selector TEXT;
