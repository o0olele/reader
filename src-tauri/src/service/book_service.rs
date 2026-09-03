use crate::{
    domain::source::BookSearchResult,
    domain::Book,
    error::AppError,
    infrastructure::ebook::{epub, title_from_filename, txt, ParsedBook},
    infrastructure::http::{client::build_source_client, request::response_error},
    repository::{book::SqliteBookRepository, BookRepository},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::pipeline::parse_book_info,
    source_engine::url::{build as build_url_request, decode_text, fetch_bytes, send},
};
use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Clone)]
pub struct BookService {
    books: SqliteBookRepository,
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl BookService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            books: SqliteBookRepository::new(pool.clone()),
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
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
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let request = build_url_request(&source, url, None, "详情 URL")?;
        let response = send(&client, &source, &request).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let html = decode_text(response, &request, &source).await?;
        let info = parse_book_info(&source, &html)?;
        let cover_url = info.cover.clone();
        self.books.update_info(book_id, &info).await?;
        if let Some(cover) = cover_url.as_deref() {
            if let Ok(request) = build_url_request(&source, cover, None, "封面 URL") {
                if let Ok(fetched) = fetch_bytes(&client, &source, &request).await {
                    let mime = fetched.content_type.as_deref().unwrap_or("image/jpeg");
                    self.books
                        .save_cover_data(
                            book_id,
                            &format!("data:{mime};base64,{}", STANDARD.encode(fetched.bytes)),
                        )
                        .await?;
                }
            }
        }
        self.load(book_id).await
    }

    pub async fn switch_source(
        &self,
        book_id: i64,
        result: &BookSearchResult,
    ) -> Result<Book, AppError> {
        let current = self.load(book_id).await?;
        if current.source_id == Some(result.source_id) {
            return Ok(current);
        }
        self.books
            .switch_source(book_id, result.source_id, &result.url)
            .await?;
        self.load(book_id).await
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
