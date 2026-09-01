-- Preserve legado rule objects verbatim while the legacy CSS projection remains
-- available to the current selector pipeline.
ALTER TABLE book_sources ADD COLUMN rule_search TEXT;
ALTER TABLE book_sources ADD COLUMN rule_book_info TEXT;
ALTER TABLE book_sources ADD COLUMN rule_toc TEXT;
ALTER TABLE book_sources ADD COLUMN rule_content TEXT;
