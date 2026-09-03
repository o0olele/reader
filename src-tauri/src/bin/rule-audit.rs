//! Static rule coverage audit for a Legado source corpus.
//!
//! Usage: `cargo run --bin rule-audit -- --corpus <file-or-dir> --out <dir>`
//! The audit deliberately executes the real rule splitter/evaluator against a
//! deterministic dummy document; it never performs network requests.

use reader_desktop_lib::source_engine::rule::{evaluate, Extraction, RuleContext};
use regex::Regex;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

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
const TOKENS: &[(&str, &str)] = &[
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

#[derive(Default)]
struct Audit {
    sources: usize,
    rules: usize,
    clean: usize,
    token_hits: BTreeMap<String, (usize, BTreeSet<usize>)>,
    errors: BTreeMap<String, usize>,
    java_methods: BTreeMap<String, BTreeSet<usize>>,
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

fn source_rules(source: &Value) -> Vec<(String, String)> {
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

fn corpus_file(path: &Path) -> Result<PathBuf, String> {
    if path.is_file() {
        return Ok(path.to_owned());
    }
    fs::read_dir(path)
        .map_err(|e| format!("cannot read corpus directory: {e}"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|candidate| candidate.extension().is_some_and(|ext| ext == "json"))
        .ok_or_else(|| format!("no JSON corpus found in {}", path.display()))
}

fn run(input: &str) -> Result<Audit, String> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| format!("invalid corpus JSON: {e}"))?;
    let sources = value.as_array().ok_or("corpus root must be a JSON array")?;
    let mut report = Audit {
        sources: sources.len(),
        ..Default::default()
    };
    let token_patterns = TOKENS
        .iter()
        .map(|(name, pattern)| (*name, Regex::new(pattern).expect("token regex")))
        .collect::<Vec<_>>();
    let java_pattern = Regex::new(r"java\.([A-Za-z0-9_]+)").expect("java method regex");
    for (source_id, source) in sources.iter().enumerate() {
        let rules = source_rules(source);
        let mut source_clean = true;
        for (path, raw) in rules {
            report.rules += 1;
            for (name, pattern) in &token_patterns {
                if pattern.is_match(&raw) {
                    let entry = report.token_hits.entry((*name).to_owned()).or_default();
                    entry.0 += 1;
                    entry.1.insert(source_id);
                }
            }
            for method in java_pattern.captures_iter(&raw).filter_map(|m| m.get(1)) {
                report
                    .java_methods
                    .entry(method.as_str().to_owned())
                    .or_default()
                    .insert(source_id);
            }
            // URL templates, headers and JS libraries are metadata rather than
            // evaluator rules; only actual rule fields are dry-run here.
            if path.starts_with("rule") {
                let normalized = raw.trim_start_matches('-').trim_start();
                let dummy_input = if normalized.starts_with("$.")
                    || normalized.starts_with("$[")
                    || normalized.to_ascii_lowercase().starts_with("@json:")
                {
                    "{}"
                } else {
                    "<html><body>audit</body></html>"
                };
                if let Err(error) = evaluate(
                    &raw,
                    dummy_input,
                    Extraction::Values,
                    &mut RuleContext::default(),
                ) {
                    source_clean = false;
                    *report.errors.entry(error.to_string()).or_default() += 1;
                }
            }
        }
        if source_clean {
            report.clean += 1;
        }
    }
    Ok(report)
}

fn markdown(report: &Audit) -> String {
    let percentage = |n: usize, d: usize| {
        if d == 0 {
            0.0
        } else {
            n as f64 * 100.0 / d as f64
        }
    };
    let mut out = format!("# Rule coverage audit\n\n- Sources: **{}**\n- Rule strings: **{}**\n- Fully executable: **{} / {} ({:.1}%)**\n- Blocked sources: **{}**\n\n", report.sources, report.rules, report.clean, report.sources, percentage(report.clean, report.sources), report.sources.saturating_sub(report.clean));
    out.push_str("## Syntax tokens\n\n| Token | Rule hits | Sources | % sources |\n| --- | ---: | ---: | ---: |\n");
    let mut tokens: Vec<_> = report.token_hits.iter().collect();
    tokens.sort_by_key(|(_, (_, sources))| std::cmp::Reverse(sources.len()));
    for (name, (hits, sources)) in tokens {
        out.push_str(&format!(
            "| `{name}` | {hits} | {} | {:.1}% |\n",
            sources.len(),
            percentage(sources.len(), report.sources)
        ));
    }
    out.push_str("\n## Execution errors\n\n| Error | Rule count |\n| --- | ---: |\n");
    for (error, count) in &report.errors {
        out.push_str(&format!(
            "| `{}` | {} |\n",
            error.replace('|', "\\|"),
            count
        ));
    }
    out.push_str("\n## java.* methods\n\n| Method | Sources |\n| --- | ---: |\n");
    let mut methods: Vec<_> = report.java_methods.iter().collect();
    methods.sort_by_key(|(_, sources)| std::cmp::Reverse(sources.len()));
    for (method, sources) in methods {
        out.push_str(&format!("| `java.{method}` | {} |\n", sources.len()));
    }
    out
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let value = |flag: &str| {
        args.windows(2)
            .find(|pair| pair[0] == flag)
            .map(|pair| pair[1].clone())
            .ok_or_else(|| format!("missing {flag}"))
    };
    let corpus = corpus_file(Path::new(&value("--corpus")?))?;
    let out_dir = PathBuf::from(value("--out")?);
    let report =
        run(&fs::read_to_string(&corpus).map_err(|e| format!("cannot read corpus: {e}"))?)?;
    fs::create_dir_all(&out_dir).map_err(|e| format!("cannot create output directory: {e}"))?;
    let output = out_dir.join("rule-audit.md");
    fs::write(&output, markdown(&report)).map_err(|e| format!("cannot write report: {e}"))?;
    println!("{}", output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audits_nested_rule_objects_and_reports_real_execution() {
        let report = run(r#"[{"name":"demo","ruleSearch":"{\"bookList\":\"div.item\",\"name\":\".name\"}","searchUrl":"https://example.test?q={{key}}"}]"#).unwrap();
        assert_eq!(report.sources, 1);
        assert_eq!(report.rules, 3);
        assert_eq!(report.clean, 1);
        assert!(report.token_hits.contains_key("{{ template }}"));
    }

    #[test]
    fn malformed_rule_is_counted_as_blocking_error() {
        let report = run(r#"[{"ruleContent":"||"}]"#).unwrap();
        assert_eq!(report.clean, 0);
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn json_rules_are_audited_against_json_input() {
        let report = run(r#"[{"ruleContent":{"content":"$..content"}}]"#).unwrap();
        assert_eq!(report.clean, 1);
        assert!(report.errors.is_empty());
    }
}
