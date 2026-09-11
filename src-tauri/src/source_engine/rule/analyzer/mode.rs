//! Legado rule dialect detection.
//!
//! A rule value picks its own evaluation mode before anything else runs:
//! explicit `@xpath:` / `@json:` / `@css:` prefixes, `@@` (force JSoup),
//! `##` / `:` (regex), JSON paths, XPath, and finally the default JSoup dialect.

use super::super::model::RuleMode;
use super::super::scanner::starts_ignore_ascii_case;

pub(super) fn detect_mode(raw: &str, json_input: bool) -> (RuleMode, &str) {
    // A leading `-` reverses the result order. Exported sources also carry a
    // leading `+` (legado's list marker, for example `+@css:.bookbox`); the
    // single-rule evaluation does not need it, and dropping it keeps dialect
    // detection intact instead of handing `+@css:` to the CSS parser.
    let candidate = raw
        .strip_prefix('-')
        .or_else(|| raw.strip_prefix('+'))
        .map(str::trim_start)
        .unwrap_or(raw);
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
    } else if candidate.starts_with("$.")
        || candidate.starts_with("$[")
        || (json_input && looks_like_legacy_json_path(candidate))
        || looks_like_json_template(candidate)
    {
        (RuleMode::Json, candidate)
    } else if candidate.starts_with('/') {
        (RuleMode::XPath, candidate)
    } else {
        (RuleMode::Default, candidate)
    }
}

fn looks_like_legacy_json_path(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && !value.contains("{{")
        && !value.contains('@')
        && !value.contains(' ')
        && !value.contains(':')
        && (value.contains("[*]") || value.contains("[-") || value.split('.').count() >= 2)
        && value
            .chars()
            .all(|c| c.is_alphanumeric() || ".[]*_-'\"".contains(c))
}

/// Single-brace `{$.path}` is legado's JSON display-template spelling (for
/// example `{$.score}分` or `连载中{$.status}已完结`). It is not a CSS selector;
/// routing it to Json mode lets `jsonpath::template` render it.
///
/// The double-brace spelling `{{$.path}}` belongs to the inline-template path
/// and must keep its own mode.
fn looks_like_json_template(value: &str) -> bool {
    let mut index = 0;
    while let Some(offset) = value[index..].find('{') {
        let start = index + offset;
        if value[start..].starts_with("{{") {
            index = start + 2;
            continue;
        }
        let rest = &value[start + 1..];
        if rest.starts_with("$.") || rest.starts_with("$[") {
            return true;
        }
        index = start + 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(raw: &str) -> (RuleMode, String) {
        let (mode, rule) = detect_mode(raw, false);
        (mode, rule.to_owned())
    }

    #[test]
    fn strips_the_legado_list_marker_without_losing_the_dialect() {
        assert_eq!(mode("+@css:.bookbox"), (RuleMode::Default, ".bookbox".into()));
        assert_eq!(
            mode("-tag.a@text"),
            (RuleMode::Default, "tag.a@text".into())
        );
        // `@js:` never reaches this function: `parse_embedded_rules` splits it
        // off first, so only the marker is left for the rule parser.
        assert_eq!(mode("+"), (RuleMode::Default, String::new()));
    }

    #[test]
    fn routes_single_brace_json_templates_to_json_mode() {
        for raw in ["{$.score}分", "连载中{$.status}已完结", "{$[0].name}"] {
            assert_eq!(mode(raw).0, RuleMode::Json, "{raw}");
        }
        // The double-brace spelling keeps whatever mode it would otherwise have.
        assert_eq!(mode("书名：{{$.name}}").0, RuleMode::Default);
        // A CSS rule that merely mentions a brace is untouched.
        assert_eq!(mode("a[href^=http]").0, RuleMode::Default);
    }

    #[test]
    fn keeps_a_leading_slash_as_xpath() {
        // Legado's documented syntax; `rules/legado_rules.jsonl` pins it.
        // Relative URLs are a separate path (`rule::evaluate_url`).
        for raw in [
            "//div[@class='item']",
            "//a/@href",
            "/html/body/div",
            "@XPath://a",
            "/search?key={{key}}&page={{page}}",
        ] {
            assert_eq!(mode(raw).0, RuleMode::XPath, "{raw}");
        }
    }
}
