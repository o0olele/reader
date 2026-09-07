//! Public types for the source debugger.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDebugStage {
    Search,
    BookInfo,
    Toc,
    Content,
}

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
    pub charset: Option<String>,
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
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Search => "搜索",
            Self::BookInfo => "详情",
            Self::Toc => "目录",
            Self::Content => "正文",
        }
    }
}
