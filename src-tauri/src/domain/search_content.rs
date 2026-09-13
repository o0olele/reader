//! One occurrence of a full-text query, in the shape the reader can address it.
//!
//! The reference app carries an offset into a `"title\nbody"` document and
//! re-resolves it on the reader side (`ReaderSearchMatcher`). Here the backend
//! already knows which display paragraph the match belongs to, so the frontend
//! only has to scroll to `paragraph_index` and mark the range — no offset maths
//! across a joined string, and no ambiguity between the title and the body.

use serde::Serialize;

/// A single hit. Offsets are UTF-16 code units from the start of the title (when
/// [`Self::in_title`]) or of paragraph [`Self::paragraph_index`], which is what
/// `String.prototype.slice` and the DOM both use — the same reason the reference
/// app keeps its offsets char-based instead of byte-based.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SearchContentHit {
    pub chapter_id: i64,
    /// Catalog index, used for the "当前位置百分比" chip.
    pub chapter_number: i64,
    pub chapter_title: String,
    /// 0-based occurrence index inside this chapter (reference `resultCountWithinChapter`).
    pub result_index: i64,
    /// Whether the match sits in the chapter title instead of the body.
    pub in_title: bool,
    /// 0-based index into the reader's paragraph list; `None` for a title hit.
    pub paragraph_index: Option<i64>,
    pub match_offset: i64,
    pub match_length: i64,
    /// The matched text as produced by the matcher — a regex hit cannot be
    /// recomputed on the frontend from the query alone.
    pub match_text: String,
    /// Context around the match, taken from the same paragraph.
    pub snippet: String,
    pub snippet_offset: i64,
    pub snippet_length: i64,
    /// Chapter position in the book, mirroring the reference's `progressPercent`.
    pub progress_percent: f64,
}

/// Progress of a scan in flight, streamed while the search runs so the panel can
/// show how far the book has been walked and how many hits are already found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SearchContentProgress {
    /// Chapters scanned so far.
    pub scanned: usize,
    /// Chapters with an offline body, i.e. what this search will walk.
    pub total: usize,
    /// Hits found so far.
    pub hits: usize,
}

/// The whole answer, including how much of the book was actually searched.
/// Chapters without an offline body are skipped (the reference searches local
/// books and already-downloaded chapters only), so the two counts differ and the
/// UI can say so instead of implying the book was fully covered.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SearchContentResponse {
    pub hits: Vec<SearchContentHit>,
    /// Chapters whose body was scanned.
    pub searched_chapters: i64,
    /// Chapters in the catalog, cached or not.
    pub total_chapters: i64,
    /// The hit cap was reached before the last chapter.
    pub truncated: bool,
    /// A newer search (or an explicit stop) replaced this one mid-scan.
    pub cancelled: bool,
}
