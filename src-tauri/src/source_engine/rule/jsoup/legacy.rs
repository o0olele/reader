//! Legacy JSoup spelling normalization and CSS compatibility helpers.

/// Legado exports both `@attr(name)`/`:attr(name)` and the CSS-looking
/// `::attr(name)` spelling.  Normalize those terminal forms before the
/// `@`-chain parser sees them; otherwise scraper treats `:attr` as an
/// unsupported pseudo-class and rejects the whole source.
pub(super) fn normalize_rule(raw: &str) -> String {
    let mut output = String::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let marker = if raw[index..].starts_with("::attr(") {
            Some(7)
        } else if raw[index..].starts_with(":attr(") {
            Some(6)
        } else {
            None
        };
        let Some(marker_len) = marker else {
            let length = raw[index..].chars().next().map(char::len_utf8).unwrap_or(1);
            output.push_str(&raw[index..index + length]);
            index += length;
            continue;
        };
        let start = index + marker_len;
        let Some(end_rel) = raw[start..].find(')') else {
            output.push_str(&raw[index..]);
            break;
        };
        let name = raw[start..start + end_rel].trim();
        if name.is_empty()
            || !name.chars().all(|character| {
                character.is_ascii_alphanumeric() || character == '-' || character == '_'
            })
        {
            output.push_str(&raw[index..start + end_rel + 1]);
        } else {
            output.push('@');
            output.push_str(name);
        }
        index = start + end_rel + 1;
    }
    output
}

pub(super) fn normalize_css_compat(css: &str) -> String {
    let mut out = css.to_owned();
    // Legado uses an escaped pipe as a selector union in a few exported
    // sources (for example `src\|class.red`). `scraper` interprets the pipe
    // as a namespace separator, so translate it to CSS's selector-list comma
    // when it is outside an attribute selector.
    if !out.contains('[') && (out.contains(r"\|") || out.contains('|')) {
        out = out.replace(r"\|", ",").replace('|', ",");
    }
    let pseudo = regex::Regex::new(r#":(?:contains|eq)\(\s*['"]?.*?['"]?\s*\)"#).unwrap();
    out = pseudo.replace_all(&out, "").into_owned();
    // scraper requires quoted attribute values; JSoup accepts these legacy forms.
    let re = regex::Regex::new(r#"\[([\w:-]+)([~|^$*]?=)([^\]"']+)\]"#).unwrap();
    out = re
        .replace_all(&out, |caps: &regex::Captures| {
            format!("[{}{}'{}']", &caps[1], &caps[2], caps[3].trim())
        })
        .into_owned();
    let re_space = regex::Regex::new(r#"\[([\w:-]+)=([^\]]*\s[^\]]*)\]"#).unwrap();
    re_space
        .replace_all(&out, |caps: &regex::Captures| {
            format!("[{}='{}']", &caps[1], caps[2].trim())
        })
        .into_owned()
}

pub(super) fn css_contains(css: &str) -> Option<String> {
    let re = regex::Regex::new(r#":contains\(\s*['"]?(.*?)['"]?\s*\)"#).unwrap();
    re.captures(css).map(|c| c[1].to_owned())
}

pub(super) fn css_eq(css: &str) -> Option<usize> {
    let re = regex::Regex::new(r#":eq\(\s*(\d+)\s*\)"#).unwrap();
    re.captures(css).and_then(|c| c[1].parse().ok())
}
