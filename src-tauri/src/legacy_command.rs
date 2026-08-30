use crate::{
    app::AppState,
    domain::{decode_text, split_chapters, Book, Chapter, ReadingProgress},
    error::AppError,
    infrastructure::http::{
        client::build_source_client,
        request::{send_source_request, source_request},
    },
    repository::{
        progress::SqliteProgressRepository, source::SqliteSourceRepository, ProgressRepository,
    },
    service::reader_service::ReaderService,
    service::{book_service::BookService, source_service::SourceService},
    source::{BookSearchResult, BookSource, CatalogRule, InfoRule, SearchRule, SourceImport},
    source_engine::{
        import::parse_sources_json,
        selector::{parse_catalog, parse_content, parse_search},
    },
};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
    path::Path,
    sync::Arc,
};
use tauri::{AppHandle, Emitter, State};
use zip::ZipArchive;

fn pool(state: &State<'_, AppState>) -> Result<sqlx::SqlitePool, AppError> {
    state.database()
}

fn global_proxy(state: &State<'_, AppState>) -> Result<Option<String>, AppError> {
    state.proxy()
}

async fn source_by_id(state: &State<'_, AppState>, source_id: i64) -> Result<BookSource, AppError> {
    SourceService::new(pool(state)?).get(source_id).await
}

#[tauri::command]
pub async fn import_txt_book(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    let title = std::path::Path::new(&filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名书籍")
        .trim()
        .to_owned();
    if title.is_empty() || bytes.is_empty() {
        return Err("书籍名称或内容不能为空".into());
    }
    let text = decode_text(&bytes).map_err(str::to_owned)?;
    let chapters = split_chapters(&text);
    tracing::info!(target: "book", filename = %filename, chapter_count = chapters.len(), "starting TXT import");
    let db = pool(&state)?;
    if let Some(existing_id) =
        sqlx::query_scalar::<_, i64>("SELECT id FROM books WHERE path = ? LIMIT 1")
            .bind(&filename)
            .fetch_optional(&db)
            .await
            .map_err(AppError::database)?
    {
        return list_books(state)
            .await?
            .into_iter()
            .find(|book| book.id == existing_id)
            .ok_or_else(|| "已存在的书籍无法读取".into());
    }
    let mut tx = db.begin().await.map_err(AppError::database)?;
    let result = sqlx::query("INSERT INTO books (title, path, group_id) VALUES (?, ?, (SELECT id FROM bookshelf_groups WHERE name = '默认书架' LIMIT 1))").bind(&title).bind(&filename).execute(&mut *tx).await.map_err(AppError::database)?;
    let book_id = result.last_insert_rowid();
    for (number, (chapter_title, content)) in chapters.iter().enumerate() {
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (?, ?, ?, ?)")
            .bind(book_id)
            .bind(number as i64)
            .bind(chapter_title)
            .bind(content)
            .execute(&mut *tx)
            .await
            .map_err(AppError::database)?;
    }
    tx.commit().await.map_err(AppError::database)?;
    tracing::info!(target: "book", book_id, chapter_count = chapters.len(), "TXT import completed");
    list_books(state)
        .await?
        .into_iter()
        .find(|book| book.id == book_id)
        .ok_or_else(|| "导入后无法读取书籍".into())
}

fn xml_attribute(event: &quick_xml::events::BytesStart<'_>, name: &[u8]) -> Option<String> {
    event
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == name)
        .and_then(|attr| String::from_utf8(attr.value.into_owned()).ok())
}

fn zip_entry(archive: &mut ZipArchive<Cursor<Vec<u8>>>, path: &str) -> Result<Vec<u8>, AppError> {
    let mut entry = archive
        .by_name(path)
        .map_err(|e| AppError::Parse(format!("EPUB 缺少文件 {path}: {e}")))?;
    let mut output = Vec::new();
    entry.read_to_end(&mut output).map_err(AppError::io)?;
    Ok(output)
}

fn xml_text(bytes: &[u8], wanted: &[u8]) -> Option<String> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut inside = false;
    let mut value = String::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event)) if event.name().as_ref() == wanted => inside = true,
            Ok(Event::Text(event)) if inside => value.push_str(&event.decode().ok()?),
            Ok(Event::End(event)) if event.name().as_ref() == wanted => break,
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => {}
        }
        buffer.clear();
    }
    (!value.trim().is_empty()).then_some(value.trim().to_owned())
}

fn clean_xhtml(bytes: &[u8]) -> (String, String) {
    let source = String::from_utf8_lossy(bytes);
    let heading = Regex::new(r"(?is)<h[1-6][^>]*>(.*?)</h[1-6]>")
        .ok()
        .and_then(|re| re.captures(&source).map(|c| c[1].to_owned()));
    let without_scripts = Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>")
        .expect("valid XHTML regex")
        .replace_all(&source, "");
    let stripped = Regex::new(r"(?is)<[^>]+>")
        .expect("valid XHTML regex")
        .replace_all(&without_scripts, "\n");
    let content = stripped
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">");
    let content = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let title = heading
        .map(|h| {
            Regex::new(r"(?is)<[^>]+>")
                .unwrap()
                .replace_all(&h, "")
                .trim()
                .to_owned()
        })
        .unwrap_or_else(|| "正文".into());
    (title, content)
}

#[tauri::command]
pub async fn import_epub_book(
    state: State<'_, AppState>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Book, AppError> {
    if bytes.is_empty() {
        return Err("EPUB 文件不能为空".into());
    }
    let mut archive =
        ZipArchive::new(Cursor::new(bytes)).map_err(|e| format!("无法打开 EPUB: {e}"))?;
    let container = zip_entry(&mut archive, "META-INF/container.xml")?;
    let mut reader = Reader::from_reader(container.as_slice());
    let mut buffer = Vec::new();
    let mut opf_path = None;
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"rootfile" =>
            {
                opf_path = xml_attribute(&event, b"full-path");
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("EPUB container.xml 解析失败: {e}").into()),
            _ => {}
        }
        buffer.clear();
    }
    let opf_path = opf_path.ok_or("EPUB 未找到 OPF 文件")?;
    let opf = zip_entry(&mut archive, &opf_path)?;
    let title = xml_text(&opf, b"dc:title")
        .or_else(|| xml_text(&opf, b"title"))
        .unwrap_or_else(|| {
            Path::new(&filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("未命名书籍")
                .to_owned()
        });
    let author = xml_text(&opf, b"dc:creator").or_else(|| xml_text(&opf, b"creator"));
    let mut manifest = HashMap::new();
    let mut spine = Vec::new();
    let mut reader = Reader::from_reader(opf.as_slice());
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"item" =>
            {
                if let (Some(id), Some(href)) =
                    (xml_attribute(&event, b"id"), xml_attribute(&event, b"href"))
                {
                    manifest.insert(id, href);
                }
            }
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"itemref" =>
            {
                if let Some(idref) = xml_attribute(&event, b"idref") {
                    spine.push(idref);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("EPUB OPF 解析失败: {e}").into()),
            _ => {}
        }
        buffer.clear();
    }
    let base = Path::new(&opf_path).parent().unwrap_or(Path::new(""));
    let mut chapters = Vec::new();
    for id in spine {
        let Some(href) = manifest.get(&id) else {
            continue;
        };
        let path = base.join(href).to_string_lossy().replace('\\', "/");
        if let Ok(content) = zip_entry(&mut archive, &path) {
            let (chapter_title, chapter_content) = clean_xhtml(&content);
            if !chapter_content.is_empty() {
                chapters.push((chapter_title, chapter_content));
            }
        }
    }
    if chapters.is_empty() {
        return Err("EPUB 中没有可读取的正文".into());
    }
    tracing::info!(target: "book", filename = %filename, chapter_count = chapters.len(), "starting EPUB import");
    let db = pool(&state)?;
    if let Some(existing_id) =
        sqlx::query_scalar::<_, i64>("SELECT id FROM books WHERE path = ? LIMIT 1")
            .bind(&filename)
            .fetch_optional(&db)
            .await
            .map_err(AppError::database)?
    {
        return list_books(state)
            .await?
            .into_iter()
            .find(|book| book.id == existing_id)
            .ok_or_else(|| "已存在的书籍无法读取".into());
    }
    let mut tx = db.begin().await.map_err(AppError::database)?;
    let result = sqlx::query("INSERT INTO books (title, author, path, group_id) VALUES (?, ?, ?, (SELECT id FROM bookshelf_groups WHERE name = '默认书架' LIMIT 1))").bind(&title).bind(&author).bind(&filename).execute(&mut *tx).await.map_err(AppError::database)?;
    let book_id = result.last_insert_rowid();
    for (number, (chapter_title, content)) in chapters.iter().enumerate() {
        sqlx::query("INSERT INTO chapters (book_id, number, title, content) VALUES (?, ?, ?, ?)")
            .bind(book_id)
            .bind(number as i64)
            .bind(chapter_title)
            .bind(content)
            .execute(&mut *tx)
            .await
            .map_err(AppError::database)?;
    }
    tx.commit().await.map_err(AppError::database)?;
    tracing::info!(target: "book", book_id, chapter_count = chapters.len(), "EPUB import completed");
    list_books(state)
        .await?
        .into_iter()
        .find(|book| book.id == book_id)
        .ok_or_else(|| "导入后无法读取书籍".into())
}

#[tauri::command]
pub async fn list_books(state: State<'_, AppState>) -> Result<Vec<Book>, AppError> {
    BookService::new(pool(&state)?).list().await
}

#[tauri::command]
pub async fn delete_book(state: State<'_, AppState>, book_id: i64) -> Result<(), AppError> {
    BookService::new(pool(&state)?).delete(book_id).await
}

#[tauri::command]
pub async fn list_book_sources(state: State<'_, AppState>) -> Result<Vec<BookSource>, AppError> {
    SourceService::new(pool(&state)?).list().await
}

#[allow(dead_code)]
async fn legacy_list_book_sources(state: State<'_, AppState>) -> Result<Vec<BookSource>, AppError> {
    let db = pool(&state)?;
    #[derive(sqlx::FromRow)]
    struct SourceRow {
        id: i64,
        name: String,
        base_url: String,
        search_url: String,
        search_item_selector: String,
        title_selector: String,
        author_selector: Option<String>,
        cover_selector: Option<String>,
        url_selector: String,
        info_title_selector: Option<String>,
        info_author_selector: Option<String>,
        info_intro_selector: Option<String>,
        catalog_item_selector: Option<String>,
        catalog_title_selector: Option<String>,
        catalog_url_selector: Option<String>,
        content_selector: Option<String>,
        enabled: i64,
        header: Option<String>,
        login_url: Option<String>,
        login_method: String,
        login_body: Option<String>,
        token_path: Option<String>,
        access_token: Option<String>,
        session_cookie: Option<String>,
        session_expires_at: Option<String>,
        sign_script: Option<String>,
        proxy_url: Option<String>,
    }
    let rows = sqlx::query_as::<_, SourceRow>(
        "SELECT id, name, base_url, search_url, search_item_selector, title_selector, author_selector, cover_selector, url_selector, info_title_selector, info_author_selector, info_intro_selector, catalog_item_selector, catalog_title_selector, catalog_url_selector, content_selector, enabled, header, login_url, login_method, login_body, token_path, access_token, session_cookie, session_expires_at, sign_script, proxy_url FROM book_sources ORDER BY name"
    ).fetch_all(&db).await.map_err(AppError::database)?;
    rows.into_iter()
        .map(|row| {
            Ok(BookSource {
                id: row.id,
                name: row.name,
                base_url: row.base_url,
                search_url: row.search_url,
                search_rule: SearchRule {
                    item: row.search_item_selector,
                    title: row.title_selector,
                    author: row.author_selector,
                    cover: row.cover_selector,
                    url: row.url_selector,
                },
                info_rule: InfoRule {
                    title: row.info_title_selector,
                    author: row.info_author_selector,
                    intro: row.info_intro_selector,
                },
                catalog_rule: CatalogRule {
                    item: row.catalog_item_selector.unwrap_or_else(|| "a".into()),
                    title: row.catalog_title_selector.unwrap_or_else(|| "a".into()),
                    url: row
                        .catalog_url_selector
                        .unwrap_or_else(|| "a::attr(href)".into()),
                },
                content_selector: row.content_selector.unwrap_or_else(|| "body".into()),
                header: row.header,
                login_url: row.login_url,
                login_method: row.login_method,
                login_body: row.login_body,
                token_path: row.token_path,
                access_token: row.access_token,
                session_cookie: row.session_cookie,
                session_expires_at: row.session_expires_at,
                sign_script: row.sign_script,
                proxy_url: row.proxy_url,
                enabled: row.enabled != 0,
            })
        })
        .collect()
}

#[derive(serde::Deserialize)]
pub struct BookSourceInput {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
    pub search_rule: SearchRule,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub info_rule: InfoRule,
    #[serde(default = "default_catalog_rule")]
    pub catalog_rule: CatalogRule,
    #[serde(default = "default_content_selector")]
    pub content_selector: String,
    #[serde(default)]
    pub header: Option<String>,
    #[serde(default)]
    pub login_url: Option<String>,
    #[serde(default = "default_login_method")]
    pub login_method: String,
    #[serde(default)]
    pub login_body: Option<String>,
    #[serde(default)]
    pub token_path: Option<String>,
    #[serde(default)]
    pub sign_script: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
}

fn default_login_method() -> String {
    "POST".into()
}

fn default_catalog_rule() -> CatalogRule {
    CatalogRule {
        item: "a".into(),
        title: "a".into(),
        url: "a::attr(href)".into(),
    }
}
fn default_content_selector() -> String {
    "body".into()
}

#[tauri::command]
pub async fn save_book_source(
    state: State<'_, AppState>,
    input: BookSourceInput,
) -> Result<BookSource, AppError> {
    let name = input.name.trim();
    let base_url = input.base_url.trim();
    let search_url = input.search_url.trim();
    if name.is_empty() || name.len() > 80 {
        return Err("书源名称需要为 1 到 80 个字符".into());
    }
    reqwest::Url::parse(base_url).map_err(|_| "书源基础 URL 无效".to_owned())?;
    if !search_url.contains("{{key}}") && !search_url.contains("{key}") {
        return Err("搜索 URL 必须包含 {{key}} 占位符".into());
    }
    for selector in [
        &input.search_rule.item,
        &input.search_rule.title,
        &input.search_rule.url,
    ] {
        scraper::Selector::parse(selector).map_err(|_| format!("无效 CSS 选择器: {selector}"))?;
    }
    let db = pool(&state)?;
    let enabled = if input.enabled.unwrap_or(true) { 1 } else { 0 };
    let source = BookSource {
        id: 0,
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        search_url: search_url.to_owned(),
        search_rule: input.search_rule,
        info_rule: input.info_rule,
        catalog_rule: input.catalog_rule,
        content_selector: input.content_selector,
        header: input.header,
        login_url: input.login_url,
        login_method: input.login_method,
        login_body: input.login_body,
        token_path: input.token_path,
        access_token: None,
        session_cookie: None,
        session_expires_at: None,
        sign_script: input.sign_script,
        proxy_url: input.proxy_url,
        enabled: enabled != 0,
    };
    tracing::info!(target: "source", source = %source.name, "saving book source");
    let id = SqliteSourceRepository::new(db)
        .upsert(&source)
        .await
        .map_err(AppError::database)?;
    Ok(BookSource { id, ..source })
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

#[allow(dead_code)]
fn legacy_build_source_client(
    source: &BookSource,
    timeout_secs: u64,
    global_proxy: Option<&str>,
) -> Result<reqwest::Client, AppError> {
    let mut builder = reqwest::Client::builder()
        .user_agent("Reader Desktop/0.1")
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(timeout_secs));
    if let Some(proxy_url) = source
        .proxy_url
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or(global_proxy)
    {
        let proxy = reqwest::Proxy::all(proxy_url.trim())
            .map_err(|error| format!("代理 URL 无效: {error}"))?;
        builder = builder.proxy(proxy);
    }
    builder
        .build()
        .map_err(AppError::network)
}

#[derive(serde::Serialize)]
pub struct AppSettings {
    pub proxy_url: Option<String>,
}

#[tauri::command]
pub async fn get_app_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    Ok(AppSettings {
        proxy_url: global_proxy(&state)?,
    })
}

#[derive(serde::Deserialize)]
pub struct AppSettingsInput {
    pub proxy_url: Option<String>,
}

#[tauri::command]
pub async fn save_app_settings(
    state: State<'_, AppState>,
    input: AppSettingsInput,
) -> Result<AppSettings, AppError> {
    let proxy = input.proxy_url.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    });
    if let Some(value) = proxy.as_deref() {
        reqwest::Proxy::all(value).map_err(|error| format!("代理 URL 无效: {error}"))?;
    }
    let db = pool(&state)?;
    sqlx::query("INSERT INTO app_settings (key, value) VALUES ('proxy_url', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP")
        .bind(proxy.as_deref().unwrap_or("")).execute(&db).await.map_err(AppError::database)?;
    *state
        .global_proxy
        .lock()
        .map_err(|_| "代理状态锁不可用".to_owned())? = proxy.clone();
    Ok(AppSettings { proxy_url: proxy })
}

#[allow(dead_code)]
fn legacy_source_request(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
) -> Result<reqwest::RequestBuilder, AppError> {
    let referer = reqwest::Url::parse(url).ok().map(|parsed| {
        let mut origin = parsed;
        origin.set_path("/");
        origin.set_query(None);
        origin.set_fragment(None);
        origin.to_string()
    });
    let mut request = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0 Safari/537.36")
        .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9,en;q=0.8")
        .header(reqwest::header::CACHE_CONTROL, "no-cache");
    if let Some(referer) = referer {
        request = request.header(reqwest::header::REFERER, referer);
    }
    if let Some(token) = source.access_token.as_deref().filter(|v| !v.is_empty()) {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    if let Some(cookie) = source.session_cookie.as_deref().filter(|v| !v.is_empty()) {
        request = request.header(reqwest::header::COOKIE, cookie);
    }
    if let Some(raw) = source.header.as_deref() {
        let parsed = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(raw).ok();
        if let Some(headers) = parsed {
            for (name, value) in headers {
                let value = value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| value.to_string());
                request = request.header(&name, value);
            }
        } else {
            for line in raw
                .split(&['\n', '&'][..])
                .filter(|line| !line.trim().is_empty())
            {
                let Some((name, value)) = line.split_once(':') else {
                    continue;
                };
                request = request
                    .header(name.trim(), value.trim())
                    .try_clone()
                    .ok_or_else(|| format!("非法请求头: {}", name.trim()))?;
            }
        }
    }
    if let Some(script) = source.sign_script.as_deref() {
        if let Some(signature) = evaluate_sign_script(script, url) {
            request = request.header("X-Signature", signature);
        }
    }
    Ok(request)
}

#[allow(dead_code)]
async fn legacy_send_source_request(
    client: &reqwest::Client,
    url: &str,
    source: &BookSource,
) -> Result<reqwest::Response, AppError> {
    let mut last_error = String::new();
    for attempt in 0..3 {
        let request = legacy_source_request(client, url, source)
            .and_then(|builder| builder.build().map_err(AppError::network))
            .map_err(|e| format!("请求构造失败，请检查认证 Header: {e}"))?;
        match client.execute(request).await {
            Ok(response) => return Ok(response),
            Err(error) => {
                last_error = if error.is_timeout() {
                    "连接超时，请检查网络或代理".into()
                } else if error.is_connect() {
                    format!("无法连接目标站点（{}），请检查 DNS、防火墙或代理", error)
                } else if error.is_builder() {
                    format!("请求配置无效: {error}")
                } else {
                    error.to_string()
                };
                if attempt < 2 {
                    tokio::time::sleep(std::time::Duration::from_millis(250 * (attempt + 1))).await;
                }
            }
        }
    }
    Err(AppError::Network(last_error))
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

/// Small deterministic signing adapter for source definitions. Supported forms are
/// `sha256({{url}})`, `sha256({{timestamp}}:secret)` and plain header values.
#[allow(dead_code)]
fn evaluate_sign_script(script: &str, url: &str) -> Option<String> {
    let timestamp = chrono_like_timestamp();
    let expression = script
        .replace("{{url}}", url)
        .replace("{{timestamp}}", &timestamp);
    let expression = expression
        .trim()
        .trim_start_matches("return")
        .trim()
        .trim_end_matches(';')
        .trim();
    let inner = expression.strip_prefix("sha256(")?.strip_suffix(')')?;
    let mut hasher = Sha256::new();
    hasher.update(inner.as_bytes());
    Some(format!("{:x}", hasher.finalize()))
}

#[allow(dead_code)]
fn chrono_like_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
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

#[tauri::command]
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

#[tauri::command]
pub async fn clear_book_source_session(
    state: State<'_, AppState>,
    source_id: i64,
) -> Result<(), AppError> {
    let db = pool(&state)?;
    sqlx::query("UPDATE book_sources SET access_token = NULL, session_cookie = NULL, session_expires_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?").bind(source_id).execute(&db).await.map_err(AppError::database).map(|_| ())
}

#[tauri::command]
pub async fn search_books(
    state: State<'_, AppState>,
    query: String,
    source_id: Option<i64>,
) -> Result<Vec<BookSearchResult>, AppError> {
    let query = query.trim();
    tracing::info!(target: "source", query = %query, source_id = ?source_id, "starting source search");
    if query.is_empty() || query.len() > 120 {
        return Err("搜索关键词需要为 1 到 120 个字符".into());
    }
    let sources = list_book_sources(state.clone()).await?;
    let sources = sources
        .into_iter()
        .filter(|source| source.enabled && source_id.is_none_or(|id| id == source.id))
        .collect::<Vec<_>>();
    if sources.is_empty() {
        return Err("没有启用的书源，请先添加书源".into());
    }
    let global_proxy = global_proxy(&state)?;
    let shared_client = reqwest::Client::builder()
        .user_agent("Reader Desktop/0.1")
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(AppError::database)?;
    let limiter = Arc::new(tokio::sync::Semaphore::new(8));
    let jobs = sources.into_iter().map(|source| {
        let shared_client = shared_client.clone();
        let global_proxy = global_proxy.clone();
        let limiter = limiter.clone();
        let keyword = encode_query(query);
        async move {
            let result: Result<Vec<BookSearchResult>, AppError> = async {
            let client = if source
                .proxy_url
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
                || global_proxy.is_some()
            {
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
                .map_err(|e| AppError::InvalidArgument(format!("{} 搜索 URL 无效: {e}", source.name)))?;
            let _request = source_request(&client, request_url.as_str(), &source)?
                .build()
                .map_err(|e| AppError::Network(format!("{} 请求构造失败，请检查 header: {e}", source.name)))?;
            let response = send_source_request(&client, request_url.as_str(), &source)
                .await
                .map_err(|e| AppError::Network(format!("{} 请求失败: {e}", source.name)))?;
            if !response.status().is_success() {
                return Err(AppError::Network(response_error(response, &source.name).await));
            }
            let body = response
                .text()
                .await
                .map_err(|e| AppError::Network(format!("{} 响应读取失败: {e}", source.name)))?;
            parse_search(&source, &body).map_err(|e| AppError::Parse(format!("{} 解析失败: {e}", source.name)))
            }
            .await;
            result
        }
    });
    let mut all = Vec::new();
    let mut failures = Vec::new();
    for result in futures::future::join_all(jobs).await {
        match result {
            Ok(results) => all.extend(results),
            Err(error) => failures.push(error),
        }
    }
    if all.is_empty() && !failures.is_empty() {
        return Err(AppError::Source(
            failures
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("；"),
        ));
    }
    let mut seen = std::collections::HashSet::new();
    all.retain(|item| seen.insert(item.url.clone()));
    Ok(all)
}

#[derive(serde::Serialize)]
pub struct SourceTestResult {
    pub source_id: i64,
    pub source_name: String,
    pub status: u16,
    pub result_count: usize,
}

#[tauri::command]
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
    let _request = source_request(&client, request_url.as_str(), &source)?
        .build()
        .map_err(|e| format!("请求构造失败，请检查认证 Header: {e}"))?;
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

fn source_has_unsupported_rules(source: &SourceImport) -> bool {
    let rules = [
        source.search_rule.item.as_str(),
        source.search_rule.title.as_str(),
        source.search_rule.url.as_str(),
        source.catalog_rule.item.as_str(),
        source.catalog_rule.title.as_str(),
        source.catalog_rule.url.as_str(),
        source.content_selector.as_str(),
    ];
    rules.iter().any(|rule| {
        rule.contains("@XPath:")
            || rule.starts_with("@Json:")
            || rule.starts_with("$.")
            || rule.contains("<js>")
            || rule.contains("&&")
            || rule.contains("||")
            || rule.contains("##")
    })
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
            let unsupported = ["@XPath:", "@Json:", "$.", "<js>", "&&", "||", "##"]
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

#[tauri::command]
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
    Ok(SourceImportReport {
        imported,
        failed,
        partial,
    })
}

#[tauri::command]
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

#[tauri::command]
pub async fn add_online_book(
    state: State<'_, AppState>,
    result: BookSearchResult,
) -> Result<Book, AppError> {
    let db = pool(&state)?;
    if let Some(existing) =
        sqlx::query_scalar::<_, i64>("SELECT id FROM books WHERE path = ? LIMIT 1")
            .bind(&result.url)
            .fetch_optional(&db)
            .await
            .map_err(AppError::database)?
    {
        return list_books(state)
            .await?
            .into_iter()
            .find(|book| book.id == existing)
            .ok_or_else(|| "书籍已存在但无法读取".into());
    }
    let inserted = sqlx::query("INSERT INTO books (title, author, path, source_id, remote_url, group_id) VALUES (?, ?, ?, ?, ?, (SELECT id FROM bookshelf_groups WHERE name = '默认书架' LIMIT 1))").bind(&result.title).bind(&result.author).bind(&result.url).bind(result.source_id).bind(&result.url).execute(&db).await.map_err(AppError::database)?;
    list_books(state)
        .await?
        .into_iter()
        .find(|book| book.id == inserted.last_insert_rowid())
        .ok_or_else(|| "加入书架后无法读取书籍".into())
}

#[tauri::command]
pub async fn fetch_online_content(
    state: State<'_, AppState>,
    source_id: i64,
    chapter_url: String,
    chapter_id: Option<i64>,
) -> Result<String, AppError> {
    let db = pool(&state)?;
    let reader_service = ReaderService::new(db.clone());
    if let Some(chapter_id) = chapter_id {
        if let Some(cached) = reader_service
            .cached_content(chapter_id)
            .await
            .map_err(AppError::database)?
        {
            tracing::debug!(target: "reader", chapter_id, "chapter cache hit");
            return Ok(cached);
        }
        tracing::debug!(target: "reader", chapter_id, "chapter cache miss");
    }
    let source = source_by_id(&state, source_id).await?;
    let proxy = global_proxy(&state)?;
    let client = build_source_client(&source, 15, proxy.as_deref())?;
    let _request = source_request(&client, &chapter_url, &source)?
        .build()
        .map_err(|e| format!("请求构造失败，请检查 header: {e}"))?;
    let response = send_source_request(&client, &chapter_url, &source).await?;
    if !response.status().is_success() {
        return Err(format!("返回 HTTP {}", response.status()).into());
    }
    let body = response.text().await.map_err(AppError::network)?;
    let content = parse_content(&source, &body)?;
    if let Some(chapter_id) = chapter_id {
        reader_service
            .cache_content(chapter_id, &content)
            .await
            .map_err(AppError::database)?;
    }
    Ok(content)
}

async fn read_chapters(db: &sqlx::SqlitePool, book_id: i64) -> Result<Vec<Chapter>, AppError> {
    ReaderService::new(db.clone()).list_chapters(book_id).await
}

#[tauri::command]
pub async fn list_chapters(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Vec<Chapter>, AppError> {
    read_chapters(&pool(&state)?, book_id).await
}

#[tauri::command]
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
        return Err(format!("目录返回 HTTP {}", response.status()).into());
    }
    let catalog = parse_catalog(&source, &response.text().await.map_err(AppError::network)?)
        .map_err(AppError::parse)?;
    tracing::info!(target: "reader", book_id, chapter_count = catalog.len(), "catalog refreshed");
    if catalog.is_empty() {
        return Err("书源没有解析出目录".into());
    }
    ReaderService::new(db.clone())
        .replace_catalog(book_id, &catalog)
        .await?;
    let chapters = read_chapters(&db, book_id).await?;
    app.emit(
        "chapter-updated",
        serde_json::json!({ "book_id": book_id, "count": chapters.len() }),
    )
    .map_err(AppError::database)?;
    Ok(chapters)
}

#[tauri::command]
pub async fn get_reading_progress(
    state: State<'_, AppState>,
    book_id: i64,
) -> Result<Option<ReadingProgress>, AppError> {
    let db = pool(&state)?;
    SqliteProgressRepository::new(db).get(book_id).await
}

#[tauri::command]
pub async fn save_reading_progress(
    state: State<'_, AppState>,
    book_id: i64,
    chapter_id: i64,
    offset: i64,
) -> Result<(), AppError> {
    if offset < 0 {
        return Err("阅读位置不能为负数".into());
    }
    let db = pool(&state)?;
    SqliteProgressRepository::new(db)
        .save(&ReadingProgress {
            book_id,
            chapter_id,
            offset,
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::{clean_xhtml, raw_unsupported_source_names, source_request};
    use crate::source::{BookSource, CatalogRule, InfoRule, SearchRule};

    fn test_source(sign_script: Option<&str>, header: Option<&str>) -> BookSource {
        BookSource {
            id: 1,
            name: "test".into(),
            base_url: "https://example.com".into(),
            search_url: "https://example.com/?q={{key}}".into(),
            search_rule: SearchRule {
                item: ".book".into(),
                title: ".title".into(),
                author: None,
                cover: None,
                url: "a::attr(href)".into(),
            },
            info_rule: InfoRule::default(),
            catalog_rule: CatalogRule {
                item: "a".into(),
                title: "a".into(),
                url: "a::attr(href)".into(),
            },
            content_selector: "body".into(),
            header: header.map(str::to_owned),
            login_url: None,
            login_method: "POST".into(),
            login_body: None,
            token_path: None,
            access_token: None,
            session_cookie: None,
            session_expires_at: None,
            sign_script: sign_script.map(str::to_owned),
            proxy_url: None,
            enabled: true,
        }
    }

    #[test]
    fn strips_xhtml_and_uses_heading() {
        let (title, content) = clean_xhtml(b"<html><body><h1>Chapter One</h1><p>Hello&nbsp;world</p><script>bad()</script></body></html>");
        assert_eq!(title, "Chapter One");
        assert_eq!(content, "Chapter One\nHello world");
    }

    #[test]
    fn signs_requests_without_custom_headers() {
        let client = reqwest::Client::new();
        let source = test_source(Some("sha256({{url}})"), None);
        let request = source_request(&client, "https://example.com/chapter", &source)
            .unwrap()
            .build()
            .unwrap();
        assert!(request.headers().contains_key("x-signature"));
    }

    #[test]
    fn reports_legacy_rules_that_need_the_full_engine() {
        let input = r#"[{"bookSourceName":"XPath source","bookSourceUrl":"https://example.com","searchUrl":"https://example.com?q={{key}}","ruleSearch":{"bookList":"@XPath://article"}}]"#;
        let names = raw_unsupported_source_names(input);
        assert!(names.contains("XPath source"));
    }
}
