//! 历史页（左导航「检索 → 历史」）的 IPC 适配层。
//!
//! 数据来自阅读位置与阅读时长两张表，聚合规则见
//! `repository/reading_record.rs`；这里只做参数形状与删除范围的翻译。

use crate::{
    app::AppState, domain::ReadingHistoryEntry, error::AppError,
    service::reader_service::ReaderService,
};
use tauri::State;

/// 跨书阅读记录，按最后阅读时间倒序。
#[tauri::command(rename = "list_reading_history")]
pub async fn list_reading_history_cmd(
    state: State<'_, AppState>,
) -> Result<Vec<ReadingHistoryEntry>, AppError> {
    ReaderService::new(state.database()?)
        .reading_history()
        .await
}

/// 忘记一本书读过：删除其阅读时长与阅读位置，书仍在书架。
#[tauri::command(rename = "clear_reading_history")]
pub async fn clear_reading_history_cmd(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<(), AppError> {
    ReaderService::new(state.database()?)
        .clear_reading_history(Some(book_id))
        .await
}

/// 清空全部阅读记录，含每日汇总（今日时长与连续天数一并归零）。
#[tauri::command(rename = "clear_all_reading_history")]
pub async fn clear_all_reading_history_cmd(state: State<'_, AppState>) -> Result<(), AppError> {
    ReaderService::new(state.database()?)
        .clear_reading_history(None)
        .await
}
