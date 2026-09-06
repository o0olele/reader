use super::*;
use crate::domain::replace_rule::ReplaceRule;

#[test]
fn applies_ordered_regex_and_literal_rules_to_title_and_content() {
    let mut first = ReplaceRule {
        name: "trim ad".into(),
        pattern: r"广告\s*".into(),
        replacement: "".into(),
        scope_title: true,
        scope_content: true,
        sort_order: 1,
        ..Default::default()
    };
    let second = ReplaceRule {
        name: "mask".into(),
        pattern: "秘密".into(),
        replacement: "公开".into(),
        is_regex: false,
        scope_title: false,
        scope_content: true,
        sort_order: 2,
        ..Default::default()
    };
    first.enabled = true;
    let processor = ContentProcessor::new(vec![second, first], "书", "源").unwrap();
    let value = processor.process("广告标题", "广告\n秘密").unwrap();
    assert_eq!(value.title, "标题");
    assert_eq!(value.content, "公开");
}

#[test]
fn scope_and_exclusion_are_respected() {
    let rule = ReplaceRule {
        name: "only source".into(),
        pattern: "x".into(),
        replacement: "y".into(),
        is_regex: false,
        scope: Some("source".into()),
        exclude_scope: Some("other".into()),
        ..Default::default()
    };
    assert!(rule.applies_to("book", "source"));
    assert!(!rule.applies_to("book", "other"));
}

#[test]
fn supports_java_capture_references_escaping_and_lookaround() {
    let rule = ReplaceRule {
        name: "groups".into(),
        pattern: r"(?<=前)(?<word>文)(字)?".into(),
        replacement: r"${word}/$1suffix/$12/\$1".into(),
        ..Default::default()
    };
    let result = ContentProcessor::new(vec![rule], "", "")
        .unwrap()
        .process("", "前文字")
        .unwrap();
    assert_eq!(result.content, "前文/文suffix/文2/$1");
}

#[test]
fn rejects_invalid_patterns_and_references_before_saving() {
    let mut rule = ReplaceRule {
        name: "bad".into(),
        pattern: "(".into(),
        ..Default::default()
    };
    assert!(validate_rule(&rule).is_err());
    rule.pattern = "(a)".into();
    for replacement in ["$2", "${missing}", "$", "\\"] {
        rule.replacement = replacement.into();
        assert!(validate_rule(&rule).is_err(), "{replacement}");
    }
    rule.is_regex = false;
    assert!(validate_rule(&rule).is_ok());
}

#[test]
fn excludes_matching_books_and_keeps_disabled_rules_inactive() {
    let mut rule = ReplaceRule {
        name: "ad".into(),
        pattern: "ad".into(),
        scope: Some("Book,https://source.test".into()),
        exclude_scope: Some("Excluded".into()),
        ..Default::default()
    };
    assert!(rule.applies_to("Book", ""));
    assert!(rule.applies_to("Unknown", "https://source.test"));
    assert!(!rule.applies_to("Excluded", "https://source.test"));
    assert!(!rule.applies_to("Unknown", ""));
    rule.enabled = false;
    assert!(!rule.applies_to("Book", ""));
}
