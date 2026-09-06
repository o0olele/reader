use crate::{
    domain::replace_rule::ReplaceRule,
    error::AppError,
    repository::replace_rule::SqliteReplaceRuleRepository,
    service::content_processor::{validate_rule, ContentProcessor, ProcessedContent},
};

pub struct ReplaceRuleService {
    repository: SqliteReplaceRuleRepository,
}

#[cfg(test)]
#[path = "replace_rule_tests.rs"]
mod tests;

impl ReplaceRuleService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            repository: SqliteReplaceRuleRepository::new(pool),
        }
    }

    pub async fn list(&self) -> Result<Vec<ReplaceRule>, AppError> {
        self.repository.list().await
    }

    pub async fn save(&self, mut rule: ReplaceRule) -> Result<ReplaceRule, AppError> {
        validate_rule(&rule)?;
        rule.name = rule.name.trim().to_owned();
        rule.id = self.repository.save(&rule).await?;
        Ok(rule)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository.delete(id).await
    }

    pub fn preview(
        rule: ReplaceRule,
        title: &str,
        content: &str,
    ) -> Result<ProcessedContent, AppError> {
        validate_rule(&rule)?;
        let rule = ReplaceRule {
            enabled: true,
            scope: None,
            exclude_scope: None,
            ..rule
        };
        ContentProcessor::new(vec![rule], "", "")?.process(title, content)
    }
}
