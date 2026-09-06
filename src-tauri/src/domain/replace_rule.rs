use serde::{Deserialize, Serialize};

/// Field aliases accept Legado's replaceRule.json spelling.
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
#[serde(default)]
pub struct ReplaceRule {
    pub id: i64,
    pub name: String,
    pub group: Option<String>,
    pub pattern: String,
    pub replacement: String,
    #[serde(alias = "isRegex")]
    pub is_regex: bool,
    pub scope: Option<String>,
    #[serde(alias = "scopeTitle")]
    pub scope_title: bool,
    #[serde(alias = "scopeContent")]
    pub scope_content: bool,
    #[serde(alias = "excludeScope")]
    pub exclude_scope: Option<String>,
    #[serde(alias = "order", alias = "sortOrder")]
    pub sort_order: i64,
    #[serde(alias = "isEnabled")]
    pub enabled: bool,
}

impl Default for ReplaceRule {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            group: None,
            pattern: String::new(),
            replacement: String::new(),
            is_regex: true,
            scope: None,
            scope_title: false,
            scope_content: true,
            exclude_scope: None,
            sort_order: 0,
            enabled: true,
        }
    }
}

impl ReplaceRule {
    pub fn applies_to(&self, book_name: &str, origin: &str) -> bool {
        let matches = |scope: &str| {
            [book_name, origin]
                .iter()
                .any(|value| !value.is_empty() && scope.contains(value))
        };
        self.enabled
            && self
                .scope
                .as_deref()
                .is_none_or(|scope| scope.trim().is_empty() || matches(scope))
            && !self.exclude_scope.as_deref().is_some_and(matches)
    }
}
