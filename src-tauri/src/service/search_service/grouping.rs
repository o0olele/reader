//! Merging the same book across the sources that returned it.

use crate::domain::source::BookSearchResult;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize)]
pub struct SearchResultGroup {
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub sources: Vec<BookSearchResult>,
}

fn group_key(result: &BookSearchResult) -> (String, String) {
    fn normalize(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(char::to_lowercase)
            .collect()
    }
    (
        normalize(&result.title),
        normalize(result.author.as_deref().unwrap_or_default()),
    )
}

/// Groups results by (title, author), keeping first-seen order and dropping a
/// URL that the same source returned twice.
pub fn group_results(results: Vec<BookSearchResult>) -> Vec<SearchResultGroup> {
    let mut groups = Vec::new();
    let mut indexes = HashMap::new();
    let mut seen = HashSet::new();
    for result in results {
        if !seen.insert((result.source_id, result.url.clone())) {
            continue;
        }
        if let Some(&index) = indexes.get(&group_key(&result)) {
            let group: &mut SearchResultGroup = &mut groups[index];
            if group.cover.is_none() {
                group.cover = result.cover.clone();
            }
            group.sources.push(result);
        } else {
            indexes.insert(group_key(&result), groups.len());
            groups.push(SearchResultGroup {
                title: result.title.clone(),
                author: result.author.clone(),
                cover: result.cover.clone(),
                sources: vec![result],
            });
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(source_id: i64, title: &str, author: Option<&str>, url: &str) -> BookSearchResult {
        BookSearchResult {
            source_id,
            source_name: format!("source-{source_id}"),
            title: title.into(),
            author: author.map(str::to_owned),
            cover: None,
            url: url.into(),
        }
    }

    #[test]
    fn merges_the_same_book_found_on_several_sources() {
        let groups = group_results(vec![
            result(1, "斗破苍穹", Some("天蚕土豆"), "https://a.test/1"),
            result(2, "斗破苍穹 ", Some("天蚕土豆"), "https://b.test/9"),
        ]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].sources.len(), 2);
    }

    #[test]
    fn keeps_different_books_apart() {
        let groups = group_results(vec![
            result(1, "斗破苍穹", Some("天蚕土豆"), "https://a.test/1"),
            result(1, "武动乾坤", Some("天蚕土豆"), "https://a.test/2"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn same_title_by_a_different_author_is_a_different_book() {
        let groups = group_results(vec![
            result(1, "长安", Some("甲"), "https://a.test/1"),
            result(2, "长安", Some("乙"), "https://b.test/1"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn drops_a_url_repeated_by_the_same_source() {
        let groups = group_results(vec![
            result(1, "重复", None, "https://a.test/1"),
            result(1, "重复", None, "https://a.test/1"),
        ]);
        assert_eq!(groups[0].sources.len(), 1);
    }

    #[test]
    fn keeps_the_same_url_when_it_comes_from_different_sources() {
        let groups = group_results(vec![
            result(1, "镜像", None, "https://a.test/1"),
            result(2, "镜像", None, "https://a.test/1"),
        ]);
        assert_eq!(groups[0].sources.len(), 2);
    }

    #[test]
    fn backfills_a_missing_cover_from_a_later_source() {
        let first = result(1, "补全", Some("作者"), "https://a.test/1");
        let mut second = result(2, "补全", Some("作者"), "https://b.test/1");
        second.cover = Some("https://b.test/cover.jpg".into());
        let groups = group_results(vec![first, second]);
        assert_eq!(groups[0].cover.as_deref(), Some("https://b.test/cover.jpg"));
    }

    #[test]
    fn a_missing_author_does_not_merge_into_a_named_one() {
        let groups = group_results(vec![
            result(1, "同名", None, "https://a.test/1"),
            result(2, "同名", Some("作者"), "https://b.test/1"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn preserves_first_seen_order() {
        let groups = group_results(vec![
            result(1, "第二本", None, "https://a.test/2"),
            result(1, "第一本", None, "https://a.test/1"),
        ]);
        assert_eq!(
            groups
                .iter()
                .map(|group| group.title.as_str())
                .collect::<Vec<_>>(),
            ["第二本", "第一本"]
        );
    }
}
