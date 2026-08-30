//! EPUB import: `META-INF/container.xml` → OPF → manifest/spine → stripped XHTML.

use super::ParsedBook;
use crate::error::AppError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::Path;
use std::sync::LazyLock;
use zip::ZipArchive;

type Archive = ZipArchive<Cursor<Vec<u8>>>;

static HEADING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<h[1-6][^>]*>(.*?)</h[1-6]>").expect("valid heading regex"));
static SCRIPT_OR_STYLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>")
        .expect("valid script/style regex")
});
static ANY_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").expect("valid tag regex"));

/// Reads `META-INF/container.xml`, follows the OPF spine and returns every
/// non-empty document as a chapter. `fallback_title` is used when the OPF
/// carries no `dc:title`.
pub fn parse(bytes: Vec<u8>, fallback_title: String) -> Result<ParsedBook, AppError> {
    if bytes.is_empty() {
        return Err(AppError::InvalidArgument("EPUB 文件不能为空".into()));
    }
    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| AppError::Parse(format!("无法打开 EPUB: {error}")))?;

    let opf_path = read_opf_path(&mut archive)?;
    let opf = zip_entry(&mut archive, &opf_path)?;

    let title = xml_text(&opf, b"dc:title")
        .or_else(|| xml_text(&opf, b"title"))
        .unwrap_or(fallback_title);
    let author = xml_text(&opf, b"dc:creator").or_else(|| xml_text(&opf, b"creator"));

    let (manifest, spine) = read_manifest_and_spine(&opf)?;
    let base = Path::new(&opf_path).parent().unwrap_or(Path::new(""));
    let mut chapters = Vec::new();
    for id in spine {
        let Some(href) = manifest.get(&id) else {
            continue;
        };
        let path = base.join(href).to_string_lossy().replace('\\', "/");
        // A spine entry pointing at a missing or unreadable file is skipped
        // rather than failing the whole import.
        if let Ok(document) = zip_entry(&mut archive, &path) {
            let (chapter_title, content) = clean_xhtml(&document);
            if !content.is_empty() {
                chapters.push((chapter_title, content));
            }
        }
    }
    if chapters.is_empty() {
        return Err(AppError::Parse("EPUB 中没有可读取的正文".into()));
    }
    Ok(ParsedBook {
        title,
        author,
        chapters,
    })
}

fn read_opf_path(archive: &mut Archive) -> Result<String, AppError> {
    let container = zip_entry(archive, "META-INF/container.xml")?;
    let mut reader = Reader::from_reader(container.as_slice());
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"rootfile" =>
            {
                if let Some(path) = xml_attribute(&event, b"full-path") {
                    return Ok(path);
                }
            }
            Ok(Event::Eof) => break,
            Err(error) => {
                return Err(AppError::Parse(format!(
                    "EPUB container.xml 解析失败: {error}"
                )))
            }
            _ => {}
        }
        buffer.clear();
    }
    Err(AppError::Parse("EPUB 未找到 OPF 文件".into()))
}

fn read_manifest_and_spine(opf: &[u8]) -> Result<(HashMap<String, String>, Vec<String>), AppError> {
    let mut manifest = HashMap::new();
    let mut spine = Vec::new();
    let mut reader = Reader::from_reader(opf);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event)) => {
                match event.name().as_ref() {
                    b"item" => {
                        if let (Some(id), Some(href)) =
                            (xml_attribute(&event, b"id"), xml_attribute(&event, b"href"))
                        {
                            manifest.insert(id, href);
                        }
                    }
                    b"itemref" => {
                        if let Some(idref) = xml_attribute(&event, b"idref") {
                            spine.push(idref);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(error) => return Err(AppError::Parse(format!("EPUB OPF 解析失败: {error}"))),
            _ => {}
        }
        buffer.clear();
    }
    Ok((manifest, spine))
}

fn xml_attribute(event: &BytesStart<'_>, name: &[u8]) -> Option<String> {
    event
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == name)
        .and_then(|attr| String::from_utf8(attr.value.into_owned()).ok())
}

fn zip_entry(archive: &mut Archive, path: &str) -> Result<Vec<u8>, AppError> {
    let mut entry = archive
        .by_name(path)
        .map_err(|error| AppError::Parse(format!("EPUB 缺少文件 {path}: {error}")))?;
    let mut output = Vec::new();
    entry.read_to_end(&mut output).map_err(AppError::io)?;
    Ok(output)
}

fn xml_text(bytes: &[u8], wanted: &[u8]) -> Option<String> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut inside = false;
    let mut value = String::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event)) if event.name().as_ref() == wanted => inside = true,
            Ok(Event::Text(event)) if inside => value.push_str(&event.decode().ok()?),
            Ok(Event::End(event)) if event.name().as_ref() == wanted => break,
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => {}
        }
        buffer.clear();
    }
    (!value.trim().is_empty()).then_some(value.trim().to_owned())
}

/// Strips an XHTML document down to `(chapter title, plain text)`.
///
/// The title comes from the first `<h1>`–`<h6>`, falling back to `正文`.
fn clean_xhtml(bytes: &[u8]) -> (String, String) {
    let source = String::from_utf8_lossy(bytes);
    let heading = HEADING
        .captures(&source)
        .map(|captures| captures[1].to_owned());
    let without_scripts = SCRIPT_OR_STYLE.replace_all(&source, "");
    let stripped = ANY_TAG.replace_all(&without_scripts, "\n");
    let content = stripped
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">");
    let content = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let title = heading
        .map(|raw| ANY_TAG.replace_all(&raw, "").trim().to_owned())
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| "正文".into());
    (title, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    const CONTAINER: &str = r#"<?xml version="1.0"?>
<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
  <rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#;

    fn opf(items: &str, spine: &str) -> String {
        format!(
            r#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>星际漫游</dc:title>
    <dc:creator>克拉克</dc:creator>
  </metadata>
  <manifest>{items}</manifest>
  <spine>{spine}</spine>
</package>"#
        )
    }

    fn build_epub(files: &[(&str, &str)]) -> Vec<u8> {
        let mut buffer = Vec::new();
        {
            let mut writer = zip::ZipWriter::new(Cursor::new(&mut buffer));
            for (name, body) in files {
                writer
                    .start_file(*name, SimpleFileOptions::default())
                    .unwrap();
                writer.write_all(body.as_bytes()).unwrap();
            }
            writer.finish().unwrap();
        }
        buffer
    }

    fn sample_epub() -> Vec<u8> {
        build_epub(&[
            ("META-INF/container.xml", CONTAINER),
            (
                "OEBPS/content.opf",
                &opf(
                    r#"<item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/>
                       <item id="c2" href="c2.xhtml" media-type="application/xhtml+xml"/>"#,
                    r#"<itemref idref="c1"/><itemref idref="c2"/>"#,
                ),
            ),
            (
                "OEBPS/c1.xhtml",
                "<html><body><h1>第一章</h1><p>开场白</p></body></html>",
            ),
            (
                "OEBPS/c2.xhtml",
                "<html><body><h1>第二章</h1><p>后续</p></body></html>",
            ),
        ])
    }

    #[test]
    fn reads_metadata_and_spine_order() {
        let book = parse(sample_epub(), "兜底书名".into()).unwrap();
        assert_eq!(book.title, "星际漫游");
        assert_eq!(book.author.as_deref(), Some("克拉克"));
        assert_eq!(
            book.chapters
                .iter()
                .map(|(title, _)| title.as_str())
                .collect::<Vec<_>>(),
            ["第一章", "第二章"]
        );
        assert_eq!(book.chapters[0].1, "第一章\n开场白");
    }

    #[test]
    fn resolves_hrefs_relative_to_the_opf_directory() {
        // c1.xhtml lives at OEBPS/c1.xhtml, not at the archive root.
        let book = parse(sample_epub(), "兜底书名".into()).unwrap();
        assert_eq!(book.chapters.len(), 2);
    }

    #[test]
    fn skips_spine_entries_whose_file_is_missing() {
        let bytes = build_epub(&[
            ("META-INF/container.xml", CONTAINER),
            (
                "OEBPS/content.opf",
                &opf(
                    r#"<item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/>
                       <item id="gone" href="gone.xhtml" media-type="application/xhtml+xml"/>"#,
                    r#"<itemref idref="c1"/><itemref idref="gone"/>"#,
                ),
            ),
            ("OEBPS/c1.xhtml", "<html><body><p>仅此一章</p></body></html>"),
        ]);
        let book = parse(bytes, "兜底书名".into()).unwrap();
        assert_eq!(book.chapters.len(), 1);
        assert_eq!(book.chapters[0].0, "正文");
    }

    #[test]
    fn falls_back_to_the_supplied_title_without_dc_title() {
        let bytes = build_epub(&[
            ("META-INF/container.xml", CONTAINER),
            (
                "OEBPS/content.opf",
                r#"<?xml version="1.0"?><package><manifest><item id="c1" href="c1.xhtml"/></manifest><spine><itemref idref="c1"/></spine></package>"#,
            ),
            ("OEBPS/c1.xhtml", "<html><body><p>正文</p></body></html>"),
        ]);
        let book = parse(bytes, "兜底书名".into()).unwrap();
        assert_eq!(book.title, "兜底书名");
        assert_eq!(book.author, None);
    }

    #[test]
    fn rejects_an_archive_without_a_container() {
        let bytes = build_epub(&[("mimetype", "application/epub+zip")]);
        assert!(parse(bytes, "兜底书名".into()).is_err());
    }

    #[test]
    fn rejects_empty_bytes() {
        assert!(parse(Vec::new(), "兜底书名".into()).is_err());
    }

    #[test]
    fn rejects_an_epub_whose_documents_are_all_blank() {
        let bytes = build_epub(&[
            ("META-INF/container.xml", CONTAINER),
            (
                "OEBPS/content.opf",
                &opf(
                    r#"<item id="c1" href="c1.xhtml"/>"#,
                    r#"<itemref idref="c1"/>"#,
                ),
            ),
            ("OEBPS/c1.xhtml", "<html><body></body></html>"),
        ]);
        assert!(parse(bytes, "兜底书名".into()).is_err());
    }

    #[test]
    fn strips_scripts_styles_and_entities() {
        let (title, content) = clean_xhtml(
            b"<html><body><h1>Chapter One</h1><p>Hello&nbsp;world</p><script>bad()</script><style>p{}</style></body></html>",
        );
        assert_eq!(title, "Chapter One");
        assert_eq!(content, "Chapter One\nHello world");
    }

    #[test]
    fn unwraps_markup_inside_the_heading() {
        let (title, _) = clean_xhtml(b"<h2><span>\xe7\xac\xac\xe4\xb8\x89\xe7\xab\xa0</span></h2><p>x</p>");
        assert_eq!(title, "第三章");
    }

    #[test]
    fn uses_the_default_title_when_there_is_no_heading() {
        let (title, content) = clean_xhtml(b"<html><body><p>only prose</p></body></html>");
        assert_eq!(title, "正文");
        assert_eq!(content, "only prose");
    }
}
