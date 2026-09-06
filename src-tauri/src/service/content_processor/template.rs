use crate::error::AppError;

pub(super) enum Part {
    Text(String),
    Group(usize),
}

/// Java replacement groups consume only digits that form an existing group.
pub(super) fn parse(value: &str, regex: &fancy_regex::Regex) -> Result<Vec<Part>, AppError> {
    let invalid = || AppError::InvalidArgument("替换文本包含无效的捕获组引用".into());
    let mut parts = Vec::new();
    let mut text = String::new();
    let mut chars = value.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '\\' => text.push(chars.next().ok_or_else(invalid)?),
            '$' => {
                if !text.is_empty() {
                    parts.push(Part::Text(std::mem::take(&mut text)));
                }
                let next = chars.next().ok_or_else(invalid)?;
                let group = if next == '{' {
                    let mut name = String::new();
                    let mut closed = false;
                    for character in chars.by_ref() {
                        if character == '}' {
                            closed = true;
                            break;
                        }
                        name.push(character);
                    }
                    if !closed || name.is_empty() {
                        return Err(invalid());
                    }
                    regex
                        .capture_names()
                        .position(|candidate| candidate == Some(name.as_str()))
                        .ok_or_else(invalid)?
                } else {
                    let mut group = next.to_digit(10).ok_or_else(invalid)? as usize;
                    if group >= regex.captures_len() {
                        return Err(invalid());
                    }
                    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
                        let candidate = group * 10 + digit as usize;
                        if candidate >= regex.captures_len() {
                            break;
                        }
                        chars.next();
                        group = candidate;
                    }
                    group
                };
                parts.push(Part::Group(group));
            }
            other => text.push(other),
        }
    }
    if !text.is_empty() {
        parts.push(Part::Text(text));
    }
    Ok(parts)
}

pub(super) fn expand(parts: &[Part], captures: &fancy_regex::Captures<'_>, output: &mut String) {
    for part in parts {
        match part {
            Part::Text(text) => output.push_str(text),
            Part::Group(group) => {
                if let Some(value) = captures.get(*group) {
                    output.push_str(value.as_str());
                }
            }
        }
    }
}
