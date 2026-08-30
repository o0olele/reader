//! Ebook decoding adapters: local file bytes to ordered chapters.

pub mod epub;
pub mod txt;

/// A locally imported book: metadata plus ordered `(title, content)` chapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedBook {
    pub title: String,
    pub author: Option<String>,
    pub chapters: Vec<(String, String)>,
}

/// Derives a display title from an import filename, dropping the extension.
pub fn title_from_filename(filename: &str) -> String {
    std::path::Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("未命名书籍")
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_extension_and_directory() {
        assert_eq!(title_from_filename("books/三体.txt"), "三体");
        assert_eq!(title_from_filename("三体.epub"), "三体");
    }

    #[test]
    fn falls_back_when_there_is_no_stem() {
        assert_eq!(title_from_filename(""), "未命名书籍");
    }
}
