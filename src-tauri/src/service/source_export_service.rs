use crate::{
    domain::source::BookSource,
    error::AppError,
    repository::{source::SqliteSourceRepository, SourceRepository},
};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct SourceExportResult {
    pub path: String,
    pub exported: usize,
}

pub struct SourceExportService {
    sources: SqliteSourceRepository,
}

impl SourceExportService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool),
        }
    }

    pub async fn export_legado(&self, target: &Path) -> Result<SourceExportResult, AppError> {
        let sources = self.sources.list().await?;
        let values = sources.iter().map(legado_value).collect::<Vec<_>>();
        let json = serde_json::to_string_pretty(&values).map_err(AppError::parse)?;
        if let Some(parent) = target.parent().filter(|path| !path.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        std::fs::write(target, json.as_bytes()).map_err(AppError::io)?;
        Ok(SourceExportResult {
            path: target.to_string_lossy().into_owned(),
            exported: values.len(),
        })
    }
}

fn legado_value(source: &BookSource) -> serde_json::Value {
    let raw = &source.raw_rules;
    serde_json::json!({
        "bookSourceName": source.name,
        "bookSourceUrl": source.base_url,
        "searchUrl": source.search_url,
        "exploreUrl": source.explore_url,
        "bookUrlPattern": source.book_url_pattern,
        "enabledCookieJar": source.enabled_cookie_jar,
        "header": source.header,
        "loginUrl": source.login_url,
        "concurrentRate": source.concurrent_rate,
        "enabled": source.enabled,
        "bookSourceGroup": source.source_group,
        "customOrder": source.custom_order,
        "weight": source.weight,
        "enabledExplore": source.enabled_explore,
        "respondTime": source.respond_time,
        "lastUpdateTime": source.last_update_time,
        "ruleSearch": raw_value(raw.search.as_deref()).unwrap_or_else(|| serde_json::json!({
            "bookList": source.search_rule.item,
            "name": source.search_rule.title,
            "author": source.search_rule.author,
            "coverUrl": source.search_rule.cover,
            "bookUrl": source.search_rule.url,
        })),
        "ruleBookInfo": raw_value(raw.book_info.as_deref()).unwrap_or_else(|| serde_json::json!({
            "name": source.info_rule.title,
            "author": source.info_rule.author,
            "intro": source.info_rule.intro,
            "coverUrl": source.info_rule.cover,
            "kind": source.info_rule.kind,
            "lastChapter": source.info_rule.latest_chapter,
            "canReName": source.info_rule.can_rename,
        })),
        "ruleToc": raw_value(raw.toc.as_deref()).unwrap_or_else(|| serde_json::json!({
            "chapterList": source.catalog_rule.item,
            "chapterName": source.catalog_rule.title,
            "chapterUrl": source.catalog_rule.url,
            "nextTocUrl": source.catalog_rule.next_url.as_ref().or(source.next_toc_url_selector.as_ref()),
        })),
        "ruleContent": raw_value(raw.content.as_deref()).unwrap_or_else(|| serde_json::json!({
            "content": source.content_selector,
            "nextContentUrl": source.next_content_url_selector,
        })),
        "ruleExplore": raw_value(raw.explore.as_deref()),
    })
}

fn raw_value(raw: Option<&str>) -> Option<serde_json::Value> {
    raw.and_then(|value| serde_json::from_str(value).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_preserved_raw_rule_objects() {
        let value = raw_value(Some(r#"{"bookList":"$.data"}"#)).unwrap();
        assert_eq!(value["bookList"], "$.data");
    }
}
