use crate::{
    error::AppError,
    infrastructure::ebook::export,
    repository::{
        book::SqliteBookRepository, chapter::SqliteChapterRepository, BookRepository,
        ChapterRepository,
    },
};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub path: String,
    pub bytes_written: usize,
}

#[derive(Clone)]
pub struct ExportService {
    books: SqliteBookRepository,
    chapters: SqliteChapterRepository,
}

impl ExportService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            books: SqliteBookRepository::new(pool.clone()),
            chapters: SqliteChapterRepository::new(pool),
        }
    }

    pub async fn export(
        &self,
        book_id: i64,
        format: &str,
        target: &Path,
    ) -> Result<ExportResult, AppError> {
        let book = self
            .books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        let chapters = self.chapters.list_for_book(book_id).await?;
        if chapters.is_empty() {
            return Err(AppError::InvalidArgument("书籍没有可导出的章节".into()));
        }
        if chapters
            .iter()
            .any(|chapter| chapter.content.trim().is_empty())
        {
            return Err(AppError::InvalidArgument(
                "仍有章节未缓存，请先完成整本下载".into(),
            ));
        }
        let bytes = match format.trim().to_ascii_lowercase().as_str() {
            "txt" => export::txt(&book.title, book.author.as_deref(), &chapters),
            "epub" => export::epub(&book.title, book.author.as_deref(), &chapters)?,
            _ => return Err(AppError::InvalidArgument("仅支持导出 TXT 或 EPUB".into())),
        };
        if let Some(parent) = target.parent().filter(|path| !path.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        std::fs::write(target, &bytes).map_err(AppError::io)?;
        Ok(ExportResult {
            path: target.to_string_lossy().into_owned(),
            bytes_written: bytes.len(),
        })
    }
}
