//! Binary import avoids JSON serialization of each byte on the WebView thread.
use crate::{app::AppState, domain::Book, error::AppError, service::book_service::BookService};
use tauri::{
    ipc::{InvokeBody, Request},
    State,
};

#[tauri::command(rename = "import_local_book")]
pub async fn import_local_book_cmd(
    state: State<'_, AppState>,
    request: Request<'_>,
) -> Result<Book, AppError> {
    let raw_name = request
        .headers()
        .get("x-book-filename")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::InvalidArgument("缺少书籍文件名".into()))?;
    let filename = urlencoding::decode(raw_name)
        .map_err(|error| AppError::InvalidArgument(format!("文件名无效：{error}")))?;
    let InvokeBody::Raw(bytes) = request.body() else {
        return Err(AppError::InvalidArgument("导入需要二进制文件内容".into()));
    };
    let service = BookService::new(state.database()?);
    let started = std::time::Instant::now();
    let result = if filename.to_lowercase().ends_with(".epub") {
        service.import_epub(&filename, bytes.clone()).await
    } else {
        service.import_txt(&filename, bytes.clone()).await
    };
    tracing::info!(target: "book", bytes = bytes.len(), elapsed_ms = started.elapsed().as_millis(), "binary import completed");
    result
}
