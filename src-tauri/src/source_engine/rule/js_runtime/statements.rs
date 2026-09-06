pub(super) fn split_last_statement(script: &str) -> Option<(&str, &str)> {
    let mut quote = None;
    let mut escaped = false;
    let mut depth = 0_i32;
    let mut split = None;
    for (index, character) in script.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote.is_some() {
            if character == '\\' {
                escaped = true;
            } else if Some(character) == quote {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' | '`' => quote = Some(character),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ';' if depth == 0 => split = Some(index),
            _ => {}
        }
    }
    let index = split?;
    let tail = script[index + 1..].trim();
    (!tail.is_empty()).then_some((&script[..index], tail))
}

pub(super) fn has_top_level_return(script: &str) -> bool {
    let mut depth = 0_i32;
    let mut quote = None;
    let mut escaped = false;
    let bytes = script.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let character = bytes[index] as char;
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if let Some(active) = quote {
            if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            index += 1;
            continue;
        }
        if matches!(character, '\'' | '"' | '`') {
            quote = Some(character);
            index += 1;
            continue;
        }
        match character {
            '{' | '(' | '[' => depth += 1,
            '}' | ')' | ']' => depth -= 1,
            _ => {}
        }
        if depth == 0
            && (index == 0 || bytes[index - 1].is_ascii_whitespace() || bytes[index - 1] == b';')
            && script[index..].starts_with("return")
            && script[index + 6..]
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_whitespace() || next == ';')
        {
            return true;
        }
        index += 1;
    }
    false
}
