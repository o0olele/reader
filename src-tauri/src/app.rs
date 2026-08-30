use crate::error::AppError;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::Path;
use std::sync::Mutex;

pub struct AppConfig {
    pub app_name: &'static str,
}

pub struct AppState {
    pub db: Mutex<Option<SqlitePool>>,
    pub global_proxy: Mutex<Option<String>>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(None),
            global_proxy: Mutex::new(None),
            config: AppConfig {
                app_name: "Reader Desktop",
            },
        }
    }

    pub fn database(&self) -> Result<SqlitePool, AppError> {
        self.db
            .lock()
            .map_err(|_| AppError::Database("数据库状态锁不可用".into()))?
            .clone()
            .ok_or_else(|| AppError::Database("数据库尚未初始化".into()))
    }

    pub fn proxy(&self) -> Result<Option<String>, AppError> {
        self.global_proxy
            .lock()
            .map_err(|_| AppError::Database("代理状态锁不可用".into()))
            .map(|value| value.clone())
    }

    pub async fn initialize_database(&self, path: &Path) -> Result<(), sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let proxy = sqlx::query_scalar::<_, String>(
            "SELECT value FROM app_settings WHERE key = 'proxy_url'",
        )
        .fetch_optional(&pool)
        .await?;
        *self.global_proxy.lock().expect("proxy mutex poisoned") =
            proxy.filter(|value| !value.is_empty());
        *self.db.lock().expect("database mutex poisoned") = Some(pool);
        Ok(())
    }
}
