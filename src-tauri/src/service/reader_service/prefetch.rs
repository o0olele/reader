//! Reading-time prefetch, ported from the reference app's
//! `ReadBook.preDownload()` (`legado-with-MD3/app/src/main/java/io/legado/app/model/ReadBook.kt:1745`).
//!
//! While one chapter is open the neighbouring chapters are pulled into the正文
//! cache in the background, so turning a page is a cache hit instead of a
//! network round trip. The window is deliberately asymmetric, exactly like the
//! reference: forward `reader_prefetch_num` chapters (default 10) and backward
//! at most 5, so re-reading backwards stays warm without re-fetching the book.

use super::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};
use tokio::sync::Semaphore;

/// The reference runs two lanes under `preDownloadSemaphore = Semaphore(2)`.
const PREFETCH_CONCURRENCY: usize = 2;
/// A chapter that failed this many times is left alone (reference
/// `downloadFailChapters[i] >= 3`), so one dead URL cannot pin the window.
const FAILURE_LIMIT: u32 = 3;
/// The reference looks back `min(5, preDownloadNum)` chapters.
const BACKWARD_LIMIT: usize = 5;

static PREFETCH_STATE: OnceLock<Mutex<PrefetchState>> = OnceLock::new();
static PREFETCH_SLOTS: OnceLock<Arc<Semaphore>> = OnceLock::new();

#[derive(Default)]
struct PrefetchState {
    /// Book id → run generation. Opening another chapter increments it, which is
    /// also how a window in flight is cancelled.
    generations: HashMap<i64, u64>,
    /// Chapter id → failures since it last succeeded.
    failures: HashMap<i64, u32>,
}

/// What one prefetch run did. `planned` is the window the rules selected.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PrefetchSummary {
    pub planned: usize,
    pub downloaded: usize,
    pub failed: usize,
}

/// The two request lanes, kept apart so `tokio::join!` reproduces the
/// reference's shape: forwards and backwards advance concurrently.
#[derive(Debug, Default)]
pub(super) struct PrefetchPlan {
    pub forward: Vec<Chapter>,
    pub backward: Vec<Chapter>,
}

/// A chapter is worth downloading when it has no body yet, has an address to
/// fetch it from, and has not already failed too often.
fn eligible(chapter: &Chapter, failures: &HashMap<i64, u32>) -> bool {
    chapter.content.trim().is_empty()
        && chapter
            .remote_url
            .as_deref()
            .is_some_and(|url| !url.trim().is_empty())
        && failures.get(&chapter.id).copied().unwrap_or(0) < FAILURE_LIMIT
}

/// Forward window first, then the backward one, nearest chapter first in each.
/// Pure, so the window rules are testable without a database or a network.
pub(super) fn prefetch_plan(
    chapters: &[Chapter],
    current: usize,
    ahead: usize,
    failures: &HashMap<i64, u32>,
) -> PrefetchPlan {
    PrefetchPlan {
        forward: chapters
            .get(current.saturating_add(1)..)
            .unwrap_or_default()
            .iter()
            .take(ahead)
            .filter(|chapter| eligible(chapter, failures))
            .cloned()
            .collect(),
        backward: chapters
            .get(..current)
            .unwrap_or_default()
            .iter()
            .rev()
            .take(BACKWARD_LIMIT)
            .filter(|chapter| eligible(chapter, failures))
            .cloned()
            .collect(),
    }
}

impl ReaderService {
    /// Warms the chapters around `chapter_id`. Fire-and-forget by nature: the
    /// caller runs it in the background and a failure here never breaks reading.
    pub async fn prefetch_around(
        &self,
        book_id: i64,
        chapter_id: i64,
    ) -> Result<PrefetchSummary, AppError> {
        let book = self
            .books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        // Local books already hold their whole text; nothing to download.
        let Some(source_id) = book.source_id else {
            return Ok(PrefetchSummary::default());
        };
        let ahead = self.settings.reader_prefetch_num().await?.max(0) as usize;
        let chapters = self.list_chapters(book_id).await?;
        let Some(current) = chapters
            .iter()
            .position(|chapter| chapter.id == chapter_id)
        else {
            return Ok(PrefetchSummary::default());
        };
        let generation = start_generation(book_id)?;
        let PrefetchPlan { forward, backward } =
            prefetch_plan(&chapters, current, ahead, &failed_chapters()?);
        let planned = forward.len() + backward.len();
        if planned == 0 {
            return Ok(PrefetchSummary::default());
        }
        let (forward, backward) = tokio::join!(
            self.fetch_lane(source_id, forward, book_id, generation),
            self.fetch_lane(source_id, backward, book_id, generation),
        );
        let (forward, backward) = (forward?, backward?);
        let summary = PrefetchSummary {
            planned,
            downloaded: forward.downloaded + backward.downloaded,
            failed: forward.failed + backward.failed,
        };
        tracing::debug!(
            target: "reader", book_id, chapter_id, planned = summary.planned,
            downloaded = summary.downloaded, failed = summary.failed,
            "chapter prefetch finished"
        );
        Ok(summary)
    }

    /// One lane. It stops the moment another chapter opens (`generation` moved
    /// on), so a fast reader never queues behind a stale window.
    async fn fetch_lane(
        &self,
        source_id: i64,
        chapters: Vec<Chapter>,
        book_id: i64,
        generation: u64,
    ) -> Result<PrefetchSummary, AppError> {
        let slots = prefetch_slots();
        let mut summary = PrefetchSummary::default();
        for chapter in chapters {
            if !is_current_generation(book_id, generation) {
                tracing::debug!(target: "reader", book_id, "chapter prefetch cancelled");
                break;
            }
            let Some(url) = chapter.remote_url.clone() else {
                continue;
            };
            let _slot = slots
                .acquire()
                .await
                .map_err(|_| AppError::Database("预下载调度器不可用".into()))?;
            match self.fetch_online_content(source_id, &url, Some(chapter.id)).await {
                Ok(_) => {
                    clear_failure(chapter.id);
                    summary.downloaded += 1;
                }
                Err(error) => {
                    note_failure(chapter.id);
                    summary.failed += 1;
                    tracing::warn!(
                        target: "reader", chapter_id = chapter.id, chapter = %chapter.title,
                        %error, "chapter prefetch failed"
                    );
                }
            }
        }
        Ok(summary)
    }
}

fn prefetch_slots() -> Arc<Semaphore> {
    PREFETCH_SLOTS
        .get_or_init(|| Arc::new(Semaphore::new(PREFETCH_CONCURRENCY)))
        .clone()
}

fn prefetch_state() -> &'static Mutex<PrefetchState> {
    PREFETCH_STATE.get_or_init(|| Mutex::new(PrefetchState::default()))
}

fn lock_state() -> Result<std::sync::MutexGuard<'static, PrefetchState>, AppError> {
    prefetch_state()
        .lock()
        .map_err(|_| AppError::Database("预下载状态锁不可用".into()))
}

/// Opens a new window for `book_id` and returns its generation. Bumping the
/// generation is also how {@link cancel_prefetch} invalidates a run.
fn start_generation(book_id: i64) -> Result<u64, AppError> {
    let mut state = lock_state()?;
    let generation = state.generations.entry(book_id).or_default();
    *generation += 1;
    Ok(*generation)
}

fn is_current_generation(book_id: i64, generation: u64) -> bool {
    lock_state()
        .map(|state| state.generations.get(&book_id).copied() == Some(generation))
        .unwrap_or(false)
}

fn failed_chapters() -> Result<HashMap<i64, u32>, AppError> {
    Ok(lock_state()?.failures.clone())
}

fn note_failure(chapter_id: i64) {
    if let Ok(mut state) = lock_state() {
        *state.failures.entry(chapter_id).or_default() += 1;
    }
}

fn clear_failure(chapter_id: i64) {
    if let Ok(mut state) = lock_state() {
        state.failures.remove(&chapter_id);
    }
}

/// Stops the window in flight for `book_id` — the reader closed the book.
pub(crate) fn cancel_prefetch(book_id: i64) -> Result<(), AppError> {
    start_generation(book_id).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::settings_service::DEFAULT_PREFETCH_CHAPTERS;
    use sqlx::sqlite::SqlitePoolOptions;

    fn chapter(id: i64, content: &str) -> Chapter {
        Chapter {
            id,
            book_id: 1,
            title: format!("第{id}章"),
            number: id,
            content: content.into(),
            remote_url: Some(format!("https://example.test/{id}")),
        }
    }

    fn ids(chapters: &[Chapter]) -> Vec<i64> {
        chapters.iter().map(|chapter| chapter.id).collect()
    }

    fn catalog(size: i64) -> Vec<Chapter> {
        (1..=size).map(|id| chapter(id, "")).collect()
    }

    #[test]
    fn window_looks_ahead_and_back_from_the_current_chapter() {
        let plan = prefetch_plan(&catalog(20), 3, 3, &HashMap::new());
        assert_eq!(ids(&plan.forward), vec![5, 6, 7]);
        assert_eq!(ids(&plan.backward), vec![3, 2, 1]);
    }

    #[test]
    fn window_clamps_at_both_ends_of_the_catalog() {
        let first = prefetch_plan(&catalog(20), 0, 10, &HashMap::new());
        assert_eq!(ids(&first.forward), (2..=11).collect::<Vec<_>>());
        assert!(first.backward.is_empty());

        let last = prefetch_plan(&catalog(20), 19, 10, &HashMap::new());
        assert!(last.forward.is_empty());
        assert_eq!(ids(&last.backward), vec![19, 18, 17, 16, 15]);
    }

    #[test]
    fn backward_window_stops_at_five_chapters() {
        let plan = prefetch_plan(&catalog(30), 20, 10, &HashMap::new());
        assert_eq!(ids(&plan.backward), vec![20, 19, 18, 17, 16]);
    }

    #[test]
    fn window_skips_cached_and_addressless_chapters() {
        let mut chapters = catalog(9);
        chapters[4].content = "本地或已缓存的正文".into();
        chapters[5].remote_url = None;
        let plan = prefetch_plan(&chapters, 3, 5, &HashMap::new());
        // 5 已缓存、6 没有地址，都被跳过。
        assert_eq!(ids(&plan.forward), vec![7, 8, 9]);
        assert_eq!(ids(&plan.backward), vec![3, 2, 1]);
    }

    #[test]
    fn failure_limit_stops_retrying_a_dead_chapter() {
        let mut failures = HashMap::new();
        failures.insert(5_i64, 2_u32);
        failures.insert(6_i64, FAILURE_LIMIT);
        let plan = prefetch_plan(&catalog(9), 3, 5, &failures);
        assert_eq!(ids(&plan.forward), vec![5, 7, 8, 9]);
    }

    #[test]
    fn disabled_prefetch_num_plans_nothing_ahead() {
        let plan = prefetch_plan(&catalog(20), 9, 0, &HashMap::new());
        assert!(plan.forward.is_empty());
        assert_eq!(ids(&plan.backward), vec![9, 8, 7, 6, 5]);
    }

    #[test]
    fn cancelling_a_book_invalidates_the_running_window() {
        let generation = start_generation(42_001).unwrap();
        assert!(is_current_generation(42_001, generation));
        cancel_prefetch(42_001).unwrap();
        assert!(!is_current_generation(42_001, generation));
    }

    async fn pool() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    /// `books.source_id` is a foreign key, so an online book needs a real source.
    async fn online_source(pool: &sqlx::SqlitePool) {
        sqlx::query("INSERT INTO book_sources (id, name, base_url, search_url, search_item_selector, title_selector, url_selector) VALUES (7, 'Test', 'https://example.test', 'https://example.test/search?q={{key}}', '.item', '.title', 'a')")
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn local_books_have_nothing_to_prefetch() {
        let pool = pool().await;
        sqlx::query("INSERT INTO books (title, path) VALUES ('Local', 'local')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'One', '', 'https://example.test/1')")
            .execute(&pool)
            .await
            .unwrap();
        let service = ReaderService::new(pool);

        let summary = service.prefetch_around(1, 1).await.unwrap();

        assert_eq!(summary, PrefetchSummary::default());
    }

    #[tokio::test]
    async fn online_books_with_a_cached_catalog_plan_nothing() {
        let pool = pool().await;
        online_source(&pool).await;
        sqlx::query("INSERT INTO books (title, path, source_id) VALUES ('Online', 'https://example.test/book', 7)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'One', 'cached', 'https://example.test/1'), (1, 1, 'Two', 'cached', 'https://example.test/2'), (1, 2, 'Three', 'cached', 'https://example.test/3')")
            .execute(&pool)
            .await
            .unwrap();
        let service = ReaderService::new(pool);

        let summary = service.prefetch_around(1, 2).await.unwrap();

        assert_eq!(summary, PrefetchSummary::default());
    }

    #[tokio::test]
    async fn a_disabled_prefetch_num_plans_nothing() {
        let pool = pool().await;
        online_source(&pool).await;
        sqlx::query("INSERT INTO books (title, path, source_id) VALUES ('Online', 'https://example.test/book', 7)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chapters (book_id, number, title, content, remote_url) VALUES (1, 0, 'One', 'body', 'https://example.test/1'), (1, 1, 'Two', '', 'https://example.test/2')")
            .execute(&pool)
            .await
            .unwrap();
        let service = ReaderService::new(pool.clone());
        service.settings.save_reader_prefetch_num(0).await.unwrap();

        let summary = service.prefetch_around(1, 1).await.unwrap();

        assert_eq!(summary.planned, 0);
        let cached: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chapter_contents")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(cached, 0);
    }

    #[tokio::test]
    async fn prefetch_num_defaults_to_the_reference_value_and_is_validated() {
        let service = ReaderService::new(pool().await);
        assert_eq!(
            service.settings.reader_prefetch_num().await.unwrap(),
            DEFAULT_PREFETCH_CHAPTERS
        );
        assert!(service.settings.save_reader_prefetch_num(-1).await.is_err());
        assert!(service.settings.save_reader_prefetch_num(999).await.is_err());
        service.settings.save_reader_prefetch_num(3).await.unwrap();
        assert_eq!(service.settings.reader_prefetch_num().await.unwrap(), 3);
    }
}
