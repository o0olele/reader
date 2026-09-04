use crate::{
    app::AppState,
    infrastructure::db,
    infrastructure::http::request::{set_detected_user_agent, set_user_agent},
};
use std::path::Path;

pub async fn initialize_database(state: &AppState, path: &Path) -> Result<(), sqlx::Error> {
    let pool = db::connect(path).await?;
    let proxy =
        sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = 'proxy_url'")
            .fetch_optional(&pool)
            .await?;
    *state.global_proxy.lock().expect("proxy mutex poisoned") =
        proxy.filter(|value| !value.is_empty());
    // Install before anything can issue a request, so the auth webview and
    // every HTTP client agree on the User-Agent from the first call onward.
    // The detected value is the previous session's `navigator.userAgent`; the
    // frontend re-reports it on boot, which only matters when the webview
    // runtime updated between launches.
    let agent =
        sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = 'user_agent'")
            .fetch_optional(&pool)
            .await?;
    set_user_agent(agent.as_deref());
    let detected = sqlx::query_scalar::<_, String>(
        "SELECT value FROM app_settings WHERE key = 'webview_user_agent'",
    )
    .fetch_optional(&pool)
    .await?;
    set_detected_user_agent(detected.as_deref());
    *state.db.lock().expect("database mutex poisoned") = Some(pool);
    Ok(())
}
