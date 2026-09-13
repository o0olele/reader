//! Full-text search across one book — the reference app's 正文搜索 / 搜索内容
//! (`legado-with-MD3/app/src/main/java/io/legado/app/data/repository/SearchContentRepository.kt`).
//!
//! Only chapters whose body is already available offline are searched, exactly
//! as the reference does (`book.isLocal || cacheChapterNames.contains(...)`): a
//! full-text search never starts a download, so it stays instant and works
//! offline. Chapters are rendered through [`ReaderService::display_chapter`]
//! first, which means a hit addresses the text the reader actually shows.

mod matcher;
#[cfg(test)]
mod tests;

use super::*;
use crate::{
    domain::search_content::{SearchContentProgress, SearchContentResponse},
    repository::replace_rule::SqliteReplaceRuleRepository,
};
use matcher::{document_hits, QueryMatcher, SearchDocument, MAX_QUERY_CHARS};
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

/// Guard rail the reference does not have: one common word in a downloaded book
/// matches tens of thousands of times, and the whole list crosses IPC in a
/// single response. Hitting the cap is reported, never silently swallowed.
const MAX_HITS: usize = 2_000;

static SEARCH_STATE: OnceLock<Mutex<HashMap<i64, u64>>> = OnceLock::new();

fn generations() -> &'static Mutex<HashMap<i64, u64>> {
    SEARCH_STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Opens a new search generation for `book_id`. A scan in flight notices at its
/// next chapter and stops, which is also how [`cancel_search_content`] works.
fn start_search(book_id: i64) -> Result<u64, AppError> {
    let mut state = generations()
        .lock()
        .map_err(|_| AppError::Database("搜索状态锁不可用".into()))?;
    let generation = state.entry(book_id).or_default();
    *generation += 1;
    Ok(*generation)
}

fn is_current_search(book_id: i64, generation: u64) -> bool {
    generations()
        .lock()
        .map(|state| state.get(&book_id).copied() == Some(generation))
        .unwrap_or(false)
}

/// Stops the scan in flight for `book_id` — a newer query, or the user closing
/// the search panel.
pub(crate) fn cancel_search_content(book_id: i64) -> Result<(), AppError> {
    start_search(book_id).map(|_| ())
}

impl ReaderService {
    /// Searches every chapter of `book_id` that has an offline body.
    ///
    /// `on_progress` fires once per chapter so the IPC adapter can stream a
    /// progress bar; a cancelled scan returns the hits found so far with
    /// [`SearchContentResponse::cancelled`] set.
    pub async fn search_content(
        &self,
        book_id: i64,
        query: &str,
        regex: bool,
        mut on_progress: impl FnMut(SearchContentProgress),
    ) -> Result<SearchContentResponse, AppError> {
        let query = query.trim();
        if query.is_empty() {
            return Err(AppError::InvalidArgument("请输入搜索关键词".into()));
        }
        if query.chars().count() > MAX_QUERY_CHARS {
            return Err(AppError::InvalidArgument(format!(
                "搜索关键词不能超过 {MAX_QUERY_CHARS} 个字符"
            )));
        }
        let matcher = QueryMatcher::new(query, regex)?;
        let book = self
            .books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        let source = match book.source_id {
            Some(id) => self.sources.get(id).await?,
            None => None,
        };
        let rules = SqliteReplaceRuleRepository::new(self.pool.clone())
            .list()
            .await?;
        let searchable = self.chapters.list_searchable(book_id).await?;
        let total_chapters = self.chapters.count_for_book(book_id).await?;
        let searchable_total = searchable.len();
        let generation = start_search(book_id)?;
        tracing::info!(
            target: "reader", book_id, regex, catalog = total_chapters,
            searchable = searchable_total, "full-text search started"
        );

        let mut hits = Vec::new();
        let mut searched = 0usize;
        let mut cancelled = false;
        let mut truncated = false;
        for chapter in searchable {
            if !is_current_search(book_id, generation) {
                cancelled = true;
                break;
            }
            let displayed = self
                .display_chapter(chapter, &book, source.as_ref(), rules.clone())
                .await?;
            let document = SearchDocument::new(&displayed);
            hits.extend(document_hits(
                &document,
                &matcher,
                &displayed,
                total_chapters,
            ));
            searched += 1;
            on_progress(SearchContentProgress {
                scanned: searched,
                total: searchable_total,
                hits: hits.len(),
            });
            if hits.len() >= MAX_HITS {
                hits.truncate(MAX_HITS);
                truncated = true;
                break;
            }
        }
        tracing::info!(
            target: "reader", book_id, hits = hits.len(), searched,
            cancelled, truncated, "full-text search finished"
        );
        Ok(SearchContentResponse {
            hits,
            searched_chapters: searched as i64,
            total_chapters,
            truncated,
            cancelled,
        })
    }
}
