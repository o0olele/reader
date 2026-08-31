use super::directive::{extract_get, extract_put, extract_replacement, extract_templates};
use super::model::{RuleAlternatives, RuleContext, RuleJoin, RuleMode, RuleParseError, SourceRule};
use super::scanner::{
    find_ignore_ascii_case, split_top_level, starts_ignore_ascii_case, Separator,
};

pub fn split_rule(raw: &str) -> Result<RuleAlternatives, RuleParseError> {
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    let pieces = split_top_level(raw)?;
    let mut alternatives = vec![Vec::new()];
    let mut next_join = RuleJoin::Chain;
    for (index, (piece, separator)) in pieces.iter().enumerate() {
        if piece.trim().is_empty() {
            let marker = separator
                .or_else(|| index.checked_sub(1).and_then(|i| pieces[i].1))
                .map(Separator::text)
                .unwrap_or("rule");
            return Err(RuleParseError::EmptyBranch(marker));
        }
        alternatives
            .last_mut()
            .unwrap()
            .extend(parse_embedded_rules(piece.trim(), next_join)?);
        match separator {
            Some(separator) if separator.is_alternative() => {
                alternatives.push(Vec::new());
                next_join = RuleJoin::Chain;
            }
            Some(separator) if separator.is_concat() => next_join = RuleJoin::Concat,
            Some(_) => next_join = RuleJoin::Interleave,
            None => {}
        }
    }
    Ok(alternatives)
}

pub fn expand_template(raw: &str, context: &RuleContext) -> String {
    let mut output = String::with_capacity(raw.len());
    let mut cursor = 0;
    while let Some(relative_start) = raw[cursor..].find("{{") {
        let start = cursor + relative_start;
        output.push_str(&raw[cursor..start]);
        let content_start = start + 2;
        let Some(relative_end) = raw[content_start..].find("}}") else {
            output.push_str(&raw[start..]);
            return output;
        };
        let end = content_start + relative_end;
        let expression = raw[content_start..end].trim();
        output.push_str(context.get(expression).unwrap_or(&raw[start..end + 2]));
        cursor = end + 2;
    }
    output.push_str(&raw[cursor..]);
    output
}

fn parse_embedded_rules(
    raw: &str,
    first_join: RuleJoin,
) -> Result<Vec<SourceRule>, RuleParseError> {
    let mut rules = Vec::new();
    let mut cursor = 0;
    let mut join = first_join;
    while cursor < raw.len() {
        let next = [
            find_ignore_ascii_case(raw, cursor, "<js>"),
            find_tail_js(raw, cursor),
        ]
        .into_iter()
        .flatten()
        .min();
        let Some(next) = next else {
            push_rule(&mut rules, &raw[cursor..], join)?;
            break;
        };
        if next > cursor {
            push_rule(&mut rules, &raw[cursor..next], join)?;
            join = RuleJoin::Chain;
        }
        if starts_ignore_ascii_case(raw, next, "<js>") {
            let script_start = next + 4;
            let close = find_ignore_ascii_case(raw, script_start, "</js>")
                .ok_or(RuleParseError::Unclosed("<js> block"))?;
            push_typed_rule(
                &mut rules,
                &raw[script_start..close],
                join,
                RuleMode::Js,
                false,
            )?;
            cursor = close + 5;
            join = RuleJoin::Chain;
        } else {
            let web = starts_ignore_ascii_case(raw, next, "@webjs:");
            let prefix_len = if web { 7 } else { 4 };
            push_typed_rule(
                &mut rules,
                &raw[next + prefix_len..],
                join,
                if web { RuleMode::WebJs } else { RuleMode::Js },
                false,
            )?;
            break;
        }
    }
    Ok(rules)
}

fn push_rule(
    output: &mut Vec<SourceRule>,
    raw: &str,
    join: RuleJoin,
) -> Result<(), RuleParseError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(());
    }
    let reverse = raw.starts_with('-');
    let (mode, cleaned) = detect_mode(raw);
    push_typed_rule(output, cleaned, join, mode, reverse)
}

fn push_typed_rule(
    output: &mut Vec<SourceRule>,
    raw: &str,
    join: RuleJoin,
    mode: RuleMode,
    reverse: bool,
) -> Result<(), RuleParseError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(());
    }
    let (put, without_put) = extract_put(raw)?;
    let (rule, replace) = extract_replacement(without_put.trim())?;
    let (get, rule) = extract_get(&rule);
    let templates = extract_templates(&rule);
    output.push(SourceRule {
        mode,
        rule,
        join,
        reverse,
        replace,
        put,
        get,
        templates,
    });
    Ok(())
}

fn detect_mode(raw: &str) -> (RuleMode, &str) {
    let candidate = raw.strip_prefix('-').map(str::trim_start).unwrap_or(raw);
    for (prefix, mode) in [
        ("@xpath:", RuleMode::XPath),
        ("@json:", RuleMode::Json),
        ("@css:", RuleMode::Default),
    ] {
        if starts_ignore_ascii_case(candidate, 0, prefix) {
            return (mode, &candidate[prefix.len()..]);
        }
    }
    if let Some(rule) = candidate.strip_prefix("@@") {
        (RuleMode::Default, rule)
    } else if candidate.starts_with("##") || candidate.starts_with(':') {
        (
            RuleMode::Regex,
            candidate.strip_prefix(':').unwrap_or(candidate),
        )
    } else if candidate.starts_with("$.") || candidate.starts_with("$[") {
        (RuleMode::Json, candidate)
    } else if candidate.starts_with('/') {
        (RuleMode::XPath, candidate)
    } else {
        (RuleMode::Default, candidate)
    }
}

fn find_tail_js(raw: &str, from: usize) -> Option<usize> {
    [
        find_ignore_ascii_case(raw, from, "@js:"),
        find_ignore_ascii_case(raw, from, "@webjs:"),
    ]
    .into_iter()
    .flatten()
    .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_alternatives_and_chains_without_touching_css_filters() {
        let parsed =
            split_rule(".book[data-x='a&&b']@text&&.author@text||@XPath://article").unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].len(), 2);
        assert_eq!(parsed[0][1].join, RuleJoin::Concat);
        assert_eq!(parsed[1][0].mode, RuleMode::XPath);
        assert_eq!(parsed[1][0].rule, "//article");
    }

    #[test]
    fn expands_known_template_and_preserves_unknown_expression() {
        let context = RuleContext::new([(String::from("page"), String::from("2"))]);
        assert_eq!(
            expand_template("/search?page={{page}}&key={{key}}", &context),
            "/search?page=2&key={{key}}"
        );
    }

    #[test]
    fn reports_unbalanced_groups_and_js_blocks() {
        assert_eq!(
            split_rule(".book[data-x='broken'").unwrap_err(),
            RuleParseError::Unclosed("balanced group")
        );
        assert_eq!(
            split_rule("<js>result").unwrap_err(),
            RuleParseError::Unclosed("<js> block")
        );
    }
}
