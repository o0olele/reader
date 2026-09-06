//! Shared Java pattern normalization. Escaped metacharacters stay literal.

pub(crate) fn normalize_java_regex(pattern: &str) -> String {
    let mut output = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        let Some(next) = chars.next() else {
            output.push('\\');
            break;
        };
        if next == 'Q' {
            let mut literal = String::new();
            while let Some(quoted) = chars.next() {
                if quoted == '\\' && chars.peek() == Some(&'E') {
                    chars.next();
                    break;
                }
                literal.push(quoted);
            }
            output.push_str(&regex::escape(&literal));
        } else {
            if next.is_ascii_alphanumeric() || "\\.*+?()[]{}^$|#&~-".contains(next) {
                output.push('\\');
            }
            output.push(next);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_escaped_metacharacters_and_java_quoted_literals() {
        let regex =
            regex::Regex::new(&normalize_java_regex(r"\（\Q[a.b]+\E\）|\(x\)|a\.b")).unwrap();
        assert!(regex.is_match("（[a.b]+）"));
        assert!(regex.is_match("(x)"));
        assert!(!regex.is_match("axb"));
        assert!(!regex.is_match("x"));
    }
}
