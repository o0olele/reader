//! Candidate inspection and source switching for an existing shelf entry.
use super::{can_rename, BookService};
use crate::{
    domain::{
        source::{BookSearchResult, ChapterRef, SourceBookPreview},
        Book,
    },
    error::AppError,
    repository::SourceRepository,
};

impl BookService {
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
}
