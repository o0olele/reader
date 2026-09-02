//! Single-request source diagnostics for the source debugger.

use crate::{
    domain::source::{BookSource, RawSourceRules},
    error::AppError,
    infrastructure::http::{client::build_source_client, request::source_request_with_method, url::resolve_url},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::pipeline::{parse_book_info, parse_catalog_page, parse_content_page, parse_search},
};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDebugStage { Search, BookInfo, Toc, Content }

#[derive(Debug, Clone, Serialize)]
pub struct SourceDebugStep {
    pub field: String,
    pub input_preview: String,
    pub node_count: usize,
    pub output_preview: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceDebugRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub auth_attached: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceDebugResult {
    pub source_id: i64,
    pub source_name: String,
    pub stage: SourceDebugStage,
    pub request: Option<SourceDebugRequest>,
    pub status: Option<u16>,
    pub response_headers: Vec<(String, String)>,
    pub duration_ms: u64,
    pub raw_html: String,
    pub steps: Vec<SourceDebugStep>,
    pub final_json: serde_json::Value,
    pub session_state: String,
    pub error: Option<String>,
}

impl SourceDebugStage {
    fn label(&self) -> &'static str {
        match self { Self::Search => "搜索", Self::BookInfo => "详情", Self::Toc => "目录", Self::Content => "正文" }
    }
}

pub struct SourceDebugService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl SourceDebugService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { sources: SqliteSourceRepository::new(pool.clone()), settings: SettingsService::new(pool) }
    }

    pub async fn run(&self, source_id: i64, stage: SourceDebugStage, input: &str) -> Result<SourceDebugResult, AppError> {
        let source = self.sources.get(source_id).await?.ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let (url, method, body) = request_for_stage(&source, &stage, input)?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let builder = source_request_with_method(&client, &url, &source, method.clone(), body.clone())?;
        let request = builder.build().map_err(AppError::network)?;
        let request_info = SourceDebugRequest {
            method: request.method().to_string(),
            url: request.url().to_string(),
            headers: request.headers().iter().map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("<非文本>").to_owned())).collect(),
            body: request.body().and_then(|body| body.as_bytes()).map(|bytes| String::from_utf8_lossy(bytes).into_owned()),
            auth_attached: request.headers().contains_key(reqwest::header::AUTHORIZATION) || request.headers().contains_key(reqwest::header::COOKIE),
        };
        let started = Instant::now();
        let response = client.execute(request).await.map_err(AppError::network)?;
        let status = response.status().as_u16();
        let response_headers = response.headers().iter().map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("<非文本>").to_owned())).collect();
        let raw = response.text().await.map_err(AppError::network)?;
        let raw_html = raw.chars().take(256 * 1024).collect::<String>();
        let mut steps = Vec::new();
        let parsed = parse_stage(&source, &stage, &raw_html, &mut steps);
        let (final_json, error) = match parsed {
            Ok(value) => (value, None),
            Err(error) => (serde_json::Value::Null, Some(error.to_string())),
        };
        Ok(SourceDebugResult { source_id, source_name: source.name.clone(), stage, request: Some(request_info), status: Some(status), response_headers, duration_ms: started.elapsed().as_millis() as u64, raw_html, steps, final_json, session_state: source.session_state().into(), error })
    }

    pub async fn update_rules(&self, source_id: i64, rules: RawSourceRules) -> Result<(), AppError> {
        self.sources.update_raw_rules(source_id, &rules).await
    }
}

fn request_for_stage(source: &BookSource, stage: &SourceDebugStage, input: &str) -> Result<(String, reqwest::Method, Option<String>), AppError> {
    let input = input.trim();
    match stage {
        SourceDebugStage::Search => Ok((resolve_url(&source.base_url, &source.search_url.replace("{{key}}", input).replace("{key}", input), "搜索 URL")?.to_string(), reqwest::Method::GET, None)),
        _ => Ok((resolve_url(&source.base_url, input, &format!("{} URL", stage.label()))?.to_string(), reqwest::Method::GET, None)),
    }
}

fn preview(values: impl IntoIterator<Item = String>) -> String { values.into_iter().collect::<Vec<_>>().join("\\n").chars().take(500).collect() }

fn parse_stage(source: &BookSource, stage: &SourceDebugStage, html: &str, steps: &mut Vec<SourceDebugStep>) -> Result<serde_json::Value, AppError> {
    let result = match stage {
        SourceDebugStage::Search => serde_json::to_value(parse_search(source, html)?).map_err(AppError::parse)?,
        SourceDebugStage::BookInfo => serde_json::to_value(parse_book_info(source, html)?).map_err(AppError::parse)?,
        SourceDebugStage::Toc => { let (items, next) = parse_catalog_page(source, html)?; serde_json::json!({"chapters": items, "next_url": next}) },
        SourceDebugStage::Content => { let (content, next) = parse_content_page(source, html)?; serde_json::json!({"content": content, "next_url": next}) },
    };
    steps.push(SourceDebugStep { field: stage.label().into(), input_preview: html.chars().take(500).collect(), node_count: result.as_array().map_or(1, |v| v.len()), output_preview: preview([result.to_string()]), error: None });
    Ok(result)
}
