use crate::{
    domain::DownloadTask,
    error::AppError,
    repository::{book::SqliteBookRepository, download::SqliteDownloadRepository, BookRepository},
    service::reader_service::ReaderService,
};
use std::sync::{Arc, OnceLock};
use tokio::sync::Semaphore;

static DOWNLOAD_SLOTS: OnceLock<Arc<Semaphore>> = OnceLock::new();

#[derive(Clone)]
pub struct DownloadService {
    pool: sqlx::SqlitePool,
    tasks: SqliteDownloadRepository,
    books: SqliteBookRepository,
}

impl DownloadService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            tasks: SqliteDownloadRepository::new(pool.clone()),
            books: SqliteBookRepository::new(pool.clone()),
            pool,
        }
    }
    pub async fn list(&self) -> Result<Vec<DownloadTask>, AppError> {
        self.tasks.list().await
    }
    pub async fn create(
        &self,
        book_id: i64,
        chapter_ids: Option<Vec<i64>>,
    ) -> Result<i64, AppError> {
        self.books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        self.tasks.create(book_id, chapter_ids.as_deref()).await
    }
    pub async fn pause(&self, task_id: i64) -> Result<(), AppError> {
        self.tasks.set_status(task_id, "paused").await
    }
    pub async fn cancel(&self, task_id: i64) -> Result<(), AppError> {
        self.tasks.set_status(task_id, "cancelled").await
    }
    pub async fn resume(&self, task_id: i64) -> Result<(), AppError> {
        self.tasks.set_status(task_id, "pending").await
    }
    pub fn spawn(pool: sqlx::SqlitePool, task_id: i64) {
        tauri::async_runtime::spawn(async move {
            let service = Self::new(pool.clone());
            if let Err(error) = service.run(task_id).await {
                tracing::warn!(target: "download", task_id, %error, "download task failed");
                let _ = service.tasks.fail(task_id, &error.to_string()).await;
            }
        });
    }
    pub async fn resume_incomplete(pool: sqlx::SqlitePool) -> Result<(), AppError> {
        let tasks = SqliteDownloadRepository::new(pool.clone());
        for task_id in tasks.incomplete_ids().await? {
            tasks.set_status(task_id, "pending").await?;
            Self::spawn(pool.clone(), task_id);
        }
        Ok(())
    }
    async fn run(&self, task_id: i64) -> Result<(), AppError> {
        if !self.tasks.claim(task_id).await? {
            return Ok(());
        }
        let _permit = DOWNLOAD_SLOTS
            .get_or_init(|| Arc::new(Semaphore::new(4)))
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| AppError::Database("下载调度器不可用".into()))?;
        let task = self
            .tasks
            .list()
            .await?
            .into_iter()
            .find(|task| task.id == task_id)
            .ok_or_else(|| AppError::InvalidArgument("下载任务不存在".into()))?;
        let book = self
            .books
            .get(task.book_id)
            .await?
            .ok_or_else(|| AppError::InvalidArgument("书籍不存在".into()))?;
        let reader = ReaderService::new(self.pool.clone());
        let mut chapters = reader.list_chapters(book.id).await?;
        if chapters.is_empty() && book.source_id.is_some() {
            chapters = reader.refresh_catalog(book.id).await?;
        }
        if let Some(selected) = self.tasks.chapter_ids(task_id).await? {
            let selected: std::collections::HashSet<_> = selected.into_iter().collect();
            chapters.retain(|chapter| selected.contains(&chapter.id));
        }
        self.tasks.set_total(task_id, chapters.len() as i64).await?;
        let Some(source_id) = book.source_id else {
            self.tasks.set_status(task_id, "completed").await?;
            return Ok(());
        };
        for chapter in chapters {
            if self.tasks.status(task_id).await?.as_deref() != Some("running") {
                return Ok(());
            }
            if chapter.content.is_empty() {
                let url = chapter
                    .remote_url
                    .as_deref()
                    .ok_or_else(|| AppError::Source(format!("章节“{}”缺少地址", chapter.title)))?;
                let mut last_error = None;
                for attempt in 0..3 {
                    match reader
                        .fetch_online_content(source_id, url, Some(chapter.id))
                        .await
                    {
                        Ok(_) => {
                            last_error = None;
                            break;
                        }
                        Err(error) => {
                            last_error = Some(error);
                            if attempt < 2 {
                                tokio::time::sleep(std::time::Duration::from_millis(
                                    300 * (attempt + 1),
                                ))
                                .await;
                            }
                        }
                    }
                }
                if let Some(error) = last_error {
                    return Err(error);
                }
            }
            self.tasks.advance(task_id).await?;
        }
        self.tasks.set_status(task_id, "completed").await
    }
}
