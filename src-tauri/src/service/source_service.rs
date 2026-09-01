//! Book-source CRUD, legado import and authentication.
//!
//! Search and source probing live in [`crate::service::search_service`].

use crate::{
    domain::source::BookSource,
    error::AppError,
    infrastructure::http::{client::build_source_client, url::resolve_url},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::{
        compat::{raw_unsupported_source_names, source_has_unsupported_rules},
        import::parse_sources_json,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SourceService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

impl SourceService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }
    pub async fn list(&self) -> Result<Vec<BookSource>, AppError> {
        self.sources.list().await
    }
    pub async fn get(&self, id: i64) -> Result<BookSource, AppError> {
        self.sources
            .get(id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))
    }
    pub async fn upsert(&self, source: &BookSource) -> Result<i64, AppError> {
        self.sources.upsert(source).await
    }

    pub async fn import_json(&self, input: &str) -> Result<SourceImportReport, AppError> {
        let sources = parse_sources_json(input)?;
        let raw_partial = raw_unsupported_source_names(input);
        let raw_rules = raw_rule_objects(input)?;
        let mut report = SourceImportReport::default();
        for source in sources {
            let partial =
                raw_partial.contains(&source.name) || source_has_unsupported_rules(&source);
            match self.upsert(&BookSource::from_import(&source)).await {
                Ok(id) => {
                    if let Some(raw) = raw_rules.get(&source.name) {
                        self.sources
                            .save_raw_rules(
                                id,
                                raw.search.as_deref(),
                                raw.book_info.as_deref(),
                                raw.toc.as_deref(),
                                raw.content.as_deref(),
                            )
                            .await?;
                    }
                    report.imported += 1;
                    if partial {
                        report.partial.push(source.name);
                    }
                }
                Err(error) => report.failed.push(format!("{}: {error}", source.name)),
            }
        }
        tracing::info!(target: "source", imported = report.imported, failed = report.failed.len(), partial = report.partial.len(), "legado source import finished");
        Ok(report)
    }

    pub async fn import_url(&self, raw_url: &str) -> Result<SourceImportReport, AppError> {
        let url = reqwest::Url::parse(raw_url.trim())
            .map_err(|_| AppError::InvalidArgument("书源 URL 无效".into()))?;
        // Deliberately plain: no cookie jar, and reqwest's default redirect
        // policy. Routing this through `build_shared_client` would change both.
        let client = reqwest::Client::builder()
            .user_agent("Reader Desktop/0.1")
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(AppError::network)?;
        let response = client.get(url).send().await.map_err(AppError::network)?;
        if !response.status().is_success() {
            return Err(format!("书源 URL 返回 HTTP {}", response.status()).into());
        }
        self.import_json(&response.text().await.map_err(AppError::network)?)
            .await
    }

    pub async fn login(&self, input: SourceLoginInput) -> Result<SourceLoginResult, AppError> {
        let source = self.get(input.source_id).await?;
        let login_url = source.login_url.as_deref().ok_or("该书源未配置登录 URL")?;
        let login_url = login_url
            .replace("{{username}}", &input.username)
            .replace("{{password}}", &input.password);
        let url = resolve_url(&source.base_url, &login_url, "登录 URL")?;
        let client = build_source_client(&source, 20, self.settings.proxy_url().await?.as_deref())?;
        let body = source
            .login_body
            .as_deref()
            .unwrap_or("{\"username\":\"{{username}}\",\"password\":\"{{password}}\"}")
            .replace("{{username}}", &input.username)
            .replace("{{password}}", &input.password);
        let mut request = match source.login_method.as_str() {
            "GET" => client.get(url),
            "PUT" => client.put(url),
            _ => client.post(url),
        };
        request = apply_headers(request, source.header.as_deref());
        request = if body.trim_start().starts_with('{') {
            request.header(reqwest::header::CONTENT_TYPE, "application/json")
        } else {
            request.header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
        }
        .body(body);
        let response = request.send().await.map_err(AppError::network)?;
        if !response.status().is_success() {
            return Err(format!("登录返回 HTTP {}", response.status()).into());
        }
        let cookies = response
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .filter_map(|v| v.split(';').next())
            .collect::<Vec<_>>()
            .join("; ");
        let response_text = response.text().await.map_err(AppError::network)?;
        let token = source.token_path.as_deref().and_then(|path| {
            serde_json::from_str::<serde_json::Value>(&response_text)
                .ok()
                .and_then(|value| json_path(&value, path))
        });
        self.sources
            .update_session(
                input.source_id,
                token.as_deref(),
                (!cookies.is_empty()).then_some(cookies.as_str()),
            )
            .await?;
        Ok(SourceLoginResult {
            source_id: input.source_id,
            authenticated: token.is_some() || !cookies.is_empty(),
            has_token: token.is_some(),
            has_cookie: !cookies.is_empty(),
        })
    }

    pub async fn clear_session(&self, source_id: i64) -> Result<(), AppError> {
        self.sources.clear_session(source_id).await
    }
}

#[derive(Default)]
struct RawRuleObjects {
    search: Option<String>,
    book_info: Option<String>,
    toc: Option<String>,
    content: Option<String>,
}

fn raw_rule_objects(
    input: &str,
) -> Result<std::collections::HashMap<String, RawRuleObjects>, AppError> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|error| AppError::Parse(format!("书源 JSON 格式无效: {error}")))?;
    let values = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    let mut result = std::collections::HashMap::new();
    for value in values {
        let Some(object) = value.as_object() else {
            continue;
        };
        let Some(name) = object
            .get("name")
            .or_else(|| object.get("bookSourceName"))
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
        else {
            continue;
        };
        let encode = |keys: &[&str]| {
            keys.iter()
                .find_map(|key| object.get(*key))
                .and_then(|rule| serde_json::to_string(rule).ok())
        };
        result.insert(
            name.to_owned(),
            RawRuleObjects {
                search: encode(&["search_rule", "ruleSearch"]),
                book_info: encode(&["info_rule", "ruleBookInfo"]),
                toc: encode(&["catalog_rule", "ruleToc"]),
                content: encode(&["content_rule", "ruleContent"]),
            },
        );
    }
    Ok(result)
}

#[derive(Debug, Default, Serialize)]
pub struct SourceImportReport {
    pub imported: usize,
    pub failed: Vec<String>,
    pub partial: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct SourceLoginInput {
    pub source_id: i64,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SourceLoginResult {
    pub source_id: i64,
    pub authenticated: bool,
    pub has_token: bool,
    pub has_cookie: bool,
}
fn apply_headers(
    mut request: reqwest::RequestBuilder,
    raw: Option<&str>,
) -> reqwest::RequestBuilder {
    if let Some(raw) = raw {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(raw)
        {
            for (name, value) in headers {
                request = request.header(
                    name,
                    value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string()),
                );
            }
        }
    }
    request
}
fn json_path(value: &serde_json::Value, path: &str) -> Option<String> {
    let mut current = value;
    for segment in path
        .trim_matches('/')
        .split(&['.', '/'][..])
        .filter(|s| !s.is_empty())
    {
        current = current.get(segment)?;
    }
    current
        .as_str()
        .map(str::to_owned)
        .or_else(|| current.as_i64().map(|v| v.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn pool() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn imports_partial_source_and_persists_it() {
        let database = pool().await;
        let service = SourceService::new(database.clone());
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article","name":".name","bookUrl":"a"},"ruleToc":{"chapterList":".chapter","chapterName":"a","chapterUrl":"a"},"ruleContent":".content"}]"#;

        let report = service.import_json(input).await.unwrap();

        assert_eq!(report.imported, 1);
        assert_eq!(report.failed, Vec::<String>::new());
        assert_eq!(report.partial, vec!["XPath source"]);
        assert_eq!(service.list().await.unwrap().len(), 1);
        let row: (String, String) =
            sqlx::query_as("SELECT rule_search, rule_content FROM book_sources WHERE name = ?")
                .bind("XPath source")
                .fetch_one(&database)
                .await
                .unwrap();
        let raw_search: serde_json::Value = serde_json::from_str(&row.0).unwrap();
        assert_eq!(raw_search["bookList"], "@XPath://article");
        assert_eq!(raw_search["bookUrl"], "a");
        assert_eq!(row.1, "\".content\"");
    }
}
