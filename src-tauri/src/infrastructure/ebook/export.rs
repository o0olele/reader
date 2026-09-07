//! Ebook encoders used by the offline download/export workflow.

use crate::{domain::Chapter, error::AppError};
use std::io::{Cursor, Write};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

pub fn txt(title: &str, author: Option<&str>, chapters: &[Chapter]) -> Vec<u8> {
    let mut output = String::new();
    output.push('\u{feff}');
    output.push_str(title.trim());
    output.push('\n');
    if let Some(author) = author.filter(|value| !value.trim().is_empty()) {
        output.push_str("作者：");
        output.push_str(author.trim());
        output.push('\n');
    }
    for chapter in chapters {
        output.push_str("\n\n");
        output.push_str(chapter.title.trim());
        output.push_str("\n\n");
        output.push_str(chapter.content.trim());
    }
    output.push('\n');
    output.into_bytes()
}

pub fn epub(title: &str, author: Option<&str>, chapters: &[Chapter]) -> Result<Vec<u8>, AppError> {
    let cursor = Cursor::new(Vec::new());
    let mut archive = ZipWriter::new(cursor);
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let compressed = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    write_entry(&mut archive, "mimetype", stored, "application/epub+zip")?;
    write_entry(
        &mut archive,
        "META-INF/container.xml",
        compressed,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )?;

    let escaped_title = xml(title);
    let escaped_author = xml(author.unwrap_or("未知作者"));
    let identifier = format!("urn:uuid:{}", uuid::Uuid::new_v4());
    write_entry(
        &mut archive,
        "OEBPS/content.opf",
        compressed,
        &package(&escaped_title, &escaped_author, &identifier, chapters.len()),
    )?;
    write_entry(
        &mut archive,
        "OEBPS/nav.xhtml",
        compressed,
        &navigation(&escaped_title, chapters),
    )?;
    for (index, chapter) in chapters.iter().enumerate() {
        write_entry(
            &mut archive,
            &format!("OEBPS/chapter-{}.xhtml", index + 1),
            compressed,
            &chapter_xhtml(&chapter.title, &chapter.content),
        )?;
    }
    archive
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| AppError::Io(format!("EPUB 写入失败: {error}")))
}

fn write_entry(
    archive: &mut ZipWriter<Cursor<Vec<u8>>>,
    path: &str,
    options: SimpleFileOptions,
    content: &str,
) -> Result<(), AppError> {
    archive
        .start_file(path, options)
        .map_err(|error| AppError::Io(format!("EPUB 条目 {path} 创建失败: {error}")))?;
    archive
        .write_all(content.as_bytes())
        .map_err(|error| AppError::Io(format!("EPUB 条目 {path} 写入失败: {error}")))
}

fn package(title: &str, author: &str, identifier: &str, count: usize) -> String {
    let manifest = (1..=count)
        .map(|index| format!(r#"    <item id="chapter-{index}" href="chapter-{index}.xhtml" media-type="application/xhtml+xml"/>"#))
        .collect::<Vec<_>>()
        .join("\n");
    let spine = (1..=count)
        .map(|index| format!(r#"    <itemref idref="chapter-{index}"/>"#))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="book-id" xml:lang="zh-CN">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:identifier id="book-id">{identifier}</dc:identifier>
    <dc:title>{title}</dc:title><dc:creator>{author}</dc:creator><dc:language>zh-CN</dc:language>
  </metadata>
  <manifest>
    <item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>
{manifest}
  </manifest>
  <spine>
{spine}
  </spine>
</package>"#
    )
}

fn navigation(title: &str, chapters: &[Chapter]) -> String {
    let items = chapters
        .iter()
        .enumerate()
        .map(|(index, chapter)| {
            format!(
                r#"      <li><a href="chapter-{}.xhtml">{}</a></li>"#,
                index + 1,
                xml(&chapter.title)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html><html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="zh-CN">
<head><meta charset="UTF-8"/><title>{title}</title></head>
<body><nav epub:type="toc"><h1>{title}</h1><ol>
{items}
    </ol></nav></body></html>"#
    )
}

fn chapter_xhtml(title: &str, content: &str) -> String {
    let paragraphs = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("<p>{}</p>", xml(line)))
        .collect::<Vec<_>>()
        .join("\n");
    let title = xml(title);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html><html xmlns="http://www.w3.org/1999/xhtml" lang="zh-CN">
<head><meta charset="UTF-8"/><title>{title}</title><style>body{{line-height:1.8;margin:5%;}}p{{text-indent:2em;}}</style></head>
<body><h1>{title}</h1>{paragraphs}</body></html>"#
    )
}

fn xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn chapters() -> Vec<Chapter> {
        vec![Chapter {
            id: 1,
            book_id: 1,
            title: "第一章 & 开始".into(),
            number: 0,
            content: "第一段\n第二段".into(),
            remote_url: None,
        }]
    }

    #[test]
    fn txt_export_contains_metadata_and_content() {
        let text = String::from_utf8(txt("书名", Some("作者"), &chapters())).unwrap();
        assert!(text.contains("书名\n作者：作者"));
        assert!(text.contains("第一章 & 开始\n\n第一段"));
    }

    #[test]
    fn epub_export_is_readable_and_escapes_xml() {
        let bytes = epub("书名", Some("作者"), &chapters()).unwrap();
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
        assert_eq!(
            archive.by_name("mimetype").unwrap().compression(),
            CompressionMethod::Stored
        );
        let mut chapter = String::new();
        archive
            .by_name("OEBPS/chapter-1.xhtml")
            .unwrap()
            .read_to_string(&mut chapter)
            .unwrap();
        assert!(chapter.contains("第一章 &amp; 开始"));
        assert!(chapter.contains("<p>第二段</p>"));
    }
}
