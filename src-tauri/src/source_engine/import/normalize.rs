//! Legado selector normalization for the CSS-compatible flat projection.

/// Normalize the selector spellings used by Legado's JSON exports.
///
/// Besides `@css:` Legado commonly emits `css:` and uses `@href`/`@src`
/// suffixes instead of the explicit `::attr(...)` form used internally.
pub(crate) fn normalize_rule(value: &str) -> String {
    let value = value.split("&&").next().unwrap_or(value).trim();
    let value = value
        .strip_prefix("@css:")
        .or_else(|| value.strip_prefix("css:"))
        .map(str::trim)
        .unwrap_or(value);
    if value.eq_ignore_ascii_case("text") || value.eq_ignore_ascii_case("@text") {
        return "*".into();
    }
    // Older imported rows may contain a single private step without an `@`
    // chain (for example `class.book` or `tag.a`). Project those too so the
    // CSS fallback remains usable when raw legado objects were not persisted.
    if !value.contains('@') && is_private_selector_step(value) {
        return private_selector(value);
    }
    if is_legado_chain(value)
        && !value.starts_with("@xpath:")
        && !value.starts_with("@json:")
        && !value.starts_with("@js:")
        && !value.starts_with("@css:")
    {
        let mut selector = String::new();
        let mut terminal = None;
        let starts_with_at = value.starts_with('@');
        for (index, step) in value.split('@').filter(|step| !step.is_empty()).enumerate() {
            let step = step.trim();
            // `@tag.foo`, `@class.foo`, and `@id.foo` are Legado's private
            // selector-chain operators.  Other `@name` suffixes (for example
            // `@href`, `@src`, or `@content`) are ordinary attribute terminals
            // and must not become descendant selectors.
            if index > 0 && matches!(step, "text" | "textNodes" | "ownText" | "html" | "all") {
                terminal = Some(step);
                break;
            }
            if let Some(attribute) = step.strip_prefix("attr(").and_then(|v| v.strip_suffix(')')) {
                terminal = Some(attribute.trim());
                break;
            }
            if !is_private_selector_step(step)
                && step != "tag"
                && (index > 0 || (index == 0 && starts_with_at))
            {
                terminal = Some(step);
                break;
            }
            let mapped = private_selector(step);
            if mapped == "*" && step == "tag" {
                continue;
            }
            if !selector.is_empty() {
                selector.push(' ');
            }
            selector.push_str(&mapped);
        }
        if !selector.is_empty() || terminal.is_some() {
            let selector = if selector.is_empty() {
                "*"
            } else {
                selector.as_str()
            };
            return match terminal {
                Some("text") | Some("textNodes") | Some("ownText") | Some("html") | Some("all") => {
                    selector.to_owned()
                }
                Some(attribute) => format!("{selector}::attr({attribute})"),
                None => selector.to_owned(),
            };
        }
    }
    let value = normalize_attr_pseudo(value);
    if let Some(attribute) = value.strip_prefix('@') {
        if !attribute.is_empty()
            && attribute
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return format!("*::attr({attribute})");
        }
    }
    if !value.contains("::attr(") {
        if let Some((selector, attribute)) = value.rsplit_once('@') {
            if !selector.is_empty()
                && !attribute.is_empty()
                && attribute
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                if attribute.eq_ignore_ascii_case("text")
                    || attribute.eq_ignore_ascii_case("textnodes")
                {
                    return selector.trim().to_owned();
                }
                return format!("{}::attr({attribute})", selector.trim());
            }
        }
    }
    value.to_owned()
}

fn normalize_attr_pseudo(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut rest = value;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("::attr(") {
            output.push_str("::attr(");
            rest = after;
        } else if let Some(after) = rest.strip_prefix(":attr(") {
            output.push_str("::attr(");
            rest = after;
        } else {
            let length = rest.chars().next().map(char::len_utf8).unwrap_or(1);
            output.push_str(&rest[..length]);
            rest = &rest[length..];
        }
    }
    output
}

fn is_legado_chain(value: &str) -> bool {
    if !value.contains('@') {
        return false;
    }
    let parts = value.split('@').map(str::trim).collect::<Vec<_>>();
    value.starts_with('@')
        || parts.iter().any(|part| is_private_selector_step(part))
        || parts.iter().any(|part| {
            matches!(*part, "text" | "textNodes" | "ownText" | "html" | "all")
                || part
                    .strip_prefix("attr(")
                    .and_then(|value| value.strip_suffix(')'))
                    .is_some_and(|name| !name.trim().is_empty())
        })
        || parts.last().is_some_and(|part| is_attribute_name(part))
}

fn is_attribute_name(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

fn private_selector(step: &str) -> String {
    let step = step.split_once('!').map_or(step, |(prefix, _)| prefix);
    let mut parts = step.split('.');
    let kind = parts.next().unwrap_or_default();
    let name = parts.next().unwrap_or_default();
    match kind {
        "class" if !name.is_empty() => format!(".{name}"),
        "id" if !name.is_empty() => format!("#{name}"),
        "tag" if !name.is_empty() => name.to_owned(),
        _ => step
            .split_once('!')
            .map_or_else(|| step.to_owned(), |(prefix, _)| prefix.to_owned()),
    }
}

fn is_private_selector_step(step: &str) -> bool {
    let step = step.split_once('!').map_or(step, |(prefix, _)| prefix);
    let mut parts = step.split('.');
    matches!(parts.next(), Some("class" | "id" | "tag")) && parts.next().is_some()
}

#[cfg(test)]
mod tests {
    use super::normalize_rule;

    #[test]
    fn does_not_rewrite_at_signs_inside_css_values_or_js_rules() {
        assert_eq!(
            normalize_rule(r#"a[href="mailto:a@b.example"]"#),
            r#"a[href="mailto:a@b.example"]"#
        );
        assert_eq!(
            normalize_rule("@js:result + '@href'"),
            "@js:result + '@href'"
        );
    }
}
