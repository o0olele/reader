use super::{push_json_values, select};
use crate::source_engine::rule::RuleExecutionError;
use serde_json::Value;

/// Render Legado's `{$.path}` templates without scanning the selected JSON
/// values as template syntax. Braces inside quoted keys/filters are literals.
pub(super) fn render(root: &Value, raw: &str) -> Result<Option<String>, RuleExecutionError> {
    let mut output = String::new();
    let mut cursor = 0;
    let mut rendered = false;
    while let Some(offset) = raw[cursor..].find("{$.") {
        let start = cursor + offset;
        let Some(end) = closing_brace(raw, start) else {
            break;
        };
        output.push_str(&raw[cursor..start]);
        let mut values = Vec::new();
        for value in select(root, &raw[start + 1..end])? {
            push_json_values(value, &mut values);
        }
        output.push_str(&values.join("\n"));
        cursor = end + 1;
        rendered = true;
    }
    if !rendered {
        return Ok(None);
    }
    output.push_str(&raw[cursor..]);
    Ok(Some(output))
}

fn closing_brace(raw: &str, start: usize) -> Option<usize> {
    let mut quote = None;
    let mut escaped = false;
    let mut depth = 0;
    for (offset, ch) in raw[start..].char_indices() {
        if escaped {
            escaped = false;
        } else if ch == '\\' && quote.is_some() {
            escaped = true;
        } else if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
        } else if matches!(ch, '\'' | '"') {
            quote = Some(ch);
        } else if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(start + offset);
            }
        }
    }
    None
}
