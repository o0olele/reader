

pub(super) fn declare_implicit_assignments(script: &str) -> String {
    let mut names = Vec::new();
    let bytes = script.as_bytes();
    let mut index = 0;
    let mut quote = None;
    let mut escaped = false;
    let mut statement_start = true;
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
        if character == '/' && bytes.get(index + 1) == Some(&b'/') {
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            statement_start = true;
            continue;
        }
        if character == ';' || character == '\n' || character == '{' {
            statement_start = true;
            index += 1;
            continue;
        }
        if character.is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if statement_start
            && (character == '_' || character == '$' || character.is_ascii_alphabetic())
        {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index] == b'_'
                    || bytes[index] == b'$'
                    || bytes[index].is_ascii_alphanumeric())
            {
                index += 1;
            }
            let name = &script[start..index];
            let mut cursor = index;
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            let declared = matches!(
                name,
                "var" | "let" | "const" | "function" | "if" | "for" | "while" | "return"
            );
            let known_global = matches!(
                name,
                "result"
                    | "url"
                    | "key"
                    | "baseUrl"
                    | "page"
                    | "java"
                    | "source"
                    | "book"
                    | "chapter"
                    | "content"
            );
            if !declared
                && !known_global
                && bytes.get(cursor) == Some(&b'=')
                && bytes.get(cursor + 1) != Some(&b'=')
                && bytes.get(cursor + 1) != Some(&b'>')
                && !names.iter().any(|existing| existing == name)
            {
                names.push(name.to_owned());
            }
            statement_start = false;
            continue;
        }
        statement_start = false;
        index += 1;
    }
    if names.is_empty() {
        script.to_owned()
    } else {
        format!("var {};\n{script}", names.join(", "))
    }
}

pub(super) fn declare_implicit_assignment(line: &str) -> String {
    let trimmed = line.trim_start();
    let Some(equal) = trimmed.find('=') else {
        return line.to_owned();
    };
    if trimmed
        .as_bytes()
        .get(equal + 1)
        .is_some_and(|next| matches!(next, b'=' | b'>'))
    {
        return line.to_owned();
    }
    let name = trimmed[..equal].trim_end();
    if name.ends_with(['!', '<', '>']) {
        return line.to_owned();
    }
    let known_global = matches!(name, "result" | "url" | "key" | "baseUrl" | "page" | "java");
    let identifier = !name.is_empty()
        && name.chars().enumerate().all(|(index, character)| {
            character == '_'
                || character == '$'
                || character.is_ascii_alphanumeric() && (index > 0 || !character.is_ascii_digit())
        });
    if !known_global && identifier {
        let indent = &line[..line.len() - trimmed.len()];
        format!("{indent}var {trimmed}")
    } else {
        line.to_owned()
    }
}
