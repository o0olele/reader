use crate::{
    error::AppError,
    repository::{source::SqliteSourceRepository, SourceRepository},
    source::BookSource,
};

#[derive(Clone)]
pub struct SourceService {
    sources: SqliteSourceRepository,
}

impl SourceService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool),
        }
    }

    pub async fn list(&self) -> Result<Vec<BookSource>, AppError> {
        self.sources.list().await
    }

    pub async fn get(&self, source_id: i64) -> Result<BookSource, AppError> {
        self.sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))
    }

    pub async fn upsert(&self, source: &BookSource) -> Result<i64, AppError> {
        self.sources.upsert(source).await
    }
}
