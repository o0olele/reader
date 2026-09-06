use crate::{domain::replace_rule::ReplaceRule, error::AppError};

#[derive(Clone)]
pub struct SqliteReplaceRuleRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteReplaceRuleRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<ReplaceRule>, AppError> {
        sqlx::query_as("SELECT * FROM replace_rules ORDER BY sort_order, id")
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::database)
    }

    pub async fn save(&self, rule: &ReplaceRule) -> Result<i64, AppError> {
        let id = (rule.id != 0).then_some(rule.id);
        sqlx::query_scalar("INSERT INTO replace_rules (id, name, \"group\", pattern, replacement, is_regex, scope, scope_title, scope_content, exclude_scope, sort_order, enabled) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET name=excluded.name, \"group\"=excluded.\"group\", pattern=excluded.pattern, replacement=excluded.replacement, is_regex=excluded.is_regex, scope=excluded.scope, scope_title=excluded.scope_title, scope_content=excluded.scope_content, exclude_scope=excluded.exclude_scope, sort_order=excluded.sort_order, enabled=excluded.enabled RETURNING id")
            .bind(id).bind(&rule.name).bind(&rule.group).bind(&rule.pattern).bind(&rule.replacement)
            .bind(rule.is_regex).bind(&rule.scope).bind(rule.scope_title).bind(rule.scope_content)
            .bind(&rule.exclude_scope).bind(rule.sort_order).bind(rule.enabled)
            .fetch_one(&self.pool).await.map_err(AppError::database)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM replace_rules WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::database)?;
        Ok(())
    }
}
