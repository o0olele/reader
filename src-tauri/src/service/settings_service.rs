use crate::error::AppError;

/// How many chapters ahead of the open one are pulled into the cache while
/// reading. The reference app defaults to the same number (legado
/// `ReadSettings.preDownloadNum`).
pub const DEFAULT_PREFETCH_CHAPTERS: i64 = 10;
/// Upper bound for the same setting; the reference exposes a 0–50 slider.
pub const MAX_PREFETCH_CHAPTERS: i64 = 50;

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

    /// `None` means "use the built-in default". Stored rather than baked in
    /// because the UA has to be changeable when a site starts rejecting the
    /// one we ship, and it must stay identical across the auth webview and
    /// every HTTP client for Cloudflare clearance cookies to survive.
    pub async fn user_agent(&self) -> Result<Option<String>, AppError> {
        sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = 'user_agent'")
            .fetch_optional(&self.pool)
            .await
            .map(|value| value.filter(|item| !item.trim().is_empty()))
            .map_err(AppError::database)
    }

    pub async fn save_user_agent(&self, user_agent: Option<&str>) -> Result<(), AppError> {
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('user_agent', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(user_agent.unwrap_or("")).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    pub async fn cache_quota_mb(&self) -> Result<Option<i64>, AppError> {
        sqlx::query_scalar::<_, String>(
            "SELECT value FROM app_settings WHERE key = 'cache_quota_mb'",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database)?
        .map(|value| {
            value
                .parse()
                .map_err(|error| AppError::Parse(format!("缓存上限设置无效: {error}")))
        })
        .transpose()
    }

    pub async fn save_cache_quota_mb(&self, megabytes: i64) -> Result<(), AppError> {
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('cache_quota_mb', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(megabytes.to_string()).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    /// The `navigator.userAgent` the main window last reported. Cached across
    /// launches so the very first request of a session already matches the
    /// webview, instead of waiting for the frontend to boot and report in.
    pub async fn webview_user_agent(&self) -> Result<Option<String>, AppError> {
        sqlx::query_scalar::<_, String>(
            "SELECT value FROM app_settings WHERE key = 'webview_user_agent'",
        )
        .fetch_optional(&self.pool)
        .await
        .map(|value| value.filter(|item| !item.trim().is_empty()))
        .map_err(AppError::database)
    }

    pub async fn save_webview_user_agent(&self, user_agent: &str) -> Result<(), AppError> {
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('webview_user_agent', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(user_agent).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    /// Daily reading goal in minutes; `0` means "no goal set" (ROADMAP-v3 E1).
    pub async fn reading_goal_minutes(&self) -> Result<i64, AppError> {
        sqlx::query_scalar::<_, String>(
            "SELECT value FROM app_settings WHERE key = 'reading_daily_goal_minutes'",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database)?
        .map(|value| {
            value
                .parse()
                .map_err(|error| AppError::Parse(format!("每日阅读目标设置无效: {error}")))
        })
        .transpose()
        .map(|value| value.unwrap_or(0))
    }

    pub async fn save_reading_goal_minutes(&self, minutes: i64) -> Result<(), AppError> {
        if minutes < 0 {
            return Err(AppError::InvalidArgument("每日阅读目标不能为负数".into()));
        }
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('reading_daily_goal_minutes', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(minutes.to_string()).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }

    /// Chapters prefetched ahead of the open one; `0` disables prefetching.
    /// Unset means the reference default, so an existing database gains the new
    /// behaviour without a migration.
    pub async fn reader_prefetch_num(&self) -> Result<i64, AppError> {
        sqlx::query_scalar::<_, String>(
            "SELECT value FROM app_settings WHERE key = 'reader_prefetch_num'",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database)?
        .map(|value| {
            value
                .parse()
                .map_err(|error| AppError::Parse(format!("预下载章节数设置无效: {error}")))
        })
        .transpose()
        .map(|value| {
            value
                .unwrap_or(DEFAULT_PREFETCH_CHAPTERS)
                .clamp(0, MAX_PREFETCH_CHAPTERS)
        })
    }

    pub async fn save_reader_prefetch_num(&self, chapters: i64) -> Result<(), AppError> {
        if !(0..=MAX_PREFETCH_CHAPTERS).contains(&chapters) {
            return Err(AppError::InvalidArgument(format!(
                "预下载章节数须在 0 到 {MAX_PREFETCH_CHAPTERS} 之间"
            )));
        }
        sqlx::query("INSERT INTO app_settings (key, value) VALUES ('reader_prefetch_num', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
            .bind(chapters.to_string()).execute(&self.pool).await.map(|_| ()).map_err(AppError::database)
    }
}
