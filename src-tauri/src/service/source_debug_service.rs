//! Single-request source diagnostics for the source debugger.

use crate::{
    domain::source::{BookSource, RawSourceRules},
    error::AppError,
    infrastructure::http::{client::build_source_client, request::is_challenge_response},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::search_service::{
        browser_body_looks_like_challenge, browser_request, navigate_browser_to_challenge,
    },
    service::settings_service::SettingsService,
    source_engine::pipeline::{
        parse_book_info, parse_catalog_page, parse_content_page, parse_search,
    },
    source_engine::url::{
        build as build_url_request, decode_text, decode_text_string, prepare, send, RequestSpec,
    },
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tauri::WebviewWindow;

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
        match self {
            Self::Search => "搜索",
            Self::BookInfo => "详情",
            Self::Toc => "目录",
            Self::Content => "正文",
        }
    }
}

pub struct SourceDebugService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl SourceDebugService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    pub async fn run_with_browser(
        &self,
        source_id: i64,
        stage: SourceDebugStage,
        input: &str,
        browser: Option<WebviewWindow>,
    ) -> Result<SourceDebugResult, AppError> {
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let request_spec = request_for_stage(&source, &stage, input)?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let request = prepare(&client, &source, &request_spec)?;
        let request_info = SourceDebugRequest {
            method: request.method().to_string(),
            url: request.url().to_string(),
            headers: request
                .headers()
                .iter()
                .map(|(name, value)| {
                    (
                        name.to_string(),
                        value.to_str().unwrap_or("<非文本>").to_owned(),
                    )
                })
                .collect(),
            body: request
                .body()
                .and_then(|body| body.as_bytes())
                .map(|bytes| String::from_utf8_lossy(bytes).into_owned()),
            auth_attached: request
                .headers()
                .contains_key(reqwest::header::AUTHORIZATION)
                || request.headers().contains_key(reqwest::header::COOKIE),
        };
        let started = Instant::now();
        let response = send(&client, &source, &request_spec).await?;
        let mut status = response.status().as_u16();
        let mut response_headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    value.to_str().unwrap_or("<非文本>").to_owned(),
                )
            })
            .collect();
        let raw = decode_text(response, &request_spec, &source).await?;
        let mut raw_html = raw.chars().take(256 * 1024).collect::<String>();
        if is_cloudflare_challenge(&response_headers, &raw_html) {
            if let Some(browser) = browser.as_ref() {
                let mut browser_succeeded = false;
                if let Some((browser_status, browser_body)) =
                    browser_request(browser, &request_spec).await?
                {
                    let browser_text = decode_text_string(browser_body, &request_spec, &source)?;
                    if (200..400).contains(&browser_status)
                        && !browser_body_looks_like_challenge(&browser_text)
                    {
                        status = browser_status;
                        raw_html = browser_text.chars().take(256 * 1024).collect();
                        response_headers.clear();
                        browser_succeeded = true;
                    }
                }
                if !browser_succeeded {
                    let _ = navigate_browser_to_challenge(browser, &request_spec);
                }
            }
        }
        let mut steps = Vec::new();
        let (final_json, error) = if is_cloudflare_challenge(&response_headers, &raw_html) {
            let message = "Cloudflare challenge：当前响应不是书源页面，未执行规则。请在浏览器认证窗口完成验证后重新读取会话";
            steps.push(SourceDebugStep {
                field: "HTTP 响应".into(),
                input_preview: raw_html.chars().take(500).collect(),
                node_count: 0,
                output_preview: message.into(),
                error: Some(message.into()),
            });
            (serde_json::Value::Null, Some(message.into()))
        } else {
            let parsed = parse_stage(&source, &stage, &raw_html, &mut steps);
            match parsed {
                Ok(value) => (value, None),
                Err(error) => (serde_json::Value::Null, Some(error.to_string())),
            }
        };
        Ok(SourceDebugResult {
            source_id,
            source_name: source.name.clone(),
            stage,
            request: Some(request_info),
            status: Some(status),
            response_headers,
            duration_ms: started.elapsed().as_millis() as u64,
            raw_html,
            steps,
            final_json,
            session_state: source.session_state().into(),
            error,
        })
    }

    pub async fn update_rules(
        &self,
        source_id: i64,
        rules: RawSourceRules,
    ) -> Result<(), AppError> {
        self.sources.update_raw_rules(source_id, &rules).await
    }
}

fn is_cloudflare_challenge(headers: &[(String, String)], body: &str) -> bool {
    is_challenge_response(
        headers
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str())),
        body,
    )
}

fn request_for_stage(
    source: &BookSource,
    stage: &SourceDebugStage,
    input: &str,
) -> Result<RequestSpec, AppError> {
    let input = input.trim();
    match stage {
        SourceDebugStage::Search => {
            build_url_request(source, &source.search_url, Some(input), "搜索 URL")
        }
        _ => build_url_request(source, input, None, &format!("{} URL", stage.label())),
    }
}

fn preview(values: impl IntoIterator<Item = String>) -> String {
    values
        .into_iter()
        .collect::<Vec<_>>()
        .join("\\n")
        .chars()
        .take(500)
        .collect()
}

fn parse_stage(
    source: &BookSource,
    stage: &SourceDebugStage,
    html: &str,
    steps: &mut Vec<SourceDebugStep>,
) -> Result<serde_json::Value, AppError> {
    let result = match stage {
        SourceDebugStage::Search => {
            serde_json::to_value(parse_search(source, html)?).map_err(AppError::parse)?
        }
        SourceDebugStage::BookInfo => {
            serde_json::to_value(parse_book_info(source, html)?).map_err(AppError::parse)?
        }
        SourceDebugStage::Toc => {
            let (items, next) = parse_catalog_page(source, html)?;
            serde_json::json!({"chapters": items, "next_url": next})
        }
        SourceDebugStage::Content => {
            let (content, next) = parse_content_page(source, html)?;
            serde_json::json!({"content": content, "next_url": next})
        }
    };
    steps.push(SourceDebugStep {
        field: stage.label().into(),
        input_preview: html.chars().take(500).collect(),
        node_count: result.as_array().map_or(1, |v| v.len()),
        output_preview: preview([result.to_string()]),
        error: None,
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::is_cloudflare_challenge;

    #[test]
    fn detects_a_real_challenge_interstitial() {
        let headers = vec![
            ("cf-mitigated".into(), "challenge".into()),
            ("server".into(), "cloudflare".into()),
        ];
        assert!(is_cloudflare_challenge(
            &headers,
            "<title>Just a moment...</title>"
        ));
    }

    /// The test this replaces was named after 69shuba and asserted that a 403
    /// carrying `Server: cloudflare` is a challenge. Measured against the real
    /// site, that is wrong: 69shuba answers a 14-byte `page not found` 403 with
    /// no challenge markup at all. `Server`/`cf-ray` ride on every proxied
    /// response, so keying off them relabels every Cloudflare-fronted 403 as
    /// "solve a captcha" and buries the status text that would explain it.
    #[test]
    fn a_plain_403_from_behind_cloudflare_is_not_a_challenge() {
        let headers = vec![
            ("server".into(), "cloudflare".into()),
            ("cf-ray".into(), "a35b5e2c5a90033f-HKG".into()),
        ];
        assert!(!is_cloudflare_challenge(&headers, "page not found"));
    }

    #[test]
    fn does_not_classify_a_normal_book_page_as_challenge() {
        let headers = vec![("content-type".into(), "text/html".into())];
        assert!(!is_cloudflare_challenge(
            &headers,
            "<article>book</article>"
        ));
    }
}
