//! Converts the HTML fragments produced by legado content rules into the plain
//! text the reader renders.
//!
//! This mirrors legado's `HtmlFormatter.formatKeepImg` for the text path: block
//! tags (`div`, `p`, `br`, `hr`, `h1..h6`, `article`, `dd`, `dl`) become line
//! breaks, remaining tags are stripped, entities are decoded, and script/style
//! subtrees are dropped.  This project does not render images, so `<img>` is
//! removed instead of kept.

use scraper::{ElementRef, Html};

/// Tags that force a line break, matching legado's `wrapHtmlRegex`.
const BLOCK_TAGS: &[&str] = &[
    "div", "p", "br", "hr", "article", "dd", "dl", "h1", "h2", "h3", "h4", "h5", "h6",
];

/// Tags whose entire subtree is removed before text extraction.
const SKIP_TAGS: &[&str] = &[
    "script",
    "style",
    "noscript",
    "template",
    "head",
    "title",
    "meta",
    "link",
];

/// Turns rule-extracted markup (for example `id.content@html`) into readable
/// paragraphs. Plain text without markup is normalized and returned unchanged.
pub fn html_to_text(html: &str) -> String {
    if html.trim().is_empty() {
        return String::new();
    }
    let document = Html::parse_fragment(html);
    let mut buffer = String::with_capacity(html.len());
    append_children(document.root_element(), &mut buffer);
    normalize(&buffer)
}

fn append_children(element: ElementRef<'_>, buffer: &mut String) {
    for child in element.children() {
        if let Some(text) = child.value().as_text() {
            buffer.push_str(text);
            continue;
        }
        let Some(child) = ElementRef::wrap(child) else {
            continue;
        };
        let name = child.value().name();
        if SKIP_TAGS.contains(&name) {
            continue;
        }
        if name == "br" {
            push_newline(buffer);
            continue;
        }
        let block = BLOCK_TAGS.contains(&name);
        if block {
            push_newline(buffer);
        }
        append_children(child, buffer);
        if block {
            push_newline(buffer);
        }
    }
}

fn push_newline(buffer: &mut String) {
    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }
}

fn normalize(raw: &str) -> String {
    raw.replace('\r', "\n")
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !matches!(c, '\u{2009}' | '\u{200c}' | '\u{200d}'))
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_block_tags_and_br_to_paragraphs() {
        let html = r#"<div class="showtxt" id="content">&nbsp;&nbsp;&nbsp;&nbsp;二月二，龙抬头。<br><br>&nbsp;&nbsp;&nbsp;&nbsp;暮色里，小镇名叫泥瓶巷。</div>"#;
        assert_eq!(
            html_to_text(html),
            "二月二，龙抬头。\n暮色里，小镇名叫泥瓶巷。"
        );
    }

    #[test]
    fn decodes_entities_and_strips_inline_tags() {
        let html = r#"<p>&amp;quot;你好&amp;quot; &lt;剑来&gt;</p>"#;
        assert_eq!(html_to_text(html), "&quot;你好&quot; <剑来>");
    }

    #[test]
    fn drops_script_and_style_subtrees() {
        let html = r#"<style>body{display:none}</style><p>正文</p><script>alert(1)</script>"#;
        assert_eq!(html_to_text(html), "正文");
    }

    #[test]
    fn normalizes_plain_text_without_markup() {
        assert_eq!(
            html_to_text("  第一段  \r\n\r\n  第二段  "),
            "第一段\n第二段"
        );
    }

    #[test]
    fn removes_img_tags() {
        let html = r#"<p>配图<img src="https://example.com/a.jpg">之后</p>"#;
        assert_eq!(html_to_text(html), "配图之后");
    }
}
