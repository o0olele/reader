//! Online source audit harness. Runs the real search request and parser for a
//! corpus, recording request/HTTP/parse outcomes as CSV.
use reader_desktop_lib::{
    domain::source::{BookSource, CatalogRule, InfoRule, RawSourceRules, SearchRule},
    infrastructure::http::client::build_shared_client,
    source_engine::{
        pipeline::parse_search,
        url::{build, decode_text, send},
    },
};
use serde_json::Value;
use std::{env, fs, path::PathBuf};

fn text(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_owned)
}
fn rule_field(v: &Value, obj: &str, key: &str) -> Option<String> {
    let raw = v.get(obj)?;
    let parsed = raw
        .as_str()
        .and_then(|s| serde_json::from_str::<Value>(s).ok())
        .unwrap_or(raw.clone());
    parsed.get(key).and_then(Value::as_str).map(str::to_owned)
}
fn source(v: &Value, id: i64) -> BookSource {
    let base = text(v, "bookSourceUrl").unwrap_or_default();
    let search_url = text(v, "searchUrl").unwrap_or_default();
    let raw_rules = RawSourceRules {
        search: v.get("ruleSearch").map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        }),
        book_info: v.get("ruleBookInfo").map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        }),
        toc: v.get("ruleToc").map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        }),
        content: v.get("ruleContent").map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        }),
        explore: v.get("ruleExplore").map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        }),
    };
    BookSource {
        id,
        name: text(v, "bookSourceName").unwrap_or_else(|| format!("source-{id}")),
        base_url: base,
        search_url,
        explore_url: text(v, "exploreUrl"),
        search_rule: SearchRule {
            item: rule_field(v, "ruleSearch", "bookList").unwrap_or_default(),
            title: rule_field(v, "ruleSearch", "name").unwrap_or_default(),
            author: rule_field(v, "ruleSearch", "author"),
            cover: rule_field(v, "ruleSearch", "coverUrl"),
            url: rule_field(v, "ruleSearch", "bookUrl").unwrap_or_default(),
        },
        info_rule: InfoRule::default(),
        catalog_rule: CatalogRule {
            item: String::new(),
            title: String::new(),
            url: String::new(),
            next_url: None,
        },
        content_selector: String::new(),
        next_toc_url_selector: None,
        next_content_url_selector: None,
        header: text(v, "header"),
        login_url: text(v, "loginUrl"),
        login_method: "GET".into(),
        login_body: None,
        token_path: None,
        access_token: None,
        session_cookie: None,
        session_expires_at: None,
        sign_script: None,
        proxy_url: None,
        concurrent_rate: None,
        enabled: true,
        raw_rules,
    }
}
fn classify(e: &str) -> &'static str {
    let s = e.to_ascii_lowercase();
    if s.contains("超时") || s.contains("timeout") {
        "timeout"
    } else if s.contains("无法连接") || s.contains("connect") {
        "connect"
    } else if s.contains("403") || s.contains("401") {
        "auth"
    } else if s.contains("404") {
        "http_404"
    } else if s.contains("cloudflare") {
        "cloudflare"
    } else if s.contains("请求") {
        "request"
    } else {
        "parse"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let corpus = args
        .windows(2)
        .find(|w| w[0] == "--corpus")
        .map(|w| PathBuf::from(&w[1]))
        .ok_or("--corpus required")?;
    let out = args
        .windows(2)
        .find(|w| w[0] == "--out")
        .map(|w| PathBuf::from(&w[1]))
        .unwrap_or_else(|| PathBuf::from("audit.csv"));
    let keyword = args
        .windows(2)
        .find(|w| w[0] == "--keyword")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| "剑来".into());
    let workers: usize = args
        .windows(2)
        .find(|w| w[0] == "--concurrency")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(8)
        .clamp(1, 64);
    let value: Value = serde_json::from_str(&fs::read_to_string(corpus)?)?;
    let arr = value.as_array().ok_or("corpus must be array")?;
    let client = build_shared_client(15)?;
    let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(workers));
    let mut rows =
        vec!["source_id,source_name,url,stage,status,result_count,error_class,error".to_string()];
    let mut joins = Vec::new();
    for (i, v) in arr.iter().enumerate() {
        let s = source(v, i as i64);
        let c = client.clone();
        let sem = sem.clone();
        let kw = keyword.clone();
        joins.push(tokio::spawn(async move {
            let _p = sem.acquire().await.unwrap();
            let req = match build(&s, &s.search_url, Some(&kw), "search URL") {
                Ok(r) => r,
                Err(e) => return (s, String::new(), "request", 0, e.to_string()),
            };
            let url = req.url.to_string();
            let resp = match send(&c, &s, &req).await {
                Ok(r) => r,
                Err(e) => return (s, url, "network", 0, e.to_string()),
            };
            let status = resp.status().as_u16();
            if !resp.status().is_success() {
                return (s, url, "http", status, format!("HTTP {status}"));
            }
            match decode_text(resp, &req, &s)
                .await
                .and_then(|h| parse_search(&s, &h))
            {
                Ok(r) => (s, url, "ok", status, r.len().to_string()),
                Err(e) => (s, url, classify(&e.to_string()), status, e.to_string()),
            }
        }));
    }
    for j in joins {
        let (s, url, stage, status, detail) = j.await?;
        let count = detail.parse::<usize>().unwrap_or(0);
        let err = if stage == "ok" { "" } else { &detail };
        let class = if stage == "ok" { "" } else { stage };
        rows.push(format!(
            "{},{},{},{},{},{},{},{}",
            s.id,
            csv(&s.name),
            csv(&url),
            stage,
            status,
            count,
            class,
            csv(err)
        ));
    }
    fs::write(&out, rows.join("\n") + "\n")?;
    eprintln!("wrote {} rows to {}", rows.len() - 1, out.display());
    Ok(())
}
fn csv(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
