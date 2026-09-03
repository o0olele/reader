//! Plain-text import: encoding detection and Legado-compatible chapter rules.

use super::ParsedBook;
use crate::error::AppError;
use fancy_regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;

const DEFAULT_TOC_RULES_JSON: &str = include_str!("../../../assets/defaultData/txtTocRule.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TxtTocRule {
    enable: bool,
    name: String,
    chapter_rule: String,
    volume_rule: String,
    serial_number: i32,
}

#[derive(Debug)]
struct CompiledTocRule {
    chapter_regex: Regex,
    volume_regex: Option<Regex>,
}

static DEFAULT_TOC_RULES: LazyLock<Vec<CompiledTocRule>> = LazyLock::new(|| {
    let mut rules: Vec<TxtTocRule> = serde_json::from_str(DEFAULT_TOC_RULES_JSON)
        .expect("bundled TXT TOC rules must be valid JSON");
    rules.sort_by_key(|rule| rule.serial_number);
    rules
        .into_iter()
        .filter(|rule| rule.enable && !rule.chapter_rule.is_empty())
        .map(|definition| {
            let compatible_pattern = rust_compatible_pattern(&definition.chapter_rule);
            let chapter_regex =
                Regex::new(&format!("(?m){compatible_pattern}")).unwrap_or_else(|error| {
                    panic!("invalid TXT TOC rule {}: {error}", definition.name)
                });
            let volume_regex = if definition.volume_rule.is_empty() {
                None
            } else {
                match Regex::new(&definition.volume_rule) {
                    Ok(regex) => Some(regex),
                    Err(error) => panic!("invalid TXT volume rule {}: {error}", definition.name),
                }
            };
            CompiledTocRule {
                chapter_regex,
                volume_regex,
            }
        })
        .collect()
});

fn rust_compatible_pattern(pattern: &str) -> String {
    // Kotlin/JVM accepts these variable-length look-behinds. Every bundled
    // occurrence represents optional indentation at a line start, so consume
    // that indentation instead; chapter titles are trimmed after matching.
    pattern
        .replace("(?<=[　\\s])", "(?:^|(?<=[\\s　]))")
        .replace("(?<=[\\s　])", "(?:^|(?<=[\\s　]))")
        .replace("(?<=[\\s　]{0,4})", "^[ \\t　]{0,4}")
        .replace("(?<=[ 　\\t]{0,4})", "^[ \\t　]{0,4}")
}

/// Decodes raw bytes, probing BOM-marked UTF-16, then UTF-8, then GBK.
pub fn decode_text(bytes: &[u8]) -> Result<String, &'static str> {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let (text, _, had_errors) = encoding_rs::UTF_16LE.decode(&bytes[2..]);
        return (!had_errors)
            .then_some(text.into_owned())
            .ok_or("无法解码 UTF-16LE 文件");
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let (text, _, had_errors) = encoding_rs::UTF_16BE.decode(&bytes[2..]);
        return (!had_errors)
            .then_some(text.into_owned())
            .ok_or("无法解码 UTF-16BE 文件");
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return Ok(text.to_owned());
    }
    let (text, _, had_errors) = encoding_rs::GBK.decode(bytes);
    (!had_errors)
        .then_some(text.into_owned())
        .ok_or("无法识别文本编码，请使用 UTF-8、GBK 或 UTF-16")
}

/// Splits decoded text using Legado's enabled built-in TXT TOC rules.
///
/// Text with no recognisable heading becomes a single `正文` chapter.
pub fn split_chapters(text: &str) -> Vec<(String, String)> {
    let Some(rule) = select_toc_rule(text) else {
        return vec![(String::from("正文"), text.trim().to_owned())];
    };
    let headings = rule
        .chapter_regex
        .find_iter(text)
        .filter_map(Result::ok)
        .map(|matched| (matched.start(), matched.end()))
        .collect::<Vec<_>>();
    if headings.is_empty() {
        return vec![(String::from("正文"), text.trim().to_owned())];
    }

    let mut chapters = Vec::with_capacity(headings.len() + 1);
    push_chapter(&mut chapters, "正文", &text[..headings[0].0], false);
    for (index, &(start, end)) in headings.iter().enumerate() {
        let title = text[start..end].trim();
        let body_end = headings
            .get(index + 1)
            .map(|&(next_start, _)| next_start)
            .unwrap_or(text.len());
        // The current chapter schema has no `is_volume` column. Keeping an
        // empty volume entry preserves the TOC boundary instead of losing it.
        let is_volume = is_volume_title(rule, title);
        push_chapter(&mut chapters, title, &text[end..body_end], is_volume);
    }
    if chapters.is_empty() {
        vec![(String::from("正文"), text.trim().to_owned())]
    } else {
        chapters
    }
}

fn select_toc_rule(text: &str) -> Option<&'static CompiledTocRule> {
    let mut best = None;
    let mut max_spaced_matches = 1;
    let mut earliest_first_match = usize::MAX;
    // Legado scans the enabled rules in reverse order, then lets an earlier
    // serial number win ties via `>=`.
    for rule in DEFAULT_TOC_RULES.iter().rev() {
        let mut previous_end = None;
        let mut spaced_matches = 0;
        let mut first_match = None;
        for matched in rule.chapter_regex.find_iter(text).filter_map(Result::ok) {
            first_match.get_or_insert(matched.start());
            if previous_end.is_none_or(|end| matched.start().saturating_sub(end) > 1000) {
                spaced_matches += 1;
                previous_end = Some(matched.end());
            }
        }
        let first_match = first_match.unwrap_or(usize::MAX);
        if spaced_matches > 0
            && (spaced_matches > max_spaced_matches
                || spaced_matches == max_spaced_matches && first_match <= earliest_first_match)
        {
            max_spaced_matches = spaced_matches;
            earliest_first_match = first_match;
            best = Some(rule);
        }
    }
    best
}

fn push_chapter(
    chapters: &mut Vec<(String, String)>,
    title: &str,
    content: &str,
    keep_when_empty: bool,
) {
    let content = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    if keep_when_empty || !content.is_empty() {
        chapters.push((title.trim().to_owned(), content));
    }
}

fn is_volume_title(rule: &CompiledTocRule, title: &str) -> bool {
    rule.volume_regex
        .as_ref()
        .is_some_and(|regex| regex.is_match(title).unwrap_or(false))
}

/// Decodes and splits a TXT import into a [`ParsedBook`].
pub fn parse(bytes: &[u8], title: String) -> Result<ParsedBook, AppError> {
    let text = decode_text(bytes).map_err(AppError::parse)?;
    let chapters = split_chapters(&text);
    if chapters
        .iter()
        .all(|(_, content)| content.trim().is_empty())
    {
        return Err(AppError::Parse("TXT 文件中没有可读取的正文".into()));
    }
    Ok(ParsedBook {
        title,
        author: None,
        chapters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_utf8() {
        assert_eq!(decode_text("中文 text".as_bytes()).unwrap(), "中文 text");
    }

    #[test]
    fn decodes_gbk_without_bom() {
        let (bytes, _, _) = encoding_rs::GBK.encode("第一章 开始\n正文内容");
        // Must not be valid UTF-8, otherwise the test would pass via the UTF-8 branch.
        assert!(std::str::from_utf8(&bytes).is_err());
        assert_eq!(decode_text(&bytes).unwrap(), "第一章 开始\n正文内容");
    }

    #[test]
    fn decodes_utf16le_with_bom() {
        let mut bytes = vec![0xFF, 0xFE];
        for unit in "中文".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_text(&bytes).unwrap(), "中文");
    }

    #[test]
    fn decodes_utf16be_with_bom() {
        let mut bytes = vec![0xFE, 0xFF];
        for unit in "中文".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        assert_eq!(decode_text(&bytes).unwrap(), "中文");
    }

    #[test]
    fn splits_on_chapter_headings() {
        let chapters = split_chapters("第一章 开始\n这是正文。\n\n第二章 继续\n下一段。");
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].0, "第一章 开始");
        assert_eq!(chapters[1].1, "下一段。");
    }

    #[test]
    fn recognises_prologue_style_headings() {
        let titles = split_chapters("楔子\na\n序章\nb\n番外 一\nc\n第三章 归来\nd")
            .into_iter()
            .map(|(title, _)| title)
            .collect::<Vec<_>>();
        assert_eq!(titles, ["楔子", "序章", "番外 一", "第三章 归来"]);
    }

    #[test]
    fn keeps_plain_text_as_one_chapter() {
        assert_eq!(
            split_chapters("没有章节标题的文本"),
            vec![("正文".to_owned(), "没有章节标题的文本".to_owned())]
        );
    }

    #[test]
    fn ignores_headings_that_are_too_long_to_be_titles() {
        // A line mentioning 第..章 inside prose must not start a new chapter.
        let prose = format!("第一章{}章", "很长的正文".repeat(10));
        let chapters = split_chapters(&prose);
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].0, "正文");
    }

    #[test]
    fn drops_leading_content_free_headings() {
        // A heading with no body before the next heading produces no empty chapter.
        let chapters = split_chapters("第一章 空\n第二章 有内容\n有效正文");
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].0, "第二章 有内容");
    }

    #[test]
    fn loads_all_bundled_legado_rules() {
        let rules: Vec<serde_json::Value> = serde_json::from_str(DEFAULT_TOC_RULES_JSON).unwrap();
        assert_eq!(rules.len(), 27);
        assert_eq!(rules[0]["id"], -1);
        assert_eq!(rules.last().unwrap()["id"], -100);
        assert_eq!(DEFAULT_TOC_RULES.len(), 12);
        assert!(rules.iter().all(|rule| rule.get("example").is_some()));
    }

    #[test]
    fn recognises_chinese_numerals_and_english_chapters() {
        let chinese = split_chapters("第两百零三章 风起\n甲。");
        assert_eq!(chinese[0].0, "第两百零三章 风起");

        let english = split_chapters("Chapter 204 The Return\n乙。");
        assert_eq!(english[0].0, "Chapter 204 The Return");
    }

    #[test]
    fn recognises_enabled_numeric_title_rule() {
        let chapters = split_chapters("1、起点\n正文一\n02美好的明天\n正文二");
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].0, "1、起点");
        assert_eq!(chapters[1].0, "02美好的明天");
    }

    #[test]
    fn bundled_volume_rule_marks_volume_titles() {
        let rule = DEFAULT_TOC_RULES
            .iter()
            .find(|rule| rule.volume_regex.is_some())
            .unwrap();
        assert!(is_volume_title(rule, "第一卷 北境"));
        assert!(!is_volume_title(rule, "第一章 北境"));
    }

    #[test]
    fn parse_rejects_empty_input() {
        assert!(parse(b"   \n  ", "空书".into()).is_err());
    }

    #[test]
    fn parse_carries_the_supplied_title() {
        let book = parse("第一章 开始\n正文".as_bytes(), "三体".into()).unwrap();
        assert_eq!(book.title, "三体");
        assert_eq!(book.author, None);
        assert_eq!(book.chapters.len(), 1);
    }
}
