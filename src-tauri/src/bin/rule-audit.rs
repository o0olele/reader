//! Static rule coverage audit for a Legado source corpus.
//!
//! Usage: `cargo run --bin rule-audit -- --corpus <file-or-dir> --out <dir>`
//! The audit deliberately executes the real rule splitter/evaluator against a
//! deterministic dummy document; it never performs network requests.

use reader_desktop_lib::error::AppError;
use reader_desktop_lib::source_engine::rule::{evaluate, evaluate_url, Extraction, RuleContext};
#[path = "rule_audit/dummy.rs"]
mod dummy;
#[path = "rule_audit/input.rs"]
mod input;
#[path = "rule_audit/report.rs"]
mod report;
#[cfg(test)]
#[path = "rule_audit/tests.rs"]
mod tests;
use input::{
    error_category, is_hook_field, is_metadata_field, is_metadata_url, is_parse_error,
    is_url_field, source_is_json, source_rules, TOKENS,
};
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

/// Dry-runs one rule against every candidate dummy input and reports the
/// failure only when none of them lets the rule execute.
///
/// A deterministic corpus can never contain every key a source expects, so a
/// single-input judgement mixes "the engine cannot run this rule" with "this
/// dummy payload has no such field". The returned message lists the error seen
/// for each input, which keeps genuine engine gaps (invalid CSS, undeclared JS
/// variables) distinguishable from payload-shape artefacts.
///
/// The fallback dialect is only allowed to excuse a *runtime* failure of the
/// preferred dialect. When the preferred dialect cannot even parse the rule, a
/// success on the other dialect means no more than "the selector matched
/// nothing", so the parse error stays reported.
///
/// URL-valued fields are judged through [`evaluate_url`], which is what the
/// pipeline uses for them: legado resolves `bookUrl` / `chapterUrl` / … with
/// `AnalyzeUrl`, so a relative URL is rendered instead of being parsed as an
/// XPath expression.
fn rule_failure(path: &str, raw: &str, json_source: bool) -> Option<String> {
    let mut errors: Vec<String> = Vec::new();
    for input in dummy::inputs_for(raw, json_source) {
        let outcome = if is_url_field(path) {
            evaluate_url(raw, input, &mut RuleContext::default()).map(|_| Vec::new())
        } else {
            evaluate(raw, input, Extraction::Values, &mut RuleContext::default())
        };
        match outcome {
            // `evaluate` surfaces an error only when every `||` alternative
            // failed, so a success here means the rule executed.
            Ok(_) if !errors.first().is_some_and(|error| is_parse_error(error)) => return None,
            Ok(_) => break,
            Err(error) => errors.push(error.to_string()),
        }
    }
    errors.dedup();
    Some(errors.join(" || "))
}

fn corpus_file(path: &Path) -> Result<PathBuf, AppError> {
    if path.is_file() {
        return Ok(path.to_owned());
    }
    fs::read_dir(path)
        .map_err(|e| AppError::Io(format!("cannot read corpus directory: {e}")))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|candidate| candidate.extension().is_some_and(|ext| ext == "json"))
        .ok_or_else(|| AppError::InvalidArgument(format!("no JSON corpus found in {}", path.display())))
}

fn run(input: &str) -> Result<Audit, AppError> {
    let value: Value = serde_json::from_str(input)
        .map_err(|e| AppError::InvalidArgument(format!("invalid corpus JSON: {e}")))?;
    let sources = value
        .as_array()
        .ok_or_else(|| AppError::InvalidArgument("corpus root must be a JSON array".into()))?;
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
            // URL templates, headers, JS libraries and plain configuration are
            // metadata rather than evaluator rules; only actual rule fields are
            // dry-run here.
            if path.starts_with("rule") && !is_metadata_url(&path, &raw) && !is_metadata_field(&path)
            {
                report.executed += 1;
                if let Some(error) = rule_failure(&path, &raw, json_source) {
                    source_clean = false;
                    *report.errors.entry(error.clone()).or_default() += 1;
                    report
                        .failed_rules
                        .entry(error.clone())
                        .or_default()
                        .insert(format!("source[{source_id}].{path} = {raw}"));
                    // A hook this engine does not implement yet is not a CSS
                    // parsing gap; keep the attribution honest.
                    let category = if is_hook_field(&path) {
                        "unimplemented hook"
                    } else {
                        error_category(&error)
                    };
                    report
                        .blocked_by
                        .entry(category.to_owned())
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

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let value = |flag: &str| {
        args.windows(2)
            .find(|pair| pair[0] == flag)
            .map(|pair| pair[1].clone())
            .ok_or_else(|| AppError::InvalidArgument(format!("missing {flag}")))
    };
    let corpus = corpus_file(Path::new(&value("--corpus")?))?;
    let out_dir = PathBuf::from(value("--out")?);
    let report = run(
        &fs::read_to_string(&corpus)
            .map_err(|e| AppError::Io(format!("cannot read corpus: {e}")))?,
    )?;
    fs::create_dir_all(&out_dir)
        .map_err(|e| AppError::Io(format!("cannot create output directory: {e}")))?;
    let output = out_dir.join("rule-audit.md");
    fs::write(&output, markdown(&report))
        .map_err(|e| AppError::Io(format!("cannot write report: {e}")))?;
    println!("{}", output.display());
    Ok(())
}
