//! Logical, schema-tolerant application backups.
//!
//! A backup is JSON rather than a raw SQLite copy so it can be restored after
//! migrations have added columns. Only application tables are included; the
//! migration bookkeeping table is deliberately excluded.

use crate::error::AppError;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::{sqlite::SqliteValueRef, Column, Row, SqlitePool, Value as SqlxValue, ValueRef};
use std::{collections::BTreeMap, path::Path};

const TABLES: &[&str] = &[
    "app_settings",
    "bookshelf_groups",
    "book_sources",
    "books",
    "chapters",
    "chapter_contents",
    "reading_progress",
    "bookmarks",
    "reading_records",
    "replace_rules",
    "download_tasks",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupDocument {
    pub format: String,
    pub version: u32,
    pub exported_at: String,
    pub tables: BTreeMap<String, Vec<Map<String, Value>>>,
}

#[derive(Clone)]
pub struct BackupService {
    pool: SqlitePool,
}

impl BackupService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn export_to(&self, path: &Path) -> Result<usize, AppError> {
        let document = self.export().await?;
        let bytes = serde_json::to_vec_pretty(&document).map_err(AppError::parse)?;
        let temp = path.with_extension("tmp");
        tokio::fs::write(&temp, bytes).await.map_err(AppError::io)?;
        tokio::fs::rename(&temp, path).await.map_err(AppError::io)?;
        Ok(document.tables.values().map(Vec::len).sum())
    }

    pub async fn export(&self) -> Result<BackupDocument, AppError> {
        let mut tables = BTreeMap::new();
        for table in TABLES {
            let rows = sqlx::query(&format!("SELECT * FROM {table}"))
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::database)?;
            let mut encoded = Vec::with_capacity(rows.len());
            for row in rows {
                let mut object = Map::new();
                for (index, column) in row.columns().iter().enumerate() {
                    object.insert(
                        column.name().to_owned(),
                        sqlite_value(row.try_get_raw(index))?,
                    );
                }
                encoded.push(object);
            }
            tables.insert((*table).to_owned(), encoded);
        }
        Ok(BackupDocument {
            format: "reader-desktop-backup".into(),
            version: 1,
            exported_at: chrono_like_now(),
            tables,
        })
    }

    pub async fn restore_from(&self, path: &Path) -> Result<usize, AppError> {
        let bytes = tokio::fs::read(path).await.map_err(AppError::io)?;
        if bytes.len() > 100 * 1024 * 1024 {
            return Err(AppError::InvalidArgument("备份文件超过 100 MiB".into()));
        }
        let document: BackupDocument = serde_json::from_slice(&bytes)
            .map_err(|error| AppError::Parse(format!("备份格式无效: {error}")))?;
        if document.format != "reader-desktop-backup" || document.version != 1 {
            return Err(AppError::InvalidArgument("不支持的备份格式或版本".into()));
        }
        let mut tx = self.pool.begin().await.map_err(AppError::database)?;
        for table in TABLES.iter().rev() {
            sqlx::query(&format!("DELETE FROM {table}"))
                .execute(&mut *tx)
                .await
                .map_err(AppError::database)?;
        }
        let mut count = 0;
        for table in TABLES {
            let Some(rows) = document.tables.get(*table) else {
                continue;
            };
            for row in rows {
                insert_row(&mut tx, table, row).await?;
                count += 1;
            }
        }
        tx.commit().await.map_err(AppError::database)?;
        Ok(count)
    }
}

fn sqlite_value(raw: Result<SqliteValueRef<'_>, sqlx::Error>) -> Result<Value, AppError> {
    let raw = raw.map_err(AppError::database)?;
    if raw.is_null() {
        return Ok(Value::Null);
    }
    if let Ok(value) = raw.to_owned().try_decode::<i64>() {
        return Ok(Value::from(value));
    }
    if let Ok(value) = raw.to_owned().try_decode::<f64>() {
        return Ok(Value::from(value));
    }
    if let Ok(value) = raw.to_owned().try_decode::<String>() {
        return Ok(Value::from(value));
    }
    let bytes = raw
        .to_owned()
        .try_decode::<Vec<u8>>()
        .map_err(AppError::database)?;
    Ok(Value::String(format!(
        "base64:{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )))
}

async fn insert_row(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    table: &str,
    row: &Map<String, Value>,
) -> Result<(), AppError> {
    if row.is_empty() || !TABLES.contains(&table) {
        return Ok(());
    }
    let columns: Vec<_> = row.keys().filter(|key| is_safe_identifier(key)).collect();
    if columns.is_empty() || columns.len() != row.len() {
        return Err(AppError::Parse(format!("备份表 {table} 含有非法列名")));
    }
    let placeholders = std::iter::repeat_n("?", columns.len())
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "INSERT INTO {table} ({}) VALUES ({placeholders})",
        columns
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut query = sqlx::query(&sql);
    for column in columns {
        let value = row.get(column).expect("column checked above");
        query = match value {
            Value::Null => query.bind(Option::<String>::None),
            Value::Bool(value) => query.bind(i64::from(*value)),
            Value::Number(value) if value.is_i64() => query.bind(value.as_i64().unwrap()),
            Value::Number(value) => query.bind(value.as_f64().unwrap_or_default()),
            Value::String(value) if value.starts_with("base64:") => query.bind(
                base64::engine::general_purpose::STANDARD
                    .decode(&value[7..])
                    .map_err(AppError::parse)?,
            ),
            Value::String(value) => query.bind(value),
            _ => {
                return Err(AppError::Parse(format!(
                    "备份表 {table} 含有不支持的值类型"
                )))
            }
        };
    }
    query.execute(&mut **tx).await.map_err(AppError::database)?;
    Ok(())
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn chrono_like_now() -> String {
    // Keep the format dependency-free; it is metadata only.
    format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn round_trip_preserves_rows_and_binary_values() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        sqlx::query("INSERT INTO app_settings(key, value) VALUES ('theme', 'dark')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO books(title, path) VALUES ('测试', 'test.txt')")
            .execute(&pool)
            .await
            .unwrap();
        let service = BackupService::new(pool.clone());
        let document = service.export().await.unwrap();
        assert_eq!(document.format, "reader-desktop-backup");
        assert_eq!(document.tables["books"].len(), 1);
        let path =
            std::env::temp_dir().join(format!("reader-backup-{}.json", uuid::Uuid::new_v4()));
        service.export_to(&path).await.unwrap();
        sqlx::query("DELETE FROM books")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM app_settings")
            .execute(&pool)
            .await
            .unwrap();
        service.restore_from(&path).await.unwrap();
        let title: String = sqlx::query_scalar("SELECT title FROM books LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(title, "测试");
        let theme: String =
            sqlx::query_scalar("SELECT value FROM app_settings WHERE key = 'theme'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(theme, "dark");
        let _ = tokio::fs::remove_file(path).await;
    }
}
