//! Step 0 regression fixtures. Database-heavy behavior is covered beside the
//! repository implementation so it can run without a Tauri application.

#[test]
fn legado_fixture_is_present() {
    let fixture = include_str!("fixtures/rules/legado_rules.jsonl");
    assert!(fixture.lines().count() >= 2);
    assert!(fixture.contains("@XPath:"));
}
