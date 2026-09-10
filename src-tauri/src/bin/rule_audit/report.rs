use super::{error_category, Audit};
use std::collections::BTreeMap;

pub(super) fn markdown(report: &Audit) -> String {
    let percentage = |n: usize, d: usize| {
        if d == 0 {
            0.0
        } else {
            n as f64 * 100.0 / d as f64
        }
    };
    let mut out = format!("# Rule coverage audit\n\n- Sources: **{}**\n- Rule strings: **{}**\n- No errors on dummy input: **{} / {} ({:.1}%)**\n- Blocked sources: **{}**\n- Executed rule fields: **{}**\n- Skipped metadata fields: **{}**\n\n", report.sources, report.rules, report.clean, report.sources, percentage(report.clean, report.sources), report.sources.saturating_sub(report.clean), report.executed, report.rules - report.executed);
    out.push_str("Input types are inferred from source metadata. Every rule is dry-run against both the HTML and the JSON dummy payload, and only counts as failing when neither of them lets it execute — a rule that merely needs a key the dummy does not carry is an artefact, not an engine gap. Empty matches count as successful execution; this report does not establish extraction correctness or online availability. JSON field paths and display templates are executed by the production engine. Failure locations use zero-based corpus array indexes.\n\n");
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
        out.push_str(&format!("| `{}` | {} |\n", cell(error), count));
    }
    out.push_str("\n## Failed rule examples\n\n| Error | Example paths |\n| --- | --- |\n");
    for (error, examples) in &report.failed_rules {
        let sample = examples
            .iter()
            .map(|v| cell(v))
            .collect::<Vec<_>>()
            .join("<br>");
        out.push_str(&format!("| {} | {} |\n", cell(error), sample));
    }
    let mut category_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (error, count) in &report.errors {
        *category_counts.entry(error_category(error)).or_default() += count;
    }
    out.push_str(
        "\n## Execution errors by category\n\n| Category | Rule count |\n| --- | ---: |\n",
    );
    let mut categories: Vec<_> = category_counts.into_iter().collect();
    categories.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    for (category, count) in categories {
        out.push_str(&format!("| {category} | {count} |\n"));
    }
    out.push_str("\n## java.* methods\n\n| Method | Sources |\n| --- | ---: |\n");
    let mut methods: Vec<_> = report.java_methods.iter().collect();
    methods.sort_by_key(|(_, sources)| std::cmp::Reverse(sources.len()));
    for (method, sources) in methods {
        out.push_str(&format!("| `java.{method}` | {} |\n", sources.len()));
    }
    out.push_str(
        "\n## Blocked sources by category\n\n| Category | Blocked sources | Unblocked by fixing only this category |\n| --- | ---: | ---: |\n",
    );
    let mut blocked: Vec<_> = report.blocked_by.iter().collect();
    blocked.sort_by_key(|(_, sources)| std::cmp::Reverse(sources.len()));
    for (category, sources) in blocked {
        let sole = sources
            .iter()
            .filter(|source| {
                report
                    .blocked_by
                    .iter()
                    .all(|(other, blocked)| other == category || !blocked.contains(source))
            })
            .count();
        out.push_str(&format!("| {category} | {} | {sole} |\n", sources.len()));
    }
    out
}

fn cell(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('`', "&#96;")
        .replace('|', "&#124;")
        .replace('\r', "")
        .replace('\n', "<br>")
}
