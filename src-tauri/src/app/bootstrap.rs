use crate::{app::AppState, infrastructure::db};
use std::path::Path;

pub async fn initialize_database(state: &AppState, path: &Path) -> Result<(), sqlx::Error> {
    let pool = db::connect(path).await?;
    let proxy =
        sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = 'proxy_url'")
            .fetch_optional(&pool)
            .await?;
    *state.global_proxy.lock().expect("proxy mutex poisoned") =
        proxy.filter(|value| !value.is_empty());
    *state.db.lock().expect("database mutex poisoned") = Some(pool);
    Ok(())
}
