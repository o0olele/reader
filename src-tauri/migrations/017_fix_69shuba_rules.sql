-- 69 书吧 changed its search rule export from plain CSS selectors to
-- Legado's indexed `@text`/`@href` spellings. The old importer stored those
-- spellings literally, which made every result fail extraction even when the
-- browser request returned HTTP 200.
UPDATE book_sources
SET title_selector = 'h3',
    author_selector = 'label:nth-of-type(1)',
    cover_selector = 'img::attr(data-src)',
    url_selector = 'h3 a::attr(href)',
    updated_at = CURRENT_TIMESTAMP
WHERE lower(base_url) LIKE 'https://www.69shuba.com%';
