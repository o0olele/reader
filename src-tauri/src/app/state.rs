use crate::{app::AppConfig, error::AppError};
use sqlx::SqlitePool;
use std::sync::Mutex;

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
            config: AppConfig::default(),
        }
    }

    pub fn database(&self) -> Result<SqlitePool, AppError> {
        self.db
            .lock()
            .map_err(|_| AppError::Database("数据库状态锁不可用".into()))?
            .clone()
            .ok_or_else(|| AppError::Database("数据库尚未初始化".into()))
    }
}
