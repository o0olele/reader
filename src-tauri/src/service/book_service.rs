use crate::{
    domain::source::{BookSearchResult, BookSource, ChapterRef, SourceBookPreview},
    domain::Book,
    error::AppError,
    infrastructure::ebook::{epub, title_from_filename, txt, ParsedBook},
    infrastructure::http::request::response_error,
    repository::{book::SqliteBookRepository, chapter::SqliteChapterRepository, BookRepository},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::{settings_service::SettingsService, source_session::SourceSession},
    source_engine::{
        legado_rules::LegadoRules,
        pipeline::parse_book_info,
        url::{build as build_url_request, decode_text},
    },
};
use base64::{engine::general_purpose::STANDARD, Engine};

/// A source may rename the book on 详情 only when it declares `canRename`.
fn can_rename(source: &BookSource) -> bool {
    LegadoRules::decode(&source.raw_rules)
        .book_info
        .and_then(|rule| rule.can_rename)
        .as_deref()
        .or(source.info_rule.can_rename.as_deref())
        .is_some_and(|value| !value.trim().is_empty())
}

#[derive(Clone)]
pub struct BookService {
    books: SqliteBookRepository,
    chapters: SqliteChapterRepository,
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl BookService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            books: SqliteBookRepository::new(pool.clone()),
            chapters: SqliteChapterRepository::new(pool.clone()),
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    /// Opens one source-scoped session for this book's source.
    async fn session(&self, source: &BookSource) -> Result<SourceSession, AppError> {
        SourceSession::new(
            source.clone(),
            self.sources.clone(),
            15,
            self.settings.proxy_url().await?.as_deref(),
        )
    }

    pub async fn list(&self) -> Result<Vec<Book>, AppError> {
        self.books.list().await
    }

    pub async fn delete(&self, book_id: i64) -> Result<(), AppError> {
        self.books.delete(book_id).await
    }

    pub async fn add_online(&self, result: &BookSearchResult) -> Result<Book, AppError> {
        if let Some(book) = self.books.find_by_path(&result.url).await? {
            return Ok(book);
        }
        let id = self
            .books
            .create_online(
                &result.title,
                result.author.as_deref(),
                &result.url,
                result.source_id,
            )
            .await?;
        self.load(id).await
    }

    pub async fn fetch_info(&self, book_id: i64) -> Result<Book, AppError> {
        let book = self.load(book_id).await?;
        let source_id = book
            .source_id
            .ok_or_else(|| AppError::Source("本地书籍没有在线书源".into()))?;
        let url = book
            .remote_url
            .as_deref()
            .ok_or_else(|| AppError::Source("书籍没有远程地址".into()))?;
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let session = self.session(&source).await?;
        let request = build_url_request(&source, url, None, "详情 URL")?;
        let response = session.send(&request).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let html = decode_text(response, &request, &source).await?;
        let info = parse_book_info(&source, &html)?;
        let cover_url = info.cover.clone();
        self.books
            .update_info(book_id, &info, can_rename(&source))
            .await?;
        self.store_cover(book_id, &session, cover_url.as_deref())
            .await?;
        self.load(book_id).await
    }

    /// Reads what a candidate source has for this book without changing it —
    /// the 换源 sheet's 加载详情 / 加载目录 options, and the check that keeps a
    /// broken source from replacing a working catalog.
    pub async fn preview_source(
        &self,
        book_id: i64,
        result: &BookSearchResult,
        with_info: bool,
        with_toc: bool,
    ) -> Result<SourceBookPreview, AppError> {
        self.load(book_id).await?;
        let source = self
            .sources
            .get(result.source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let session = self.session(&source).await?;
        let mut preview = SourceBookPreview::default();
        if with_toc {
            let catalog = session.fetch_catalog(&result.url).await?;
            if catalog.is_empty() {
                return Err(AppError::Source("书源没有解析出目录".into()));
            }
            preview.latest_chapter = catalog.last().map(|(title, _)| title.clone());
            preview.chapters = catalog
                .into_iter()
                .map(|(title, url)| ChapterRef { title, url })
                .collect();
        }
        if with_info {
            // 详情 failing does not make the source unusable — only a missing
            // 目录 does — so it is reported beside the row instead of failing it.
            match session.fetch_book_info(&result.url).await {
                Ok(info) => {
                    if preview.latest_chapter.is_none() {
                        preview.latest_chapter = info.latest_chapter.clone();
                    }
                    preview.info = Some(info);
                }
                Err(error) => preview.info_error = Some(error.to_string()),
            }
        }
        Ok(preview)
    }

    /// 换源: re-points the shelf entry at another source.
    ///
    /// The candidate's catalog is fetched *before* anything is written, so a
    /// source that cannot produce a 目录 leaves the current one untouched — the
    /// reference app's `ChangeBookSource` does the same by calling `getToc`
    /// first. `chapters` is the previewed catalog when the sheet already loaded
    /// it, which saves a second walk over the source's 目录 pages.
    pub async fn switch_source(
        &self,
        book_id: i64,
        result: &BookSearchResult,
        chapters: Option<Vec<ChapterRef>>,
    ) -> Result<Book, AppError> {
        let current = self.load(book_id).await?;
        if current.source_id == Some(result.source_id)
            && current.remote_url.as_deref() == Some(result.url.as_str())
        {
            return Ok(current);
        }
        let source = self
            .sources
            .get(result.source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let session = self.session(&source).await?;
        let catalog = match chapters {
            Some(chapters) => chapters
                .into_iter()
                .map(|chapter| (chapter.title, chapter.url))
                .collect::<Vec<_>>(),
            None => session.fetch_catalog(&result.url).await?,
        };
        if catalog.is_empty() {
            return Err(AppError::Source("书源没有解析出目录".into()));
        }
        // 详情 is a nice-to-have: a source whose catalog parses but whose info
        // rule is broken still reads fine, so its failure must not block 换源.
        let info = session.fetch_book_info(&result.url).await.ok();
        let cover_url = info.as_ref().and_then(|info| info.cover.clone());
        self.books
            .switch_source(book_id, result.source_id, &result.url)
            .await?;
        if let Some(info) = info.as_ref() {
            self.books
                .update_info(book_id, info, can_rename(&source))
                .await?;
        }
        self.store_cover(book_id, &session, cover_url.as_deref())
            .await?;
        self.chapters.replace_catalog(book_id, &catalog).await?;
        tracing::info!(target: "book", book_id, source = %source.name, chapter_count = catalog.len(), "book source switched");
        self.load(book_id).await
    }

    /// Downloads the parsed cover into `cover_data`; a source that will not
    /// serve its own image simply leaves the placeholder in place.
    async fn store_cover(
        &self,
        book_id: i64,
        session: &SourceSession,
        cover_url: Option<&str>,
    ) -> Result<(), AppError> {
        let Some(cover) = cover_url else {
            return Ok(());
        };
        let Some(fetched) = session.fetch_cover(cover).await else {
            return Ok(());
        };
        let mime = fetched.content_type.as_deref().unwrap_or("image/jpeg");
        self.books
            .save_cover_data(
                book_id,
                &format!("data:{mime};base64,{}", STANDARD.encode(fetched.bytes)),
            )
            .await
    }

    pub async fn import_txt(&self, filename: &str, bytes: &[u8]) -> Result<Book, AppError> {
        let title = title_from_filename(filename);
        if title.is_empty() || bytes.is_empty() {
            return Err(AppError::InvalidArgument("书籍名称或内容不能为空".into()));
        }
        self.import(filename, txt::parse(bytes, title)?).await
    }

    pub async fn import_epub(&self, filename: &str, bytes: Vec<u8>) -> Result<Book, AppError> {
        let fallback_title = title_from_filename(filename);
        self.import(filename, epub::parse(bytes, fallback_title)?)
            .await
    }

    /// Persists a parsed local book, returning the existing shelf entry when the
    /// same file has already been imported.
    async fn import(&self, filename: &str, parsed: ParsedBook) -> Result<Book, AppError> {
        if let Some(existing) = self.books.find_by_path(filename).await? {
            tracing::info!(target: "book", filename, book_id = existing.id, "import skipped, book already on shelf");
            return Ok(existing);
        }
        let chapter_count = parsed.chapters.len();
        tracing::info!(target: "book", filename, chapter_count, "starting local import");
        let id = self
            .books
            .create_local_with_chapters(
                &parsed.title,
                parsed.author.as_deref(),
                filename,
                &parsed.chapters,
            )
            .await?;
        tracing::info!(target: "book", filename, book_id = id, chapter_count, "local import completed");
        self.load(id).await
    }

    async fn load(&self, id: i64) -> Result<Book, AppError> {
        self.books
            .get(id)
            .await?
            .ok_or_else(|| AppError::Database(format!("书籍 {id} 写入后无法读取")))
    }
}
