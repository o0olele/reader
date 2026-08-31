//! SQLite connection and migration adapter.

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::Path;

pub async fn connect(path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let started = std::time::Instant::now();
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!(target: "database", elapsed_ms = started.elapsed().as_millis() as u64, "database migrations applied");
    Ok(pool)
}
