//! Plain-text import: encoding detection and heuristic chapter splitting.

use super::ParsedBook;
use crate::error::AppError;

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

/// Splits decoded text on `第X章/节/回`, `序章`, `楔子` and `番外` headings.
///
/// Text with no recognisable heading becomes a single `正文` chapter.
pub fn split_chapters(text: &str) -> Vec<(String, String)> {
    let mut chapters = Vec::new();
    let mut current_title = String::from("正文");
    let mut current = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if is_heading(trimmed) {
            if !current.trim().is_empty() {
                chapters.push((current_title, current.trim().to_owned()));
            }
            current_title = trimmed.to_owned();
            current.clear();
        } else if !trimmed.is_empty() {
            if !current.is_empty() {
                current.push('\n');
            }
            current.push_str(trimmed);
        }
    }
    if !current.trim().is_empty() {
        chapters.push((current_title, current.trim().to_owned()));
    }
    if chapters.is_empty() {
        vec![(String::from("正文"), text.trim().to_owned())]
    } else {
        chapters
    }
}

fn is_heading(trimmed: &str) -> bool {
    trimmed.len() >= 2
        && trimmed.len() <= 40
        && (trimmed.starts_with("第")
            && (trimmed.contains('章') || trimmed.contains('节') || trimmed.contains('回'))
            || trimmed.starts_with("序章")
            || trimmed.starts_with("楔子")
            || trimmed.starts_with("番外"))
}

/// Decodes and splits a TXT import into a [`ParsedBook`].
pub fn parse(bytes: &[u8], title: String) -> Result<ParsedBook, AppError> {
    let text = decode_text(bytes).map_err(AppError::parse)?;
    let chapters = split_chapters(&text);
    if chapters.iter().all(|(_, content)| content.trim().is_empty()) {
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
        let titles = split_chapters("楔子\na\n序章\nb\n番外 一\nc\n第三回 归来\nd")
            .into_iter()
            .map(|(title, _)| title)
            .collect::<Vec<_>>();
        assert_eq!(titles, ["楔子", "序章", "番外 一", "第三回 归来"]);
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
        let chapters = split_chapters("第一章 空\n第二章 有内容\n正文");
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].0, "第二章 有内容");
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
