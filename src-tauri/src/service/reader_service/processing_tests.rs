use super::*;
use crate::{domain::replace_rule::ReplaceRule, service::replace_rule_service::ReplaceRuleService};

#[tokio::test]
async fn processed_reads_use_current_rules_and_preserve_local_and_cached_originals() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    sqlx::query("INSERT INTO book_sources (id, name, base_url, search_url, search_item_selector, title_selector, url_selector, rule_content) VALUES (201, 'Source', 'https://source.test', '', '', '', '', ?)")
        .bind(serde_json::json!({"replaceRegex": "##first\\nsecond##clean"}).to_string())
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO books (id, title, path, source_id) VALUES (201, 'Book', 'online', 201), (202, 'Local', 'local', NULL)")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content, remote_url) VALUES (201, 201, 0, 'Chapter', '', 'https://source.test/chapter'), (202, 202, 0, 'Chapter', 'clean', NULL)")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO chapter_contents (chapter_id, content) VALUES (201, 'first' || char(10) || 'second')")
        .execute(&pool).await.unwrap();
    let rules = ReplaceRuleService::new(pool.clone());
    let mut rule = rules
        .save(ReplaceRule {
            name: "display".into(),
            pattern: "clean".into(),
            replacement: "display".into(),
            is_regex: false,
            ..Default::default()
        })
        .await
        .unwrap();
    let reader = ReaderService::new(pool.clone());
    for id in [201, 202] {
        assert_eq!(reader.read_chapter(id).await.unwrap().content, "display");
    }
    assert_eq!(reader.read_chapter(201).await.unwrap().content, "display");
    rule.enabled = false;
    rules.save(rule).await.unwrap();
    for id in [201, 202] {
        assert_eq!(reader.read_chapter(id).await.unwrap().content, "clean");
    }
    assert_eq!(
        reader.chapters.get(201).await.unwrap().unwrap().content,
        "first\nsecond"
    );
    assert_eq!(
        reader.chapters.get(202).await.unwrap().unwrap().content,
        "clean"
    );
    sqlx::query("UPDATE book_sources SET rule_content = ? WHERE id = 201")
        .bind(serde_json::json!({"replaceRegex": "@js:result.replace('first', book.name).replace('second', chapter.title) + chapter.index"}).to_string())
        .execute(&pool).await.unwrap();
    assert_eq!(
        reader.read_chapter(201).await.unwrap().content,
        "Book\nChapter0"
    );
    sqlx::query("UPDATE book_sources SET rule_content = ? WHERE id = 201")
        .bind(serde_json::json!({"replaceRegex": "##first##{{chapter.title}}"}).to_string())
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        reader.read_chapter(201).await.unwrap().content,
        "Chapter\nsecond"
    );
}
