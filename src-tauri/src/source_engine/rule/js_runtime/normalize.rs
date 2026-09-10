use super::implicit::declare_implicit_assignments;

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
    let raw_lines: Vec<String> = script.lines().map(str::to_owned).collect();
    if raw_lines.len() < 2 {
        return script;
    }
    let mut lines = split_comma_declarations(&raw_lines);
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

/// Rewrites `lines` so a declaration whose comma-continuation starts on the
/// next line is terminated before that continuation.
///
/// legado exports are routinely pretty-printed one declaration per line, which
/// splits `var list = [],` / `d = java.getElement(...)` across two statements.
/// Appending `;` to the first line (as the plain semicolon pass does) leaves
/// `var list = [],;` and QuickJS aborts the whole script with "variable name
/// expected" — an error that has nothing to do with the rule's intent. The
/// continuation is emitted as its own declaration, carrying the original
/// keyword so `var` scoping is preserved.
fn split_comma_declarations(lines: &[String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].trim_end();
        let continuation = declaration_keyword(line)
            .filter(|_| is_unfinished_declaration(line))
            .and_then(|keyword| {
                let next = lines.get(index + 1)?.trim();
                (!next.is_empty() && starts_assignment_or_declaration(next))
                    .then_some((keyword, next))
            });
        let terminated = match continuation {
            Some((keyword, next)) => {
                // An earlier pass may already have prefixed the continuation
                // with its own declaration (`var d = ...`); keep exactly one.
                let next = declaration_keyword(next)
                    .map(|existing| next[existing.len()..].trim_start())
                    .unwrap_or(next);
                let next = next.trim_start();
                // The continuation is emitted as its own declaration next, so
                // skip the line it came from.
                index += 1;
                Some(format!("{keyword} {next}"))
            }
            None => None,
        };
        output.push(match &terminated {
            Some(_) => format!("{};", line[..line.len() - 1].trim_end()),
            None => line.to_owned(),
        });
        if let Some(continuation) = terminated {
            output.push(continuation);
        }
        index += 1;
    }
    output
}

fn declaration_keyword(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_start();
    ["var ", "let ", "const "]
        .into_iter()
        .find(|prefix| trimmed.starts_with(prefix))
}

/// Whether a line that ends with `,` is an unfinished declaration.
///
/// Only the shapes that actually appear in legado exports matter here: a
/// declaration keyword, a `=` assignment, and the comma at the end. Anything
/// whose head ends in `)` or `}` after the comma is the tail of a multi-line
/// call or literal (`foo(a,`) whose continuation belongs to the same statement,
/// so it is left alone. A closing `]` is stripped first, because `var a = [],`
/// *is* a finished array followed by a continued declaration — the exact corpus
/// shape this handles — and a single line cannot tell the two apart by balance.
fn is_unfinished_declaration(line: &str) -> bool {
    let Some(head) = line.strip_suffix(',') else {
        return false;
    };
    let head = head.trim_end();
    let head = head.strip_suffix(']').unwrap_or(head).trim_end();
    if head.ends_with([')', '}']) || declaration_keyword(head).is_none() {
        return false;
    }
    let Some(equal) = head.find('=') else {
        return false;
    };
    let before = head.as_bytes().get(equal + 1).copied();
    !matches!(before, Some(b'=') | Some(b'>'))
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
            && rest[3..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_whitespace())
        {
            Some(("var", 3))
        } else if rest.starts_with("const")
            && rest[5..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_whitespace())
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

fn should_insert_statement_semicolon(previous: &str, current: &str) -> bool {    let previous = previous.trim();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminates_a_declaration_split_by_a_comma() {
        let lines = vec!["var list = [],".to_owned(), "d = f()".to_owned()];
        let split = split_comma_declarations(&lines);
        assert_eq!(split[0], "var list = [];");
        assert_eq!(split[1].split_whitespace().collect::<Vec<_>>(), ["var", "d", "=", "f()"]);
        let normalized =
            normalize_js_statement_boundaries("var list = [],\nd = java.getElement('.x')\nlist");
        assert!(
            normalized.contains("var list = [];"),
            "{normalized}"
        );
        assert!(
            normalized.contains("java.getElement('.x')"),
            "{normalized}"
        );
        assert!(!normalized.contains("[],;"), "{normalized}");
    }

    #[test]
    fn recognizes_an_unfinished_declaration_shape() {
        assert!(is_unfinished_declaration("var list = [],"));
        assert!(is_unfinished_declaration("var a = 1,"));
        assert!(is_unfinished_declaration("var tags = [1,"));
        assert!(!is_unfinished_declaration("var list = [];"));
        assert!(!is_unfinished_declaration("var list = []"));
        assert!(!is_unfinished_declaration("foo(bar,"));
        assert!(!is_unfinished_declaration("var a == 1,"));
        assert!(!is_unfinished_declaration("call(other),"));
        assert!(!is_unfinished_declaration("var obj = {a: 1},"));
    }

    #[test]
    fn keeps_multiline_array_literals_on_one_statement() {
        let normalized = normalize_js_statement_boundaries("var tags = [\n  'a',\n  'b'\n]\ntags");
        assert!(!normalized.contains("var tags = [;"), "{normalized}");
        assert!(normalized.contains("var tags = [\n  'a',\n  'b'\n]"), "{normalized}");
    }

    #[test]
    fn leaves_a_complete_declaration_untouched() {
        let normalized = normalize_js_statement_boundaries("var list = [];\nlist");
        assert_eq!(normalized, "var list = [];\nlist");
    }
}
