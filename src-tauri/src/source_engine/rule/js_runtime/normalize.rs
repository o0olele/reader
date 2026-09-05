use super::implicit::{declare_implicit_assignment, declare_implicit_assignments};

/// Legado exports frequently omit semicolons between assignment statements,
/// relying on Android's JavaScript engine to insert them at line boundaries.
/// QuickJS is stricter for a few otherwise-valid continuations (notably
/// `headerSign=...` followed by `paramSign=...`). Add terminators only where a
/// new declaration/assignment clearly starts, leaving multiline expressions,
/// control headers, and object/function bodies untouched.
pub(super) fn normalize_js_statement_boundaries(script: &str) -> String {
    // Rhino permits redeclaring `let`/`const` in separately evaluated source
    // fragments. QuickJS rejects those lexical redeclarations even in sloppy
    // mode, so use function-scoped `var` for legado compatibility.
    let script = relax_lexical_declarations(script);
    let script = declare_implicit_assignments(&script);
    let mut lines: Vec<String> = script.lines().map(declare_implicit_assignment).collect();
    if lines.len() < 2 {
        return script;
    }
    for index in 1..lines.len() {
        if should_insert_statement_semicolon(&lines[index - 1], &lines[index]) {
            let previous = lines[index - 1].trim_end();
            if !previous.ends_with(';') {
                lines[index - 1].push(';');
            }
        }
    }
    lines.join("\n")
}

fn relax_lexical_declarations(script: &str) -> String {
    let mut output = String::with_capacity(script.len());
    let mut cursor = 0;
    for (index, _) in script.match_indices(|c: char| c.is_ascii_alphabetic()) {
        if index < cursor {
            continue;
        }
        output.push_str(&script[cursor..index]);
        let rest = &script[index..];
        let replacement = if rest.starts_with("let")
            && rest[3..].chars().next().is_some_and(|c| c.is_ascii_whitespace())
        {
            Some(("var", 3))
        } else if rest.starts_with("const")
            && rest[5..].chars().next().is_some_and(|c| c.is_ascii_whitespace())
        {
            Some(("var", 5))
        } else {
            None
        };
        if let Some((word, length)) = replacement {
            output.push_str(word);
            cursor = index + length;
        } else {
            output.push_str(&script[index..index + 1]);
            cursor = index + 1;
        }
    }
    output.push_str(&script[cursor..]);
    output
}

fn should_insert_statement_semicolon(previous: &str, current: &str) -> bool {
    let previous = previous.trim();
    let current = current.trim_start();
    if previous.is_empty()
        || previous.starts_with("//")
        || previous.ends_with([
            '{', '(', '[', ',', '.', ':', '?', '=', '+', '-', '*', '/', '&', '|',
        ])
        || ["else", "catch", "finally"]
            .iter()
            .any(|keyword| current.starts_with(keyword))
    {
        return false;
    }
    if is_control_header(previous) {
        return false;
    }
    starts_assignment_or_declaration(current)
        || matches!(current.chars().next(), Some('\'' | '"' | '`'))
}

fn is_control_header(line: &str) -> bool {
    ["if", "for", "while", "switch", "with", "catch"]
        .iter()
        .any(|keyword| {
            line.strip_prefix(keyword)
                .is_some_and(|rest| rest.trim_start().starts_with('('))
        })
}

fn starts_assignment_or_declaration(line: &str) -> bool {
    let line = line.trim_start();
    if ["var ", "let ", "const "]
        .iter()
        .any(|prefix| line.starts_with(prefix))
    {
        return true;
    }
    let Some(equal) = line.find('=') else {
        return false;
    };
    let lhs = line[..equal].trim_end();
    if lhs.is_empty() || lhs.ends_with(['=', '!', '<', '>']) {
        return false;
    }
    lhs.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '$' | '.' | '[' | ']' | '\'' | '"')
            || character.is_ascii_whitespace()
    })
}
