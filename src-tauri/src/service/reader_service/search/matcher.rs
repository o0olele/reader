//! The pure half of full-text search: compile a query, cut a chapter into the
//! reader's display units, and address every match as (paragraph, offset).
//!
//! Split from `search.rs` to keep both files under the project's 250-line
//! non-test limit (ROADMAP-v3 §6.3 #1).

use crate::{domain::search_content::SearchContentHit, domain::Chapter, error::AppError};

/// Context kept either side of a match, in characters (the reference's ±12).
const SNIPPET_PADDING: usize = 12;
/// Long enough for a sentence, short enough that the regex engine is never
/// handed a novel as a pattern.
pub(super) const MAX_QUERY_CHARS: usize = 120;

/// Compiled once per search and reused for every chapter.
pub(super) struct QueryMatcher {
    regex: regex::Regex,
}

impl QueryMatcher {
    pub(super) fn new(query: &str, regex: bool) -> Result<Self, AppError> {
        // A plain query is escaped, so a `.` or `*` from a book title stays
        // literal text; it is folded too, which is what the reference's own
        // highlighting does (`getHighlightRanges` uses `ignoreCase`).
        let pattern = if regex {
            query.to_owned()
        } else {
            regex::escape(query)
        };
        let regex = regex::RegexBuilder::new(&pattern)
            .case_insensitive(!regex)
            .build()
            .map_err(|error| AppError::InvalidArgument(format!("搜索表达式无效：{error}")))?;
        Ok(Self { regex })
    }

    /// Byte ranges of every non-empty match, left to right and non-overlapping
    /// (`find_iter`, like the reference's `indexOf` loop and `Regex.findAll`).
    pub(super) fn find(&self, text: &str) -> Vec<(usize, usize)> {
        self.regex
            .find_iter(text)
            .filter(|found| found.end() > found.start())
            .map(|found| (found.start(), found.end() - found.start()))
            .collect()
    }
}

/// One chapter in the shape the reader displays it: the title line, then the
/// paragraphs `splitParagraphs` yields on the frontend. Addressing a hit as
/// (paragraph, offset) is what makes the jump exact — a raw offset into a
/// joined `"title\nbody"` document has to be re-resolved on the other side.
pub(super) struct SearchDocument {
    title: String,
    paragraphs: Vec<String>,
}

impl SearchDocument {
    pub(super) fn new(chapter: &Chapter) -> Self {
        Self {
            title: chapter.title.clone(),
            paragraphs: chapter
                .content
                .split('\n')
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect(),
        }
    }
}

/// Offsets travel as UTF-16 code units: JavaScript slices strings by code unit,
/// so a Rust byte offset would mark the wrong span as soon as the chapter holds
/// anything outside Latin-1.
fn utf16_len(text: &str) -> i64 {
    text.encode_utf16().count() as i64
}

/// The context window around one match, plus the match position inside it.
fn snippet(text: &str, start: usize, length: usize) -> (String, i64, i64) {
    let before = text[..start]
        .chars()
        .rev()
        .take(SNIPPET_PADDING)
        .collect::<Vec<_>>();
    let from = start
        - before
            .iter()
            .map(|character| character.len_utf8())
            .sum::<usize>();
    let after = text[start + length..]
        .chars()
        .take(SNIPPET_PADDING)
        .collect::<String>();
    let prefix = &text[from..start];
    let matched = &text[start..start + length];
    (
        format!("{prefix}{matched}{after}"),
        utf16_len(prefix),
        utf16_len(matched),
    )
}

/// Every hit of one chapter, in reading order: the title line first, then each
/// paragraph. Pure, so the addressing rules are testable without a database.
pub(super) fn document_hits(
    document: &SearchDocument,
    matcher: &QueryMatcher,
    chapter: &Chapter,
    total_chapters: i64,
) -> Vec<SearchContentHit> {
    let progress_percent = if total_chapters > 0 {
        (chapter.number + 1) as f64 / total_chapters as f64 * 100.0
    } else {
        0.0
    };
    let title = std::iter::once((true, None, document.title.as_str()));
    let paragraphs = document
        .paragraphs
        .iter()
        .enumerate()
        .map(|(index, text)| (false, Some(index as i64), text.as_str()));

    let mut hits: Vec<SearchContentHit> = Vec::new();
    for (in_title, paragraph_index, text) in title.chain(paragraphs) {
        for (start, length) in matcher.find(text) {
            let (snippet, snippet_offset, snippet_length) = snippet(text, start, length);
            hits.push(SearchContentHit {
                chapter_id: chapter.id,
                chapter_number: chapter.number,
                chapter_title: document.title.clone(),
                result_index: hits.len() as i64,
                in_title,
                paragraph_index,
                match_offset: utf16_len(&text[..start]),
                match_length: utf16_len(&text[start..start + length]),
                match_text: text[start..start + length].to_owned(),
                snippet,
                snippet_offset,
                snippet_length,
                progress_percent,
            });
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chapter(title: &str, content: &str) -> Chapter {
        Chapter {
            id: 3,
            book_id: 1,
            title: title.into(),
            number: 1,
            content: content.into(),
            remote_url: None,
        }
    }

    #[test]
    fn plain_queries_are_literal_and_case_folded() {
        let matcher = QueryMatcher::new("a.c", false).unwrap();
        assert_eq!(matcher.find("xA.Cy a.c"), vec![(1, 3), (6, 3)]);
        assert!(matcher.find("abc").is_empty());
    }

    #[test]
    fn regex_queries_are_used_as_written() {
        let matcher = QueryMatcher::new(r"\d{2}", true).unwrap();
        assert_eq!(matcher.find("第12章"), vec![(3, 2)]);
        // 正则模式下不折叠大小写：是否忽略大小写由表达式自己决定。
        assert!(QueryMatcher::new("abc", true)
            .unwrap()
            .find("ABC")
            .is_empty());
        assert!(QueryMatcher::new("(", true).is_err());
        assert!(QueryMatcher::new("a*", true)
            .unwrap()
            .find("bbb")
            .is_empty());
    }

    #[test]
    fn hits_are_addressed_by_paragraph_and_utf16_offset() {
        let chapter = chapter("第 5 章 长街", "第一段有长街\n\n第🙏二段长街");
        let document = SearchDocument::new(&chapter);
        let matcher = QueryMatcher::new("长街", false).unwrap();

        let hits = document_hits(&document, &matcher, &chapter, 20);

        assert_eq!(hits.len(), 3);
        assert!(hits[0].in_title);
        assert_eq!(hits[0].paragraph_index, None);
        assert_eq!(hits[0].result_index, 0);
        assert_eq!(hits[1].paragraph_index, Some(0));
        assert_eq!(hits[2].paragraph_index, Some(1));
        // 段落内第 5 个字符起（前 4 个字是「第一段有」）。
        assert_eq!((hits[1].match_offset, hits[1].match_length), (4, 2));
        // 偏移以 UTF-16 计数：emoji 占 2 个单位，按字节或按字符都会算错。
        assert_eq!(hits[2].match_offset, 5);
        assert_eq!(hits[2].match_text, "长街");
        assert_eq!(hits[0].progress_percent, 10.0);
    }

    #[test]
    fn snippets_keep_twelve_characters_of_context() {
        let text = format!("{}命中{}", "前".repeat(12), "后".repeat(5));
        let (window, offset, length) = snippet(&text, text.find("命中").unwrap(), "命中".len());
        assert_eq!(window, text);
        assert_eq!((offset, length), (12, 2));

        let long = format!("{}命中{}", "前".repeat(30), "后".repeat(30));
        let (window, offset, _) = snippet(&long, long.find("命中").unwrap(), "命中".len());
        assert_eq!(
            window,
            format!("{}命中{}", "前".repeat(12), "后".repeat(12))
        );
        assert_eq!(offset, 12);
    }
}
