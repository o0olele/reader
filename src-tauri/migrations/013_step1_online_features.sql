ALTER TABLE books ADD COLUMN intro TEXT;
ALTER TABLE books ADD COLUMN kind TEXT;
ALTER TABLE books ADD COLUMN latest_chapter TEXT;
ALTER TABLE books ADD COLUMN cover_url TEXT;
ALTER TABLE books ADD COLUMN cover_data TEXT;

ALTER TABLE book_sources ADD COLUMN next_toc_url_selector TEXT;
ALTER TABLE book_sources ADD COLUMN next_content_url_selector TEXT;
ALTER TABLE book_sources ADD COLUMN info_cover_selector TEXT;
