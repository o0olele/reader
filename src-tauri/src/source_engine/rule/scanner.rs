use super::model::RuleParseError;

#[derive(Clone, Copy)]
pub(super) enum Separator {
    Alternative,
    Concat,
    Interleave,
}

impl Separator {
    pub(super) fn text(self) -> &'static str {
        match self {
            Self::Alternative => "||",
            Self::Concat => "&&",
            Self::Interleave => "%%",
        }
    }

    pub(super) fn is_alternative(self) -> bool {
        matches!(self, Self::Alternative)
    }

    pub(super) fn is_concat(self) -> bool {
        matches!(self, Self::Concat)
    }
}

pub(super) fn split_top_level(
    raw: &str,
) -> Result<Vec<(String, Option<Separator>)>, RuleParseError> {
    let trimmed = raw.trim_start();
    if trimmed.starts_with(':') || trimmed.starts_with("-:") {
        return Ok(vec![(trimmed.to_owned(), None)]);
    }
    let mut result = Vec::new();
    let mut start = 0;
    let mut index = 0;
    let mut quote = None;
    let mut escaped = false;
    let mut stack = Vec::new();
    let mut js_block = false;
    let mut tail_js = false;

    while index < raw.len() {
        if js_block {
            if starts_ignore_ascii_case(raw, index, "</js>") {
                js_block = false;
                index += 5;
            } else {
                index += char_len(raw, index);
            }
            continue;
        }
        if tail_js {
            break;
        }
        if escaped {
            escaped = false;
            index += char_len(raw, index);
            continue;
        }

        let byte = raw.as_bytes()[index];
        if byte == b'\\' {
            escaped = true;
            index += 1;
            continue;
        }
        if let Some(active) = quote {
            if byte == active {
                quote = None;
            }
            index += char_len(raw, index);
            continue;
        }
        if byte == b'\'' || byte == b'"' || byte == b'`' {
            quote = Some(byte);
            index += 1;
            continue;
        }
        if starts_ignore_ascii_case(raw, index, "<js>") {
            js_block = true;
            index += 4;
            continue;
        }
        if starts_ignore_ascii_case(raw, index, "@js:")
            || starts_ignore_ascii_case(raw, index, "@webjs:")
        {
            tail_js = true;
            continue;
        }

        match byte {
            b'[' | b'(' | b'{' => stack.push(byte),
            b']' | b')' | b'}' => {
                let expected = match byte {
                    b']' => b'[',
                    b')' => b'(',
                    _ => b'{',
                };
                if stack.last() == Some(&expected) {
                    stack.pop();
                }
            }
            _ => {}
        }

        if stack.is_empty() {
            let separator = if raw[index..].starts_with("||") {
                Some(Separator::Alternative)
            } else if raw[index..].starts_with("&&") {
                Some(Separator::Concat)
            } else if raw[index..].starts_with("%%") {
                Some(Separator::Interleave)
            } else {
                None
            };
            if let Some(separator) = separator {
                result.push((raw[start..index].to_owned(), Some(separator)));
                index += 2;
                start = index;
                continue;
            }
        }
        index += char_len(raw, index);
    }

    if quote.is_some() {
        return Err(RuleParseError::Unclosed("quote"));
    }
    if js_block {
        return Err(RuleParseError::Unclosed("<js> block"));
    }
    if !stack.is_empty() {
        return Err(RuleParseError::Unclosed("balanced group"));
    }
    result.push((raw[start..].to_owned(), None));
    Ok(result)
}

pub(super) fn find_ignore_ascii_case(raw: &str, from: usize, needle: &str) -> Option<usize> {
    raw[from..]
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())
        .map(|relative| from + relative)
}

pub(super) fn starts_ignore_ascii_case(raw: &str, index: usize, needle: &str) -> bool {
    raw.get(index..index + needle.len())
        .is_some_and(|value| value.eq_ignore_ascii_case(needle))
}

fn char_len(raw: &str, index: usize) -> usize {
    raw[index..].chars().next().map(char::len_utf8).unwrap_or(1)
}
