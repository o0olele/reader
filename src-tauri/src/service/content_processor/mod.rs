//! Display processing never writes over imported text or chapter caches.
mod replacement;
mod template;

use crate::{domain::replace_rule::ReplaceRule, error::AppError};
use replacement::CompiledReplacement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProcessedContent {
    pub title: String,
    pub content: String,
}

pub struct ContentProcessor {
    rules: Vec<(ReplaceRule, CompiledReplacement)>,
}

impl ContentProcessor {
    pub fn new(rules: Vec<ReplaceRule>, book_name: &str, origin: &str) -> Result<Self, AppError> {
        let mut rules: Vec<_> = rules
            .into_iter()
            .filter(|rule| rule.applies_to(book_name, origin))
            .map(|rule| Ok((rule.clone(), CompiledReplacement::new(&rule)?)))
            .collect::<Result<_, AppError>>()?;
        rules.sort_by_key(|(rule, _)| (rule.sort_order, rule.id));
        Ok(Self { rules })
    }

    pub fn process(&self, title: &str, content: &str) -> Result<ProcessedContent, AppError> {
        let mut result = ProcessedContent {
            title: title.to_owned(),
            content: content.to_owned(),
        };
        for (rule, replacement) in &self.rules {
            if rule.scope_title {
                result.title = replacement.apply(&result.title)?;
            }
            if rule.scope_content {
                result.content = replacement.apply(&result.content)?;
            }
        }
        result.content = paragraphs(&result.content);
        Ok(result)
    }
}

pub fn validate_rule(rule: &ReplaceRule) -> Result<(), AppError> {
    if rule.id < 0 || rule.name.trim().is_empty() || rule.pattern.is_empty() {
        return Err(AppError::InvalidArgument("请填写规则名称和匹配内容".into()));
    }
    if !rule.scope_title && !rule.scope_content {
        return Err(AppError::InvalidArgument(
            "至少选择标题或正文作为替换对象".into(),
        ));
    }
    CompiledReplacement::new(rule).map(|_| ())
}

fn paragraphs(content: &str) -> String {
    content
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests;
