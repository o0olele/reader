//! Typed access to the legado rule objects stored verbatim on a source.
//!
//! Legado's JSON exports spell the same rule under several keys, and allow a
//! bare string where an object is expected (`"ruleContent": ".content"`). The
//! alias lists below mirror the ones the CSS importer already relies on in
//! [`crate::source_engine::import`].

use crate::domain::source::RawSourceRules;
use serde::Deserialize;

fn parse<T: Default + for<'de> Deserialize<'de>>(raw: Option<&String>, bare: &str) -> Option<T> {
    let raw = raw?.trim();
    if raw.is_empty() || raw == "null" {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(raw).ok()?;
    // A bare string stands for the object's primary rule.
    if let Some(text) = value.as_str() {
        let mut object = serde_json::Map::new();
        object.insert(bare.to_owned(), text.into());
        return serde_json::from_value(serde_json::Value::Object(object)).ok();
    }
    serde_json::from_value(value).ok()
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct LegadoSearchRule {
    #[serde(alias = "bookList", alias = "list", alias = "item")]
    pub book_list: Option<String>,
    #[serde(alias = "title")]
    pub name: Option<String>,
    pub author: Option<String>,
    #[serde(alias = "bookUrl", alias = "url", alias = "detail")]
    pub book_url: Option<String>,
    #[serde(alias = "coverUrl", alias = "cover")]
    pub cover_url: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct LegadoBookInfoRule {
    #[serde(alias = "title")]
    pub name: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    #[serde(alias = "lastChapter", alias = "latestChapter")]
    pub last_chapter: Option<String>,
    #[serde(alias = "coverUrl", alias = "cover")]
    pub cover_url: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct LegadoTocRule {
    #[serde(alias = "chapterList", alias = "list", alias = "item")]
    pub chapter_list: Option<String>,
    #[serde(alias = "chapterName", alias = "name", alias = "title")]
    pub chapter_name: Option<String>,
    #[serde(alias = "chapterUrl", alias = "url")]
    pub chapter_url: Option<String>,
    #[serde(alias = "nextTocUrl", alias = "nextUrl", alias = "next")]
    pub next_toc_url: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct LegadoContentRule {
    #[serde(alias = "selector", alias = "main")]
    pub content: Option<String>,
    #[serde(alias = "nextContentUrl", alias = "nextUrl", alias = "next")]
    pub next_content_url: Option<String>,
}

/// The four rule objects of one source, as far as they could be decoded.
#[derive(Debug, Default)]
pub struct LegadoRules {
    pub search: Option<LegadoSearchRule>,
    pub book_info: Option<LegadoBookInfoRule>,
    pub toc: Option<LegadoTocRule>,
    pub content: Option<LegadoContentRule>,
}

impl LegadoRules {
    pub fn decode(raw: &RawSourceRules) -> Self {
        Self {
            search: parse(raw.search.as_ref(), "bookList"),
            book_info: parse(raw.book_info.as_ref(), "name"),
            toc: parse(raw.toc.as_ref(), "chapterList"),
            content: parse(raw.content.as_ref(), "content"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(search: &str, content: &str) -> RawSourceRules {
        RawSourceRules {
            search: Some(search.into()),
            content: Some(content.into()),
            ..Default::default()
        }
    }

    #[test]
    fn decodes_legado_key_spellings() {
        let rules = LegadoRules::decode(&raw(
            r#"{"bookList":"class.book","name":".title","bookUrl":"a@href"}"#,
            "{}",
        ));
        let search = rules.search.unwrap();
        assert_eq!(search.book_list.as_deref(), Some("class.book"));
        assert_eq!(search.name.as_deref(), Some(".title"));
        assert_eq!(search.book_url.as_deref(), Some("a@href"));
    }

    #[test]
    fn accepts_alternative_key_spellings() {
        let rules = LegadoRules::decode(&raw(r#"{"list":"article","detail":"a"}"#, "{}"));
        let search = rules.search.unwrap();
        assert_eq!(search.book_list.as_deref(), Some("article"));
        assert_eq!(search.book_url.as_deref(), Some("a"));
    }

    #[test]
    fn expands_a_bare_string_into_the_primary_rule() {
        let rules = LegadoRules::decode(&raw("{}", r#""id.content@textNodes""#));
        assert_eq!(
            rules.content.unwrap().content.as_deref(),
            Some("id.content@textNodes")
        );
    }

    #[test]
    fn treats_missing_and_unparsable_objects_as_absent() {
        let rules = LegadoRules::decode(&RawSourceRules::default());
        assert!(rules.search.is_none());
        assert!(rules.toc.is_none());
        assert!(LegadoRules::decode(&raw("not json", "null"))
            .search
            .is_none());
        assert!(LegadoRules::decode(&raw("{}", "null")).content.is_none());
    }
}
