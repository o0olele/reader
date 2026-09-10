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
fn malformed_rule_is_ignored_like_legado() {
    let report = run(r#"[{"ruleContent":"||"}]"#).unwrap();
    assert_eq!(report.clean, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn json_rules_are_audited_against_json_input() {
    let report = run(r#"[{"ruleContent":{"content":"$..content"}}]"#).unwrap();
    assert_eq!(report.clean, 1);
    assert!(report.errors.is_empty());
}

#[test]
fn classifies_execution_errors_for_actionable_follow_up() {
    assert_eq!(
        error_category("source error: JavaScript 执行失败: not a function"),
        "js runtime"
    );
    assert_eq!(
        error_category(
            "default-mode rule is not supported: `a.` is not a CSS selector: EmptySelector"
        ),
        "css compatibility"
    );
    assert_eq!(
        error_category("source error: JavaScript 执行失败: cannot read property of null"),
        "js runtime"
    );
    assert_eq!(
        error_category("source error: JavaImporter is not defined"),
        "unsupported JVM access"
    );
}

#[test]
fn skips_literal_url_metadata_from_rule_execution() {
    assert!(is_metadata_url(
        "ruleSearch.bookUrl",
        "https://example.test/book/1"
    ));
    assert!(!is_metadata_url(
        "ruleSearch.bookUrl",
        "$.id@js:'https://example.test/book/' + result"
    ));
}

#[test]
fn executes_json_paths_instead_of_skipping_them() {
    let report = run(r#"[{"searchUrl":"https://example.test/api/search","ruleSearch":{"name":"data[*].title"}}]"#).unwrap();
    assert_eq!(report.clean, 1);
    let report = run(r#"[{"searchUrl":"https://example.test/api/search","ruleSearch":{"name":"data[invalid].title"}}]"#).unwrap();
    assert_eq!(report.clean, 0);
    assert!(report.blocked_by.contains_key("path parser"));
    assert!(markdown(&report).contains("source[0].ruleSearch.name"));
}

#[test]
fn mixed_sources_use_html_for_html_stages_and_json_for_json_rules() {
    assert_eq!(
        dummy::input_for("[property=og:title]@content", true),
        dummy::HTML
    );
    assert_eq!(dummy::input_for("@XPath://a/@href", true), dummy::HTML);
    assert_eq!(dummy::input_for("@Json:$.data[*]", false), dummy::JSON);
    assert_eq!(dummy::input_for("data[*].title", true), dummy::JSON);
    assert_eq!(dummy::input_for("@CSS:div", true), dummy::HTML);
}

#[test]
fn classifies_nested_json_path_errors() {
    assert_eq!(
        error_category("source error: json path is invalid: unexpected character at 14"),
        "path parser"
    );
}

#[test]
fn only_parse_level_failures_ignore_the_fallback_input() {
    // The preferred dialect could not parse the rule, so letting the other
    // dialect report "no match" would hide a real engine gap.
    assert!(is_parse_error(
        "source error: json path is invalid: unexpected character at 14"
    ));
    assert!(is_parse_error(
        "default-mode rule is not supported: `a.` is not a CSS selector: EmptySelector"
    ));
    // A script that ran and failed on the payload's shape is still excused.
    assert!(!is_parse_error(
        "source error: JavaScript 执行失败: Error: not a function"
    ));
    assert!(!is_parse_error(
        "source error: JavaScript 执行失败: Error: cannot read property 'x' of undefined"
    ));
}
