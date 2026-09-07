use crate::{app::AppState, error::AppError, service::backup_service::BackupService};
use serde::Serialize;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct BackupResult {
    pub rows: usize,
    pub path: String,
}

#[tauri::command(rename = "export_backup")]
pub async fn export_backup_cmd(
    state: State<'_, AppState>,
    path: String,
) -> Result<BackupResult, AppError> {
    let path = validate_path(path)?;
    let rows = BackupService::new(state.database()?)
        .export_to(&path)
        .await?;
    Ok(BackupResult {
        rows,
        path: path.display().to_string(),
    })
}

#[tauri::command(rename = "restore_backup")]
pub async fn restore_backup_cmd(
    state: State<'_, AppState>,
    path: String,
) -> Result<BackupResult, AppError> {
    let path = validate_path(path)?;
    let rows = BackupService::new(state.database()?)
        .restore_from(&path)
        .await?;
    Ok(BackupResult {
        rows,
        path: path.display().to_string(),
    })
}

fn validate_path(raw: String) -> Result<PathBuf, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidArgument("备份路径不能为空".into()));
    }
    let path = PathBuf::from(trimmed);
    if path.extension().and_then(|value| value.to_str()) != Some("json") {
        return Err(AppError::InvalidArgument(
            "备份文件必须使用 .json 扩展名".into(),
        ));
    }
    Ok(path)
}
