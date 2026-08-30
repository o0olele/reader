use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub path: Option<String>,
    pub group_id: Option<i64>,
    pub source_id: Option<i64>,
    pub remote_url: Option<String>,
    pub chapter_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    pub id: i64,
    pub book_id: i64,
    pub title: String,
    pub number: i64,
    pub content: String,
    pub remote_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadingProgress {
    pub book_id: i64,
    pub chapter_id: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookshelfGroup {
    pub id: i64,
    pub name: String,
    pub book_count: i64,
}

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

pub fn split_chapters(text: &str) -> Vec<(String, String)> {
    let mut chapters = Vec::new();
    let mut current_title = String::from("正文");
    let mut current = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        let is_heading = trimmed.len() >= 2
            && trimmed.len() <= 40
            && (trimmed.starts_with("第")
                && (trimmed.contains('章') || trimmed.contains('节') || trimmed.contains('回'))
                || trimmed.starts_with("序章")
                || trimmed.starts_with("楔子")
                || trimmed.starts_with("番外"));
        if is_heading {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_utf8_and_detects_chapters() {
        let text = "第一章 开始\n这是正文。\n\n第二章 继续\n下一段。";
        let chapters = split_chapters(text);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].0, "第一章 开始");
        assert_eq!(chapters[1].1, "下一段。");
    }

    #[test]
    fn keeps_plain_text_as_one_chapter() {
        let chapters = split_chapters("没有章节标题的文本");
        assert_eq!(
            chapters,
            vec![("正文".to_owned(), "没有章节标题的文本".to_owned())]
        );
    }
}
