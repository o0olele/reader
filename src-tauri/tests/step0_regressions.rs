//! Step 0 regression fixtures. Database-heavy behavior is covered beside the
//! repository implementation so it can run without a Tauri application.

#[test]
fn legado_fixture_is_present() {
    let fixture = include_str!("fixtures/rules/legado_rules.jsonl");
    assert!(fixture.lines().count() >= 2);
    assert!(fixture.contains("@XPath:"));
}

#[test]
fn app_settings_drift_migration_is_documented_noop() {
    let migration = include_str!("../migrations/012_app_settings_drift.sql");
    assert!(!migration.to_ascii_uppercase().contains("CREATE TABLE"));
    assert!(migration.contains("migration 001"));
}
