//! Source-facing orchestration still awaiting a home in `service/`.
//!
//! What is left here is online search, source authentication, legado JSON
//! import and online chapter fetching. Everything else has moved to
//! `service/` + `repository/`; see `command/` for the IPC surface.

use crate::{
    app::AppState,
    domain::Chapter,
    error::AppError,
    infrastructure::http::{client::build_source_client, request::send_source_request},
    repository::source::SqliteSourceRepository,
    service::reader_service::ReaderService,
    service::source_service::SourceService,
    source::{BookSearchResult, BookSource, SourceImport},
    source_engine::{
        import::parse_sources_json,
        selector::{parse_catalog, parse_content, parse_search},
    },
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tauri::{AppHandle, Emitter, State};

fn pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, AppError> {
    state.database()
}

fn global_proxy(state: &State<'_, AppState>) -> Result<Option<String>, AppError> {
    state.proxy()
}

async fn source_by_id(state: &State<'_, AppState>, source_id: i64) -> Result<BookSource, AppError> {
    SourceService::new(pool(state)?).get(source_id).await
}

fn encode_query(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

async fn response_error(response: reqwest::Response, source_name: &str) -> String {
    let status = response.status();
    let detail = response.text().await.unwrap_or_default();
    let detail = detail.split_whitespace().collect::<Vec<_>>().join(" ");
    let lower = detail.to_ascii_lowercase();
    if lower.contains("just a moment") || lower.contains("cf-chl-") || lower.contains("cloudflare")
    {
        return format!("{source_name} 需要浏览器执行 JavaScript 验证（Cloudflare challenge），HTTP 客户端无法直接通过");
    }
    let detail = detail.chars().take(180).collect::<String>();
    if detail.is_empty() {
        format!("{source_name} 返回 HTTP {status}")
    } else {
        format!("{source_name} 返回 HTTP {status}: {detail}")
    }
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

#[derive(serde::Deserialize)]
pub struct SourceLoginInput {
    pub source_id: i64,
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct SourceLoginResult {
    pub source_id: i64,
    pub authenticated: bool,
    pub has_token: bool,
    pub has_cookie: bool,
}

pub async fn login_book_source(
    state: State<'_, AppState>,
    input: SourceLoginInput,
) -> Result<SourceLoginResult, AppError> {
    let db = pool(&state)?;
    let source = source_by_id(&state, input.source_id).await?;
    let login_url = source.login_url.as_deref().ok_or("该书源未配置登录 URL")?;
    let login_url = login_url
        .replace("{{username}}", &input.username)
        .replace("{{password}}", &input.password);
    let url = reqwest::Url::parse(&login_url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(&login_url)))
        .map_err(|e| format!("登录 URL 无效: {e}"))?;
    let proxy = global_proxy(&state)?;
    let client = build_source_client(&source, 20, proxy.as_deref())?;
    let body = source
        .login_body
        .as_deref()
        .unwrap_or("{\"username\":\"{{username}}\",\"password\":\"{{password}}\"}")
        .replace("{{username}}", &input.username)
        .replace("{{password}}", &input.password);
    let mut request = match source.login_method.as_str() {
        "GET" => client.get(url.clone()),
        "PUT" => client.put(url.clone()),
        _ => client.post(url.clone()),
    };
    if let Some(raw) = source.header.as_deref() {
        if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(raw)
        {
            for (name, value) in headers {
                request = request.header(name, value.as_str().unwrap_or(&value.to_string()));
            }
        }
    }
    if body.trim_start().starts_with('{') {
        request = request
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
    } else {
        request = request
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(body);
    }
    let response = request
        .send()
        .await
        .map_err(|e| format!("登录请求失败: {e}"))?;
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
            .and_then(|v| json_path(&v, path))
    });
    sqlx::query("UPDATE book_sources SET access_token = ?, session_cookie = ?, session_expires_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?").bind(&token).bind(if cookies.is_empty() { None } else { Some(cookies.clone()) }).bind(input.source_id).execute(&db).await.map_err(AppError::database)?;
    Ok(SourceLoginResult {
        source_id: input.source_id,
        authenticated: token.is_some() || !cookies.is_empty(),
        has_token: token.is_some(),
        has_cookie: !cookies.is_empty(),
    })
}

pub async fn clear_book_source_session(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    let db = pool(&state)?;
    sqlx::query("UPDATE book_sources SET access_token = NULL, session_cookie = NULL, session_expires_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?").bind(source_id).execute(&db).await.map_err(AppError::database).map(|_| ())
}

/// One book, with every source that returned it.
#[derive(serde::Serialize)]
pub struct SearchResultGroup {
    pub title: String,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub sources: Vec<BookSearchResult>,
}

/// A source that did not contribute results, and why.
#[derive(serde::Serialize)]
pub struct SourceFailure {
    pub source_id: i64,
    pub source_name: String,
    pub reason: String,
}

/// Search never fails just because some sources did: partial results and the
/// per-source failures are always reported together.
#[derive(serde::Serialize)]
pub struct SearchResponse {
    pub groups: Vec<SearchResultGroup>,
    pub failures: Vec<SourceFailure>,
    pub searched_sources: usize,
}

/// Books are matched across sources on title + author, ignoring whitespace and
/// case so that "斗破苍穹" and "斗破苍穹 " collapse into one entry.
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
    let mut groups: Vec<SearchResultGroup> = Vec::new();
    let mut index_of: HashMap<(String, String), usize> = HashMap::new();
    let mut seen_urls: HashSet<(i64, String)> = HashSet::new();
    for result in results {
        // The same source listing a book twice is noise, not a second edition.
        if !seen_urls.insert((result.source_id, result.url.clone())) {
            continue;
        }
        match index_of.get(&group_key(&result)) {
            Some(&index) => {
                let group: &mut SearchResultGroup = &mut groups[index];
                // Members of a group share a title and author by construction,
                // but only some sources bother returning a cover.
                if group.cover.is_none() {
                    group.cover = result.cover.clone();
                }
                group.sources.push(result);
            }
            None => {
                index_of.insert(group_key(&result), groups.len());
                groups.push(SearchResultGroup {
                    title: result.title.clone(),
                    author: result.author.clone(),
                    cover: result.cover.clone(),
                    sources: vec![result],
                });
            }
        }
    }
    groups
}

pub async fn search_books(
    state: State<'_, AppState>,
    query: String,
    source_id: Option<i64>,
) -> Result<SearchResponse, AppError> {
    let query = query.trim();
    tracing::info!(target: "source", query = %query, source_id = ?source_id, "starting source search");
    if query.is_empty() || query.len() > 120 {
        return Err(AppError::InvalidArgument(
            "搜索关键词需要为 1 到 120 个字符".into(),
        ));
    }
    let sources = SourceService::new(pool(&state)?).list().await?;
    let sources = sources
        .into_iter()
        .filter(|source| source.enabled && source_id.is_none_or(|id| id == source.id))
        .collect::<Vec<_>>();
    if sources.is_empty() {
        return Err(AppError::Source("没有启用的书源，请先添加书源".into()));
    }
    let searched_sources = sources.len();
    let global_proxy = global_proxy(&state)?;
    let shared_client = reqwest::Client::builder()
        .user_agent("Reader Desktop/0.1")
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(AppError::network)?;
    let limiter = Arc::new(tokio::sync::Semaphore::new(8));
    let jobs = sources.into_iter().map(|source| {
        let shared_client = shared_client.clone();
        let global_proxy = global_proxy.clone();
        let limiter = limiter.clone();
        let keyword = encode_query(query);
        async move {
            let outcome = search_one_source(source.clone(), keyword, shared_client, global_proxy, limiter).await;
            (source.id, source.name, outcome)
        }
    });

    let mut results = Vec::new();
    let mut failures = Vec::new();
    for (source_id, source_name, outcome) in futures::future::join_all(jobs).await {
        match outcome {
            Ok(found) => results.extend(found),
            Err(error) => {
                tracing::warn!(target: "source", source = %source_name, error = %error, "source search failed");
                failures.push(SourceFailure {
                    source_id,
                    source_name,
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

async fn search_one_source(
    source: BookSource,
    keyword: String,
    shared_client: reqwest::Client,
    global_proxy: Option<String>,
    limiter: Arc<tokio::sync::Semaphore>,
) -> Result<Vec<BookSearchResult>, AppError> {
    // A source with its own proxy cannot share the pooled client.
    let needs_own_client = source
        .proxy_url
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        || global_proxy.is_some();
    let client = if needs_own_client {
        build_source_client(&source, 15, global_proxy.as_deref())?
    } else {
        shared_client
    };
    let _permit = limiter
        .acquire_owned()
        .await
        .map_err(|_| AppError::Source("搜索并发限制器不可用".into()))?;
    let url = source
        .search_url
        .replace("{{key}}", &keyword)
        .replace("{key}", &keyword)
        .replace("<key>", &keyword)
        .replace("<searchKey>", &keyword);
    let request_url = reqwest::Url::parse(&url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(&url)))
        .map_err(|e| AppError::InvalidArgument(format!("搜索 URL 无效: {e}")))?;
    let response = send_source_request(&client, request_url.as_str(), &source).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    let body = response.text().await.map_err(AppError::network)?;
    parse_search(&source, &body).map_err(AppError::parse)
}

#[derive(serde::Serialize)]
pub struct SourceTestResult {
    pub source_id: i64,
    pub source_name: String,
    pub status: u16,
    pub result_count: usize,
}

pub async fn test_book_source(
    state: State<'_, AppState>,
    source_id: i64,
    query: String,
) -> Result<SourceTestResult, AppError> {
    let source = source_by_id(&state, source_id).await?;
    let keyword = encode_query(query.trim());
    let url = source
        .search_url
        .replace("{{key}}", &keyword)
        .replace("{key}", &keyword)
        .replace("<key>", &keyword)
        .replace("<searchKey>", &keyword);
    let request_url = reqwest::Url::parse(&url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(&url)))
        .map_err(|e| format!("搜索 URL 无效: {e}"))?;
    let proxy = global_proxy(&state)?;
    let client = build_source_client(&source, 15, proxy.as_deref())?;
    let response = send_source_request(&client, request_url.as_str(), &source).await?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    let results = parse_search(&source, &response.text().await.map_err(AppError::network)?)
        .map_err(|e| format!("解析失败: {e}"))?;
    Ok(SourceTestResult {
        source_id,
        source_name: source.name,
        status,
        result_count: results.len(),
    })
}

#[derive(serde::Serialize)]
pub struct SourceImportReport {
    pub imported: usize,
    pub failed: Vec<String>,
    pub partial: Vec<String>,
}

/// Rule syntax the CSS-only engine cannot execute yet. Sources using any of
/// these import successfully but are reported as partial.
const UNSUPPORTED_RULE_MARKERS: [&str; 7] = ["@XPath:", "@Json:", "$.", "<js>", "&&", "||", "##"];

/// `@Json:` and `$.` only mean "JSONPath" at the start of a rule; elsewhere they
/// are ordinary characters (`$.` appears inside plenty of CSS attribute values).
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

/// The importer drops unsupported syntax while normalising rules, so the parsed
/// `SourceImport` alone cannot tell us what was lost. This re-reads the raw JSON
/// to flag sources whose original rules needed the full engine.
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
            let unsupported = UNSUPPORTED_RULE_MARKERS
                .iter()
                .any(|marker| encoded.contains(marker));
            unsupported.then_some(name)
        })
        .collect()
}

async fn persist_imported_source(
    db: &sqlx::SqlitePool,
    source: &SourceImport,
) -> Result<i64, AppError> {
    let model = BookSource {
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
    };
    SqliteSourceRepository::new(db.clone()).upsert(&model).await
}

pub async fn import_book_sources_json(
    state: State<'_, AppState>,
    json: String,
) -> Result<SourceImportReport, AppError> {
    let sources = parse_sources_json(&json)?;
    let raw_partial = raw_unsupported_source_names(&json);
    let db = pool(&state)?;
    let mut imported = 0;
    let mut failed = Vec::new();
    let mut partial = Vec::new();
    for source in sources {
        let has_unsupported_rules =
            raw_partial.contains(&source.name) || source_has_unsupported_rules(&source);
        match persist_imported_source(&db, &source).await {
            Ok(_) => {
                imported += 1;
                if has_unsupported_rules {
                    partial.push(source.name.clone());
                }
            }
            Err(error) => failed.push(format!("{}: {error}", source.name)),
        }
    }
    tracing::info!(target: "source", imported, failed = failed.len(), partial = partial.len(), "legado source import finished");
    Ok(SourceImportReport {
        imported,
        failed,
        partial,
    })
}

pub async fn import_book_sources_url(
    state: State<'_, AppState>,
    url: String,
) -> Result<SourceImportReport, AppError> {
    let url = reqwest::Url::parse(url.trim()).map_err(|_| "书源 URL 无效".to_owned())?;
    let client = reqwest::Client::builder()
        .user_agent("Reader Desktop/0.1")
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(AppError::network)?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("下载书源失败: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("书源 URL 返回 HTTP {}", response.status()).into());
    }
    import_book_sources_json(state, response.text().await.map_err(AppError::network)?).await
}

pub async fn fetch_online_content(
    state: State<'_, AppState>,
    source_id: i64,
    chapter_url: String,
    chapter_id: Option<i64>,
) -> Result<String, AppError> {
    let db = pool(&state)?;
    let reader_service = ReaderService::new(db.clone());
    if let Some(chapter_id) = chapter_id {
        if let Some(cached) = reader_service.cached_content(chapter_id).await? {
            tracing::debug!(target: "reader", chapter_id, "chapter cache hit");
            return Ok(cached);
        }
        tracing::debug!(target: "reader", chapter_id, "chapter cache miss");
    }
    let source = source_by_id(&state, source_id).await?;
    let proxy = global_proxy(&state)?;
    let client = build_source_client(&source, 15, proxy.as_deref())?;
    let response = send_source_request(&client, &chapter_url, &source).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    let body = response.text().await.map_err(AppError::network)?;
    let content = parse_content(&source, &body)?;
    if let Some(chapter_id) = chapter_id {
        reader_service.cache_content(chapter_id, &content).await?;
    }
    Ok(content)
}

pub async fn list_chapters(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    ReaderService::new(pool(&state)?).list_chapters(book_id).await
}

pub async fn refresh_catalog(
    app: AppHandle,
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    let db = pool(&state)?;
    let (source_id, book_url) = sqlx::query_as::<_, (Option<i64>, Option<String>)>(
        "SELECT source_id, remote_url FROM books WHERE id = ?",
    )
    .bind(book_id)
    .fetch_optional(&db)
    .await
    .map_err(AppError::database)?
    .ok_or("书籍不存在")?;
    let source_id = source_id.ok_or("本地书籍没有在线书源")?;
    let book_url = book_url.ok_or("书籍没有远程地址")?;
    let source = source_by_id(&state, source_id).await?;
    let proxy = global_proxy(&state)?;
    let client = build_source_client(&source, 15, proxy.as_deref())?;
    let request_url = reqwest::Url::parse(&book_url)
        .or_else(|_| reqwest::Url::parse(&source.base_url).and_then(|base| base.join(&book_url)))
        .map_err(|e| format!("目录 URL 无效: {e}"))?;
    let response = send_source_request(&client, request_url.as_str(), &source).await?;
    if !response.status().is_success() {
        return Err(AppError::Network(
            response_error(response, &source.name).await,
        ));
    }
    let catalog = parse_catalog(&source, &response.text().await.map_err(AppError::network)?)
        .map_err(AppError::parse)?;
    tracing::info!(target: "reader", book_id, chapter_count = catalog.len(), "catalog refreshed");
    if catalog.is_empty() {
        return Err("书源没有解析出目录".into());
    }
    let reader_service = ReaderService::new(db);
    reader_service.replace_catalog(book_id, &catalog).await?;
    let chapters = reader_service.list_chapters(book_id).await?;
    app.emit(
        "chapter-updated",
        serde_json::json!({ "book_id": book_id, "count": chapters.len() }),
    )
    .map_err(AppError::database)?;
    Ok(chapters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_legacy_rules_that_need_the_full_engine() {
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article"}}]"#;
        let names = raw_unsupported_source_names(input);
        assert!(names.contains("XPath source"));
    }

    #[test]
    fn a_dollar_sign_inside_a_css_rule_is_not_jsonpath() {
        // `$.` only means JSONPath at the start of a rule.
        assert!(!rule_needs_full_engine("a[href$=.html]"));
        assert!(rule_needs_full_engine("$.data.books"));
    }

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
    fn merges_the_same_book_found_on_several_sources() {
        let groups = group_results(vec![
            result(1, "斗破苍穹", Some("天蚕土豆"), "https://a.test/1"),
            result(2, "斗破苍穹 ", Some("天蚕土豆"), "https://b.test/9"),
        ]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].sources.len(), 2);
        assert_eq!(groups[0].title, "斗破苍穹");
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
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].sources.len(), 1);
    }

    #[test]
    fn keeps_the_same_url_when_it_comes_from_different_sources() {
        // Two sources can legitimately mirror one site.
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
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].cover.as_deref(), Some("https://b.test/cover.jpg"));
    }

    #[test]
    fn a_missing_author_does_not_merge_into_a_named_one() {
        // Safer to show two entries than to fuse two different books.
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
            groups.iter().map(|g| g.title.as_str()).collect::<Vec<_>>(),
            ["第二本", "第一本"]
        );
    }
}
