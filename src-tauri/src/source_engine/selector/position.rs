//! Alternative splitting and legacy position/index filtering for the CSS fallback.

pub(super) fn split_alternatives(raw: &str) -> Vec<&str> {
    let mut branches = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    let mut quote = None;
    let bytes = raw.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let character = bytes[index] as char;
        match quote {
            Some(active) if character == active => quote = None,
            Some(_) => {}
            None => match character {
                '\'' | '"' => quote = Some(character),
                '[' | '(' => depth += 1,
                ']' | ')' => depth = (depth - 1).max(0),
                '|' if depth == 0 && bytes.get(index + 1) == Some(&b'|') => {
                    branches.push(raw[start..index].trim());
                    index += 1;
                    start = index + 1;
                }
                _ => {}
            },
        }
        index += 1;
    }
    branches.push(raw[start..].trim());
    branches
}

/// Removes a trailing legado index/range expression from a CSS projection.
/// The rule engine executes these filters with the correct semantics; the
/// fallback only needs a valid selector and therefore deliberately ignores the
/// position when projecting to scraper CSS.
pub(super) fn strip_legacy_position(value: &str) -> String {
    let value = value.trim();
    if let Some(content) = value.strip_suffix(']') {
        if let Some(open) = content.rfind('[') {
            let expression = content[open + 1..].trim();
            if is_position_expression(expression) {
                return content[..open].trim_end().to_owned();
            }
        }
    }
    if let Some(bang) = value.rfind('!') {
        let suffix = value[bang + 1..].trim();
        if is_index_list(suffix) {
            return value[..bang].trim_end_matches('.').trim_end().to_owned();
        }
    }
    for (dot, _) in value.match_indices('.').rev() {
        let suffix = value[dot + 1..].trim();
        if is_index_list(suffix) {
            return value[..dot].trim_end().to_owned();
        }
    }
    value.to_owned()
}

fn is_position_expression(value: &str) -> bool {
    let value = value.strip_prefix('!').unwrap_or(value).trim();
    !value.is_empty()
        && value.split(',').all(|part| {
            let part = part.trim();
            if part.contains(':') {
                let pieces = part.split(':').collect::<Vec<_>>();
                (2..=3).contains(&pieces.len())
                    && pieces
                        .iter()
                        .all(|piece| piece.trim().is_empty() || piece.trim().parse::<i32>().is_ok())
            } else {
                part.parse::<i32>().is_ok()
            }
        })
}

fn is_index_list(value: &str) -> bool {
    !value.is_empty()
        && value
            .split(':')
            .all(|part| part.trim().parse::<i32>().is_ok())
}
