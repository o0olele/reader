//! Book-source CRUD, legado import and authentication.
//!
//! Search and source probing live in [`crate::service::search_service`].

#[path = "source_service/auth.rs"]
pub mod auth;
#[path = "source_service/cookies.rs"]
pub mod cookies;

use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::request::user_agent,
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::{
        compat::{raw_unsupported_source_names, source_has_unsupported_rules},
        import::parse_sources_json,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SourceService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl SourceService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }
    pub async fn list(&self) -> Result<Vec<BookSource>, AppError> {
        self.sources.list().await
    }
    pub async fn get(&self, id: i64) -> Result<BookSource, AppError> {
        self.sources
            .get(id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))
    }
    pub async fn upsert(&self, source: &BookSource) -> Result<i64, AppError> {
        self.sources.upsert(source).await
    }

    pub async fn set_enabled(&self, source_id: i64, enabled: bool) -> Result<(), AppError> {
        self.sources.set_enabled(source_id, enabled).await
    }

    pub async fn update_management(
        &self,
        source_id: i64,
        source_group: Option<String>,
        custom_order: i64,
        weight: i64,
        enabled_explore: bool,
    ) -> Result<(), AppError> {
        let source_group = source_group
            .map(|group| group.trim().to_owned())
            .filter(|group| !group.is_empty());
        if source_group
            .as_ref()
            .is_some_and(|group| group.chars().count() > 80)
        {
            return Err(AppError::InvalidArgument(
                "书源分组不能超过 80 个字符".into(),
            ));
        }
        self.sources
            .update_management(
                source_id,
                source_group.as_deref(),
                custom_order,
                weight,
                enabled_explore,
            )
            .await
    }

    pub async fn import_json(&self, input: &str) -> Result<SourceImportReport, AppError> {
        let sources = parse_sources_json(input)?;
        let raw_partial = raw_unsupported_source_names(input);
        let mut report = SourceImportReport::default();
        for source in sources {
            let partial =
                raw_partial.contains(&source.name) || source_has_unsupported_rules(&source);
            match self.upsert(&BookSource::from_import(&source)).await {
                Ok(_) => {
                    report.imported += 1;
                    if partial {
                        report.partial.push(source.name);
                    }
                }
                Err(error) => report.failed.push(format!("{}: {error}", source.name)),
            }
        }
        tracing::info!(target: "source", imported = report.imported, failed = report.failed.len(), partial = report.partial.len(), "legado source import finished");
        Ok(report)
    }

    pub async fn import_url(&self, raw_url: &str) -> Result<SourceImportReport, AppError> {
        let url = reqwest::Url::parse(raw_url.trim())
            .map_err(|_| AppError::InvalidArgument("书源 URL 无效".into()))?;
        // Deliberately plain: no cookie jar, and reqwest's default redirect
        // policy. Routing this through `build_shared_client` would change both.
        let client = reqwest::Client::builder()
            .user_agent(user_agent())
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(AppError::network)?;
        let response = client.get(url).send().await.map_err(AppError::network)?;
        if !response.status().is_success() {
            return Err(format!("书源 URL 返回 HTTP {}", response.status()).into());
        }
        self.import_json(&response.text().await.map_err(AppError::network)?)
            .await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SourceImportReport {
    pub imported: usize,
    pub failed: Vec<String>,
    pub partial: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct SourceLoginInput {
    pub source_id: i64,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SourceLoginResult {
    pub source_id: i64,
    pub authenticated: bool,
    pub has_token: bool,
    pub has_cookie: bool,
    pub session_expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SourceSessionStatus {
    pub source_id: i64,
    pub state: String,
    pub has_token: bool,
    pub has_cookie: bool,
    pub expires_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::SourceService;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn pool() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn imports_partial_source_and_persists_it() {
        let service = SourceService::new(pool().await);
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article","name":".name","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;

        let report = service.import_json(input).await.unwrap();

        assert_eq!(report.imported, 1);
        assert_eq!(report.failed, Vec::<String>::new());
        assert_eq!(report.partial, vec!["XPath source"]);

        // The raw legado rules must survive the round trip through the
        // database, not just the write: the engine reads them back on search.
        let sources = service.list().await.unwrap();
        assert_eq!(sources.len(), 1);
        let raw = &sources[0].raw_rules;
        let search: serde_json::Value = serde_json::from_str(raw.search.as_ref().unwrap()).unwrap();
        assert_eq!(search["bookList"], "@XPath://article");
        assert_eq!(search["bookUrl"], "a");
        assert_eq!(raw.content.as_deref(), Some("\".content\""));
        assert!(raw.book_info.is_none());
    }
}
