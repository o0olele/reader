use serde_json::Value;

const RULE_OBJECTS: &[&str] = &[
    "ruleSearch",
    "ruleBookInfo",
    "ruleToc",
    "ruleContent",
    "ruleExplore",
];
const RULE_STRINGS: &[&str] = &[
    "searchUrl",
    "exploreUrl",
    "loginUrl",
    "loginCheckJs",
    "coverDecodeJs",
    "jsLib",
    "header",
];
pub(super) const TOKENS: &[(&str, &str)] = &[
    ("exclude !n", r"![0-9]"),
    ("range .a:b", r"\.-?[0-9]+:-?[0-9]+"),
    ("@@ force JSoup", r"@@"),
    ("JSONPath recursive ..", r"\$\.\."),
    ("JSONPath filter ?()", r"\?\("),
    ("XPath", r"(^|\|\||&&|%%)\s*//"),
    (
        "url option ,{...}",
        r#",\s*\{[\s\S]*["'](method|body|charset|headers|webView|js|type|retry|origin|bodyJs|webJs)["']"#,
    ),
    ("java.* call", r"java\.[A-Za-z0-9_]+"),
    ("@js:", r"@js:"),
    ("<js>", r"<js>"),
    ("{{ template }}", r"\{\{"),
    ("@get:", r"@get:"),
    ("@put:", r"@put:"),
    ("## replace", r"##"),
    ("|| alternative", r"\|\|"),
    ("&& chain", r"&&"),
    ("%% cross-merge", r"%%"),
];

pub(super) fn source_is_json(source: &Value, rules: &[(String, String)]) -> bool {
    if rules.iter().any(|(_, raw)| {
        let l = raw.to_ascii_lowercase();
        l.starts_with("@json:") || raw.contains("$.") || raw.contains("$[")
    }) {
        return true;
    }
    serde_json::to_string(source)
        .map(|s| {
            let l = s.to_ascii_lowercase();
            l.contains("@json:") || l.contains("/api/") || l.contains("json")
        })
        .unwrap_or(false)
}

pub(super) fn error_category(error: &str) -> &'static str {
    let l = error.to_ascii_lowercase();
    if l.contains("rule is empty") {
        "empty rule"
    } else if l.contains("unclosed quote") {
        "unclosed quote"
    } else if l.contains("empty branch") || l.contains("unclosed balanced") {
        "empty branch"
    } else if l.contains("regex") || l.contains("escape sequence") {
        "regex compatibility"
    } else if l.contains("template") {
        "template delimiter"
    } else if l.contains("xpath") || l.contains("jsonpath") || l.contains("json path") {
        "path parser"
    } else if l.contains("javaimporter")
        || l.contains("java.lang")
        || l.contains("java.util")
        || l.contains("java.io")
        || l.contains("java.security")
    {
        "unsupported JVM access"
    } else if l.contains("source error") || l.contains("javascript") || l.contains("quickjs") {
        "js runtime"
    } else if l.contains("default-mode rule is not supported")
        && (l.contains("not a css selector") || l.contains("emptyselector"))
    {
        "css compatibility"
    } else if l.contains("cannot read property")
        || l.contains("cannot read properties")
        || l.contains("is not defined")
    {
        "harness input"
    } else {
        "other"
    }
}

fn walk(value: &Value, path: &str, out: &mut Vec<(String, String)>) {
    match value {
        Value::String(text) if !text.trim().is_empty() => {
            out.push((path.to_owned(), text.to_owned()))
        }
        Value::Object(map) => {
            for (key, child) in map {
                walk(child, &format!("{path}.{key}"), out);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                walk(child, &format!("{path}[{index}]"), out);
            }
        }
        _ => {}
    }
}

pub(super) fn source_rules(source: &Value) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    for key in RULE_OBJECTS {
        if let Some(value) = source.get(*key) {
            if let Value::String(raw) = value {
                if let Ok(parsed) = serde_json::from_str(raw) {
                    walk(&parsed, key, &mut rules);
                } else {
                    walk(value, key, &mut rules);
                }
            } else {
                walk(value, key, &mut rules);
            }
        }
    }
    for key in RULE_STRINGS {
        if let Some(value) = source.get(*key) {
            walk(value, key, &mut rules);
        }
    }
    rules
}

pub(super) fn is_metadata_url(path: &str, raw: &str) -> bool {
    let key = path
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    (key.ends_with("url") || key == "url")
        && (raw.trim_start().starts_with("http://") || raw.trim_start().starts_with("https://"))
        && !raw.contains("@js:")
        && !raw.contains("@json:")
        && !raw.contains("@xpath:")
}
