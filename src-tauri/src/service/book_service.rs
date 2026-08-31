use crate::{
    domain::source::BookSearchResult,
    domain::Book,
    error::AppError,
    infrastructure::ebook::{epub, title_from_filename, txt, ParsedBook},
    repository::{book::SqliteBookRepository, BookRepository},
};

#[derive(Clone)]
pub struct BookService {
    books: SqliteBookRepository,
}

impl BookService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            books: SqliteBookRepository::new(pool),
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
