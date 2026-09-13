//! The database-backed half of the full-text search rules: which chapters are
//! walked, what the progress callback sees, and how a scan is cancelled.
//!
//! Every test uses its own book id. Scan generations live in a process-wide map
//! keyed by book id — that is exactly how a newer search cancels the one in
//! flight — so two tests sharing an id would cancel each other when they run in
//! parallel.

use super::*;
use crate::{domain::replace_rule::ReplaceRule, service::replace_rule_service::ReplaceRuleService};
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
async fn search_scans_only_chapters_with_a_body_and_reports_progress() {
    let pool = pool().await;
    sqlx::query("INSERT INTO books (id, title, path) VALUES (901, 'Local', 'local')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content) VALUES (1, 901, 0, 'One', '长街的灯'), (2, 901, 1, 'Two', ''), (3, 901, 2, 'Three', '长街尽头')")
        .execute(&pool)
        .await
        .unwrap();
    let service = ReaderService::new(pool);
    let mut progress = Vec::new();

    let response = service
        .search_content(901, "长街", false, |update| progress.push(update))
        .await
        .unwrap();

    assert_eq!(response.searched_chapters, 2);
    assert_eq!(response.total_chapters, 3);
    assert_eq!(response.hits.len(), 2);
    assert!(!response.cancelled && !response.truncated);
    assert_eq!(
        response
            .hits
            .iter()
            .map(|hit| hit.chapter_id)
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
    // 第二章没有正文，从未进入扫描，因此进度只有两次。
    assert_eq!(
        progress,
        vec![
            SearchContentProgress {
                scanned: 1,
                total: 2,
                hits: 1
            },
            SearchContentProgress {
                scanned: 2,
                total: 2,
                hits: 2
            },
        ]
    );
}

#[tokio::test]
async fn searching_a_non_local_book_without_a_cache_finds_nothing() {
    let pool = pool().await;
    sqlx::query("INSERT INTO books (id, title, path) VALUES (902, 'Local', 'local')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content, remote_url) VALUES (1, 902, 0, 'One', '', 'https://example.test/1')")
        .execute(&pool)
        .await
        .unwrap();
    let service = ReaderService::new(pool);

    let response = service.search_content(902, "长街", false, |_| {}).await.unwrap();

    assert!(response.hits.is_empty());
    assert_eq!((response.searched_chapters, response.total_chapters), (0, 1));
}

#[tokio::test]
async fn hits_address_the_displayed_text_not_the_raw_body() {
    let pool = pool().await;
    sqlx::query("INSERT INTO books (id, title, path) VALUES (903, 'Local', 'local')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content) VALUES (1, 903, 0, 'One', 'first\nsecond')")
        .execute(&pool)
        .await
        .unwrap();
    // 净化规则把命中词换掉：搜索走的是阅读器显示的那份正文。
    ReplaceRuleService::new(pool.clone())
        .save(ReplaceRule {
            name: "display".into(),
            pattern: "first".into(),
            replacement: "长街".into(),
            is_regex: false,
            ..Default::default()
        })
        .await
        .unwrap();
    let service = ReaderService::new(pool);

    let displayed = service.search_content(903, "长街", false, |_| {}).await.unwrap();
    let hidden = service.search_content(903, "first", false, |_| {}).await.unwrap();

    assert_eq!(displayed.hits.len(), 1);
    assert_eq!(displayed.hits[0].paragraph_index, Some(0));
    assert_eq!(displayed.hits[0].match_offset, 0);
    assert!(hidden.hits.is_empty());
}

#[tokio::test]
async fn an_empty_query_is_rejected_instead_of_matching_everything() {
    let service = ReaderService::new(pool().await);
    assert!(service
        .search_content(904, "   ", false, |_| {})
        .await
        .is_err());
}

#[tokio::test]
async fn an_overlong_query_is_rejected() {
    let service = ReaderService::new(pool().await);
    assert!(service
        .search_content(905, &"长".repeat(121), false, |_| {})
        .await
        .is_err());
}

#[tokio::test]
async fn a_newer_search_cancels_the_scan_in_flight() {
    let pool = pool().await;
    sqlx::query("INSERT INTO books (id, title, path) VALUES (906, 'Local', 'local')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content) VALUES (1, 906, 0, 'One', '长街'), (2, 906, 1, 'Two', '长街')")
        .execute(&pool)
        .await
        .unwrap();
    let service = ReaderService::new(pool);

    let response = service
        .search_content(906, "长街", false, |_| {
            // 第一站就到来的「新搜索」把这一次作废。
            cancel_search_content(906).unwrap();
        })
        .await
        .unwrap();

    assert!(response.cancelled);
    assert_eq!(response.searched_chapters, 1);
}
