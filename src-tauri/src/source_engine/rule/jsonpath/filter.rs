use crate::source_engine::rule::RuleExecutionError;
use serde_json::Value;
use std::cmp::Ordering;
#[derive(Debug)]
pub(super) enum FilterExpr {
    Truthy(String),
    Compare { path: String, op: Op, value: Lit },
    And(Vec<FilterExpr>),
    Or(Vec<FilterExpr>),
}
#[derive(Clone, Copy, Debug)]
pub(super) enum Op {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}
#[derive(Debug)]
pub(super) enum Lit {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}
pub(super) fn parse_filter(raw: &str) -> Result<FilterExpr, RuleExecutionError> {
    let raw = trim(raw.trim());
    let p = split(raw, "||");
    if p.len() > 1 {
        return Ok(FilterExpr::Or(
            p.into_iter().map(parse_filter).collect::<Result<_, _>>()?,
        ));
    }
    let p = split(raw, "&&");
    if p.len() > 1 {
        return Ok(FilterExpr::And(
            p.into_iter().map(parse_filter).collect::<Result<_, _>>()?,
        ));
    }
    for (n, o) in [
        ("!=", Op::Ne),
        (">=", Op::Ge),
        ("<=", Op::Le),
        ("==", Op::Eq),
        (">", Op::Gt),
        ("<", Op::Lt),
    ] {
        if let Some(i) = find(raw, n) {
            let l = raw[..i].trim();
            if !l.starts_with('@') {
                return Err(invalid("filter left side must start with `@`"));
            }
            return Ok(FilterExpr::Compare {
                path: l.into(),
                op: o,
                value: literal(raw[i + n.len()..].trim())?,
            });
        }
    }
    raw.starts_with('@')
        .then(|| FilterExpr::Truthy(raw.into()))
        .ok_or_else(|| invalid("unsupported filter expression"))
}
pub(super) fn eval_filter(e: &FilterExpr, v: &Value) -> bool {
    match e {
        FilterExpr::Truthy(p) => at(v, p).is_some_and(truthy),
        FilterExpr::And(x) => x.iter().all(|e| eval_filter(e, v)),
        FilterExpr::Or(x) => x.iter().any(|e| eval_filter(e, v)),
        FilterExpr::Compare { path, op, value } => at(v, path).is_some_and(|a| cmp(a, *op, value)),
    }
}
fn at<'a>(v: &'a Value, p: &str) -> Option<&'a Value> {
    let mut v = v;
    let p = p.strip_prefix('@')?;
    if p.is_empty() {
        return Some(v);
    }
    for s in p.trim_start_matches('.').split('.') {
        v = v.get(s)?
    }
    Some(v)
}
fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(x) => *x,
        Value::Number(x) => x.as_f64().is_some_and(|n| n != 0.),
        Value::String(x) => !x.is_empty(),
        Value::Array(x) => !x.is_empty(),
        Value::Object(x) => !x.is_empty(),
    }
}
fn cmp(a: &Value, o: Op, b: &Lit) -> bool {
    let x = match b {
        Lit::Number(n) => a.as_f64().and_then(|x| x.partial_cmp(n)),
        Lit::String(s) => a.as_str().map(|x| x.cmp(s)),
        Lit::Bool(expected) => a.as_bool().map(|actual| actual.cmp(expected)),
        Lit::Null => a.is_null().then_some(Ordering::Equal),
    };
    match o {
        Op::Eq => x == Some(Ordering::Equal),
        Op::Ne => x != Some(Ordering::Equal),
        Op::Gt => x == Some(Ordering::Greater),
        Op::Ge => matches!(x, Some(Ordering::Greater | Ordering::Equal)),
        Op::Lt => x == Some(Ordering::Less),
        Op::Le => matches!(x, Some(Ordering::Less | Ordering::Equal)),
    }
}
fn literal(x: &str) -> Result<Lit, RuleExecutionError> {
    if x.len() >= 2
        && ((x.starts_with('\'') && x.ends_with('\'')) || (x.starts_with('"') && x.ends_with('"')))
    {
        return Ok(Lit::String(x[1..x.len() - 1].into()));
    }
    match x {
        "true" => Ok(Lit::Bool(true)),
        "false" => Ok(Lit::Bool(false)),
        "null" => Ok(Lit::Null),
        _ => x
            .parse()
            .map(Lit::Number)
            .map_err(|_| invalid("invalid filter literal")),
    }
}
fn trim(mut x: &str) -> &str {
    loop {
        if !x.starts_with('(') || !x.ends_with(')') {
            return x;
        }
        let mut d = 0;
        for (i, c) in x.char_indices() {
            if c == '(' {
                d += 1
            } else if c == ')' {
                d -= 1;
                if d == 0 && i == x.len() - 1 {
                    x = &x[1..x.len() - 1];
                    break;
                } else if d == 0 {
                    return x;
                }
            }
        }
    }
}
fn find(x: &str, n: &str) -> Option<usize> {
    positions(x, n).into_iter().next()
}
fn split<'a>(x: &'a str, n: &str) -> Vec<&'a str> {
    let p = positions(x, n);
    if p.is_empty() {
        return vec![x.trim()];
    }
    let mut o = Vec::new();
    let mut s = 0;
    for i in p {
        o.push(x[s..i].trim());
        s = i + n.len()
    }
    o.push(x[s..].trim());
    o
}
fn positions(x: &str, n: &str) -> Vec<usize> {
    let mut o = Vec::new();
    let mut d: usize = 0;
    let mut q = None;
    let mut i = 0;
    while i < x.len() {
        let t = &x[i..];
        let c = t.chars().next().unwrap();
        if let Some(a) = q {
            if c == a && !x[..i].ends_with('\\') {
                q = None
            }
        } else if c == '\'' || c == '"' {
            q = Some(c)
        } else if c == '(' {
            d += 1
        } else if c == ')' {
            d = d.saturating_sub(1)
        } else if d == 0 && t.starts_with(n) {
            o.push(i);
            i += n.len();
            continue;
        }
        i += c.len_utf8()
    }
    o
}
fn invalid(m: impl Into<String>) -> RuleExecutionError {
    RuleExecutionError::InvalidJsonPath(m.into())
}
