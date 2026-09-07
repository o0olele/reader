use super::*;
use crate::{
    infrastructure::http::request::response_error,
    service::source_session::SourceSession,
    source_engine::{
        pipeline::{parse_catalog_page, parse_content_page},
        url::{build_with_base, decode_text},
    },
};
use std::collections::HashSet;
impl ReaderService {
    pub async fn fetch_online_content(
        &self,
        source_id: i64,
        chapter_url: &str,
        chapter_id: Option<i64>,
    ) -> Result<String, AppError> {
        if let Some(id) = chapter_id {
            if let Some(cached) = self.cached_content(id).await? {
                tracing::debug!(target: "reader", chapter_id = id, "chapter cache hit");
                return Ok(cached);
            }
            tracing::debug!(target: "reader", chapter_id = id, "chapter cache miss");
        }
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let session = SourceSession::new(
            source.clone(),
            self.sources.clone(),
            15,
            self.settings.proxy_url().await?.as_deref(),
        )?;
        let mut current_rule = chapter_url.to_owned();
        let mut current_base = source.base_url.clone();
        let mut visited = HashSet::new();
        let mut pages = Vec::new();
        for _ in 0..20 {
            let request = build_with_base(&source, &current_base, &current_rule, None, "正文 URL")?;
            let request_key = format!("{} {} {:?}", request.method, request.url, request.body);
            if !visited.insert(request_key) {
                break;
            }
            let response = session.send(&request).await?;
            if !response.status().is_success() {
                return Err(AppError::Network(
                    response_error(response, &source.name).await,
                ));
            }
            let html = decode_text(response, &request, &source).await?;
            let (page, next) = parse_content_page(&source, &html)?;
            pages.push(page);
            let Some(next) = next else {
                break;
            };
            current_base = request.url.to_string();
            current_rule = next;
        }
        let content = pages.join("\n");
        if let Some(id) = chapter_id {
            self.cache_content(id, &content).await?;
        }
        Ok(content)
    }

    pub async fn refresh_catalog(&self, book_id: i64) -> Result<Vec<Chapter>, AppError> {
        let book = self
            .books
            .get(book_id)
            .await?
            .ok_or_else(|| AppError::Source("书籍不存在".into()))?;
        let source_id = book
            .source_id
            .ok_or_else(|| AppError::Source("本地书籍没有在线书源".into()))?;
        let book_url = book
            .remote_url
            .ok_or_else(|| AppError::Source("书籍没有远程地址".into()))?;
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let session = SourceSession::new(
            source.clone(),
            self.sources.clone(),
            15,
            self.settings.proxy_url().await?.as_deref(),
        )?;
        let mut current_rule = book_url;
        let mut current_base = source.base_url.clone();
        let mut visited = HashSet::new();
        let mut catalog = Vec::new();
        for _ in 0..50 {
            let request = build_with_base(&source, &current_base, &current_rule, None, "目录 URL")?;
            let request_key = format!("{} {} {:?}", request.method, request.url, request.body);
            if !visited.insert(request_key) {
                break;
            }
            let response = session.send(&request).await?;
            if !response.status().is_success() {
                return Err(AppError::Network(
                    response_error(response, &source.name).await,
                ));
            }
            let html = decode_text(response, &request, &source).await?;
            let (page, next) = parse_catalog_page(&source, &html)?;
            catalog.extend(page);
            let Some(next) = next else {
                break;
            };
            current_base = request.url.to_string();
            current_rule = next;
        }
        tracing::info!(target: "reader", book_id, chapter_count = catalog.len(), "catalog refreshed");
        if catalog.is_empty() {
            return Err(AppError::Source("书源没有解析出目录".into()));
        }
        self.replace_catalog(book_id, &catalog).await?;
        self.list_chapters(book_id).await
    }
}
