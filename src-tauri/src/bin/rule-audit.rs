//! Static rule coverage audit for a Legado source corpus.
//!
//! Usage: `cargo run --bin rule-audit -- --corpus <file-or-dir> --out <dir>`
//! The audit deliberately executes the real rule splitter/evaluator against a
//! deterministic dummy document; it never performs network requests.

use reader_desktop_lib::source_engine::rule::{evaluate, Extraction, RuleContext};
#[path = "rule_audit/dummy.rs"]
mod dummy;
#[path = "rule_audit/input.rs"]
mod input;
#[path = "rule_audit/report.rs"]
mod report;
#[cfg(test)]
#[path = "rule_audit/tests.rs"]
mod tests;
use input::{error_category, is_metadata_url, source_is_json, source_rules, TOKENS};
use regex::Regex;
use report::markdown;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Default)]
struct Audit {
    sources: usize,
    rules: usize,
    clean: usize,
    executed: usize,
    token_hits: BTreeMap<String, (usize, BTreeSet<usize>)>,
    errors: BTreeMap<String, usize>,
    java_methods: BTreeMap<String, BTreeSet<usize>>,
    blocked_by: BTreeMap<String, BTreeSet<usize>>,
    failed_rules: BTreeMap<String, BTreeSet<String>>,
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
        let json_source = source_is_json(source, &rules);
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
            if path.starts_with("rule") && !is_metadata_url(&path, &raw) {
                report.executed += 1;
                let dummy_input = dummy::input_for(&raw, json_source);
                if let Err(error) = evaluate(
                    &raw,
                    dummy_input,
                    Extraction::Values,
                    &mut RuleContext::default(),
                ) {
                    source_clean = false;
                    *report.errors.entry(error.to_string()).or_default() += 1;
                    report
                        .failed_rules
                        .entry(error.to_string())
                        .or_default()
                        .insert(format!("source[{source_id}].{path} = {raw}"));
                    report
                        .blocked_by
                        .entry(error_category(&error.to_string()).to_owned())
                        .or_default()
                        .insert(source_id);
                }
            }
        }
        if source_clean {
            report.clean += 1;
        }
    }
    Ok(report)
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
