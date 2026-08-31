//! Book-source workflows: search, import, probing and authentication.

use crate::{
    error::AppError,
    infrastructure::http::{
        client::build_source_client,
        request::{response_error, send_source_request},
    },
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source::{BookSearchResult, BookSource, SourceImport},
    source_engine::{import::parse_sources_json, selector::parse_search},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

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

    pub async fn search(
        &self,
        query: &str,
        source_id: Option<i64>,
    ) -> Result<SearchResponse, AppError> {
        let query = query.trim();
        tracing::info!(target: "source", query = %query, source_id = ?source_id, "starting source search");
        if query.is_empty() || query.len() > 120 {
            return Err(AppError::InvalidArgument(
                "搜索关键词需要为 1 到 120 个字符".into(),
            ));
        }
        let sources = self
            .list()
            .await?
            .into_iter()
            .filter(|s| s.enabled && source_id.is_none_or(|id| id == s.id))
            .collect::<Vec<_>>();
        if sources.is_empty() {
            return Err(AppError::Source("没有启用的书源，请先添加书源".into()));
        }
        let searched_sources = sources.len();
        let proxy = self.settings.proxy_url().await?;
        let shared = reqwest::Client::builder()
            .user_agent("Reader Desktop/0.1")
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::limited(5))
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(AppError::network)?;
        let limiter = Arc::new(tokio::sync::Semaphore::new(8));
        let keyword = encode_query(query);
        let jobs = sources.into_iter().map(|source| {
            let shared = shared.clone();
            let proxy = proxy.clone();
            let limiter = limiter.clone();
            let keyword = keyword.clone();
            async move {
                let result =
                    search_one_source(source.clone(), keyword, shared, proxy, limiter).await;
                (source.id, source.name, result)
            }
        });
        let mut results = Vec::new();
        let mut failures = Vec::new();
        for (id, name, result) in futures::future::join_all(jobs).await {
            match result {
                Ok(found) => results.extend(found),
                Err(error) => {
                    tracing::warn!(target: "source", source = %name, error = %error, "source search failed");
                    failures.push(SourceFailure {
                        source_id: id,
                        source_name: name,
                        reason: error.to_string(),
                    });
                }
            }
        }
        let groups = group_results(results);
        tracing::info!(target: "source", groups = groups.len(), failures = failures.len(), searched_sources, "source search finished");
        Ok(SearchResponse {
            groups,
            failures,
            searched_sources,
        })
    }

    pub async fn test(&self, source_id: i64, query: &str) -> Result<SourceTestResult, AppError> {
        let source = self.get(source_id).await?;
        let url = resolve_url(
            &source,
            &expand_search_url(&source, &encode_query(query.trim())),
            "搜索 URL",
        )?;
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let response = send_source_request(&client, url.as_str(), &source).await?;
        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        let results = parse_search(&source, &response.text().await.map_err(AppError::network)?)
            .map_err(AppError::parse)?;
        Ok(SourceTestResult {
            source_id,
            source_name: source.name,
            status,
            result_count: results.len(),
        })
    }

    pub async fn import_json(&self, input: &str) -> Result<SourceImportReport, AppError> {
        let sources = parse_sources_json(input)?;
        let raw_partial = raw_unsupported_source_names(input);
        let mut report = SourceImportReport::default();
        for source in sources {
            let partial =
                raw_partial.contains(&source.name) || source_has_unsupported_rules(&source);
            match self.persist_imported_source(&source).await {
                Ok(_) => {
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
        let url = resolve_url(&source, &login_url, "登录 URL")?;
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
    async fn persist_imported_source(&self, source: &SourceImport) -> Result<i64, AppError> {
        self.upsert(&BookSource {
            id: 0,
            name: source.name.clone(),
            base_url: source.base_url.clone(),
            search_url: source.search_url.clone(),
            search_rule: source.search_rule.clone(),
            info_rule: source.info_rule.clone(),
            catalog_rule: source.catalog_rule.clone(),
            content_selector: source.content_selector.clone(),
            header: source.header.clone(),
            login_url: source.login_url.clone(),
            login_method: source.login_method.clone(),
            login_body: source.login_body.clone(),
            token_path: source.token_path.clone(),
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: source.sign_script.clone(),
            proxy_url: source.proxy_url.clone(),
            enabled: source.enabled,
        })
        .await
    }
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
#[derive(Debug, Serialize)]
pub struct SourceTestResult {
    pub source_id: i64,
    pub source_name: String,
    pub status: u16,
    pub result_count: usize,
}
#[derive(Debug, Serialize)]
pub struct SearchResultGroup {
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub sources: Vec<BookSearchResult>,
}
#[derive(Debug, Serialize)]
pub struct SourceFailure {
    pub source_id: i64,
    pub source_name: String,
    pub reason: String,
}
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub groups: Vec<SearchResultGroup>,
    pub failures: Vec<SourceFailure>,
    pub searched_sources: usize,
}

fn encode_query(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn expand_search_url(source: &BookSource, keyword: &str) -> String {
    source
        .search_url
        .replace("{{key}}", keyword)
        .replace("{key}", keyword)
        .replace("<key>", keyword)
        .replace("<searchKey>", keyword)
}
fn resolve_url(source: &BookSource, value: &str, label: &str) -> Result<reqwest::Url, AppError> {
    reqwest::Url::parse(value)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(value)))
        .map_err(|e| AppError::InvalidArgument(format!("{label}无效: {e}")))
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
async fn search_one_source(
    source: BookSource,
    keyword: String,
    shared: reqwest::Client,
    proxy: Option<String>,
    limiter: Arc<tokio::sync::Semaphore>,
) -> Result<Vec<BookSearchResult>, AppError> {
    let own = source
        .proxy_url
        .as_deref()
        .is_some_and(|v| !v.trim().is_empty())
        || proxy.is_some();
    let client = if own {
        build_source_client(&source, 15, proxy.as_deref())?
    } else {
        shared
    };
    let _permit = limiter
        .acquire_owned()
        .await
        .map_err(|_| AppError::Source("搜索并发限制器不可用".into()))?;
    let url = resolve_url(&source, &expand_search_url(&source, &keyword), "搜索 URL")?;
    let response = send_source_request(&client, url.as_str(), &source).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    parse_search(&source, &response.text().await.map_err(AppError::network)?)
        .map_err(AppError::parse)
}
fn group_key(result: &BookSearchResult) -> (String, String) {
    fn normalize(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(char::to_lowercase)
            .collect()
    }
    (
        normalize(&result.title),
        normalize(result.author.as_deref().unwrap_or_default()),
    )
}
fn group_results(results: Vec<BookSearchResult>) -> Vec<SearchResultGroup> {
    let mut groups = Vec::new();
    let mut indexes = HashMap::new();
    let mut seen = HashSet::new();
    for result in results {
        if !seen.insert((result.source_id, result.url.clone())) {
            continue;
        }
        if let Some(&index) = indexes.get(&group_key(&result)) {
            let group: &mut SearchResultGroup = &mut groups[index];
            if group.cover.is_none() {
                group.cover = result.cover.clone();
            }
            group.sources.push(result);
        } else {
            indexes.insert(group_key(&result), groups.len());
            groups.push(SearchResultGroup {
                title: result.title.clone(),
                author: result.author.clone(),
                cover: result.cover.clone(),
                sources: vec![result],
            });
        }
    }
    groups
}
const UNSUPPORTED_RULE_MARKERS: [&str; 7] = ["@XPath:", "@Json:", "$.", "<js>", "&&", "||", "##"];
fn rule_needs_full_engine(rule: &str) -> bool {
    rule.starts_with("@Json:")
        || rule.starts_with("$.")
        || ["@XPath:", "<js>", "&&", "||", "##"]
            .iter()
            .any(|marker| rule.contains(marker))
}
fn source_has_unsupported_rules(source: &SourceImport) -> bool {
    [
        source.search_rule.item.as_str(),
        source.search_rule.title.as_str(),
        source.search_rule.url.as_str(),
        source.catalog_rule.item.as_str(),
        source.catalog_rule.title.as_str(),
        source.catalog_rule.url.as_str(),
        source.content_selector.as_str(),
    ]
    .iter()
    .any(|rule| rule_needs_full_engine(rule))
}
fn raw_unsupported_source_names(input: &str) -> HashSet<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(input) else {
        return HashSet::new();
    };
    let values = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    values
        .into_iter()
        .filter_map(|value| {
            let object = value.as_object()?;
            let name = object
                .get("name")
                .or_else(|| object.get("bookSourceName"))?
                .as_str()?
                .trim()
                .to_owned();
            let encoded = serde_json::to_string(object).ok()?;
            UNSUPPORTED_RULE_MARKERS
                .iter()
                .any(|marker| encoded.contains(marker))
                .then_some(name)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(source_id: i64, title: &str, author: Option<&str>, url: &str) -> BookSearchResult {
        BookSearchResult {
            source_id,
            source_name: format!("source-{source_id}"),
            title: title.into(),
            author: author.map(str::to_owned),
            cover: None,
            url: url.into(),
        }
    }

    #[test]
    fn legado_xpath_is_reported() {
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article"}}]"#;
        assert!(raw_unsupported_source_names(input).contains("XPath source"));
    }

    #[test]
    fn a_dollar_sign_inside_a_css_rule_is_not_jsonpath() {
        assert!(!rule_needs_full_engine("a[href$=.html]"));
        assert!(rule_needs_full_engine("$.data.books"));
    }

    #[test]
    fn merges_the_same_book_found_on_several_sources() {
        let groups = group_results(vec![
            result(1, "斗破苍穹", Some("天蚕土豆"), "https://a.test/1"),
            result(2, "斗破苍穹 ", Some("天蚕土豆"), "https://b.test/9"),
        ]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].sources.len(), 2);
    }

    #[test]
    fn keeps_different_books_apart() {
        let groups = group_results(vec![
            result(1, "斗破苍穹", Some("天蚕土豆"), "https://a.test/1"),
            result(1, "武动乾坤", Some("天蚕土豆"), "https://a.test/2"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn same_title_by_a_different_author_is_a_different_book() {
        let groups = group_results(vec![
            result(1, "长安", Some("甲"), "https://a.test/1"),
            result(2, "长安", Some("乙"), "https://b.test/1"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn drops_a_url_repeated_by_the_same_source() {
        let groups = group_results(vec![
            result(1, "重复", None, "https://a.test/1"),
            result(1, "重复", None, "https://a.test/1"),
        ]);
        assert_eq!(groups[0].sources.len(), 1);
    }

    #[test]
    fn keeps_the_same_url_when_it_comes_from_different_sources() {
        let groups = group_results(vec![
            result(1, "镜像", None, "https://a.test/1"),
            result(2, "镜像", None, "https://a.test/1"),
        ]);
        assert_eq!(groups[0].sources.len(), 2);
    }

    #[test]
    fn backfills_a_missing_cover_from_a_later_source() {
        let first = result(1, "补全", Some("作者"), "https://a.test/1");
        let mut second = result(2, "补全", Some("作者"), "https://b.test/1");
        second.cover = Some("https://b.test/cover.jpg".into());
        let groups = group_results(vec![first, second]);
        assert_eq!(groups[0].cover.as_deref(), Some("https://b.test/cover.jpg"));
    }

    #[test]
    fn a_missing_author_does_not_merge_into_a_named_one() {
        let groups = group_results(vec![
            result(1, "同名", None, "https://a.test/1"),
            result(2, "同名", Some("作者"), "https://b.test/1"),
        ]);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn preserves_first_seen_order() {
        let groups = group_results(vec![
            result(1, "第二本", None, "https://a.test/2"),
            result(1, "第一本", None, "https://a.test/1"),
        ]);
        assert_eq!(
            groups.iter().map(|group| group.title.as_str()).collect::<Vec<_>>(),
            ["第二本", "第一本"]
        );
    }
}
