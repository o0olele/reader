use crate::error::AppError;

#[derive(Clone)]
pub struct SettingsService {
    pool: sqlx::SqlitePool,
}

impl SettingsService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn proxy_url(&self) -> Result<Option<String>, AppError> {
        sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = 'proxy_url'")
            .fetch_optional(&self.pool)
            .await
            .map(|value| value.filter(|item| !item.is_empty()))
            .map_err(AppError::database)
    }

    pub async fn save_proxy_url(&self, proxy: Option<&str>) -> Result<(), AppError> {
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('proxy_url', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(proxy.unwrap_or("")).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }
}
