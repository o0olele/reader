use super::filter::{parse_filter, FilterExpr};
use crate::source_engine::rule::RuleExecutionError;
#[derive(Debug)]
pub(super) enum Token {
    Key(String),
    RecursiveKey(String),
    Wildcard,
    RecursiveWildcard,
    Index(isize),
    Filter(FilterExpr),
}
pub(super) fn parse_path(path: &str) -> Result<Vec<Token>, RuleExecutionError> {
    let path = path.trim();
    if !path.starts_with('$') {
        return Err(invalid("path must start with `$`"));
    };
    let mut out = Vec::new();
    let mut i = 1;
    while i < path.len() {
        let b = path.as_bytes();
        if b[i] == b'.' {
            let rec = i + 1 < path.len() && b[i + 1] == b'.';
            i += if rec { 2 } else { 1 };
            if i >= path.len() {
                return Err(invalid("missing object key"));
            };
            if !rec && path.as_bytes()[i] == b'[' {
                continue;
            }
            let s = i;
            while i < path.len() && path.as_bytes()[i] != b'.' && path.as_bytes()[i] != b'[' {
                i += 1
            }
            if s == i {
                return Err(invalid("missing object key"));
            };
            let k = &path[s..i];
            out.push(match (rec, k) {
                (true, "*") => Token::RecursiveWildcard,
                (false, "*") => Token::Wildcard,
                (true, _) => Token::RecursiveKey(k.into()),
                (false, _) => Token::Key(k.into()),
            });
        } else if b[i] == b'[' {
            let (inside, n) = bracket(path, i)?;
            let t = inside.trim();
            out.push(if t == "*" {
                Token::Wildcard
            } else if let Some(e) = t.strip_prefix("?(").and_then(|x| x.strip_suffix(')')) {
                Token::Filter(parse_filter(e)?)
            } else if (t.starts_with('\'') && t.ends_with('\''))
                || (t.starts_with('"') && t.ends_with('"'))
            {
                Token::Key(t[1..t.len() - 1].into())
            } else if let Ok(n) = t.parse() {
                Token::Index(n)
            } else {
                return Err(invalid(format!("unsupported bracket `{t}`")));
            });
            i = n;
        } else {
            return Err(invalid(format!("unexpected character at {i}")));
        }
    }
    Ok(out)
}
fn bracket(path: &str, start: usize) -> Result<(&str, usize), RuleExecutionError> {
    let mut q = None;
    let mut d = 0;
    for (o, c) in path[start + 1..].char_indices() {
        let i = start + 1 + o;
        if let Some(a) = q {
            if c == a && !path[..i].ends_with('\\') {
                q = None
            }
        } else if c == '\'' || c == '"' {
            q = Some(c)
        } else if c == '(' {
            d += 1
        } else if c == ')' && d > 0 {
            d -= 1
        } else if c == ']' && d == 0 {
            return Ok((&path[start + 1..i], i + 1));
        }
    }
    Err(invalid("unclosed bracket"))
}
fn invalid(m: impl Into<String>) -> RuleExecutionError {
    RuleExecutionError::InvalidJsonPath(m.into())
}
