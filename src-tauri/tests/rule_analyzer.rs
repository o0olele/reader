use reader_desktop_lib::source_engine::rule::{split_rule, RuleJoin, RuleMode, SourceRule};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    origin: String,
    raw: String,
    expected: String,
}

fn fingerprint(groups: &[Vec<SourceRule>]) -> String {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|rule| {
                    let join = match rule.join {
                        RuleJoin::Chain => 'C',
                        RuleJoin::Concat => 'A',
                        RuleJoin::Interleave => 'I',
                    };
                    let mode = match rule.mode {
                        RuleMode::Default => 'D',
                        RuleMode::XPath => 'X',
                        RuleMode::Json => 'J',
                        RuleMode::Js => 'S',
                        RuleMode::Regex => 'R',
                        RuleMode::WebJs => 'W',
                    };
                    format!(
                        "{join}{mode}{}:{}",
                        if rule.reverse { "-" } else { "" },
                        rule.rule
                    )
                })
                .collect::<Vec<_>>()
                .join(" > ")
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

#[test]
fn matches_legado_rule_fixture() {
    let fixture = include_str!("fixtures/rules/legado_rules.jsonl");
    let cases = fixture
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Fixture>(line).expect("valid fixture row"))
        .collect::<Vec<_>>();

    assert_eq!(
        cases.len(),
        50,
        "the compatibility baseline must stay at 50 cases"
    );
    for case in cases {
        let parsed = split_rule(&case.raw)
            .unwrap_or_else(|error| panic!("{} failed to parse: {error}", case.origin));
        assert_eq!(
            fingerprint(&parsed),
            case.expected,
            "origin: {}",
            case.origin
        );
    }
}
