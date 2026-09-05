use crate::error::AppError;
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Credentials and request defaults exposed to a source's JavaScript rules.
/// Network access is deliberately only available through these injected
/// functions; the QuickJS sandbox has no filesystem, process, or environment
/// access.
#[derive(Clone, Debug, Default)]
pub struct JsHttpContext {
    pub base_url: String,
    pub headers: Option<String>,
    pub access_token: Option<String>,
    pub session_cookie: Option<String>,
    pub session_expired: bool,
    pub sign_script: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct JsContext {
    pub result: String,
    pub url: Option<String>,
    pub key: Option<String>,
    pub base_url: Option<String>,
    pub variables: HashMap<String, String>,
    pub http: Option<JsHttpContext>,
    /// Legado aliases exposed by AnalyzeRule.evalJS.
    pub title: Option<String>,
    pub src: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Json(JsonValue),
}

#[async_trait]
pub trait JsRuntime: Send + Sync {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError>;
}
