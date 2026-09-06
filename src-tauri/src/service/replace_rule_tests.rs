use super::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[tokio::test]
async fn rules_survive_reopening_and_support_update_ordering_and_deletion() {
    let path = std::env::temp_dir().join(format!("reader-rules-{}.db", uuid::Uuid::new_v4()));
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let service = ReplaceRuleService::new(pool.clone());
    assert!(service.save(ReplaceRule::default()).await.is_err());
    let mut first = service
        .save(ReplaceRule {
            name: "first".into(),
            group: Some("group".into()),
            pattern: "ad".into(),
            scope: Some("Book".into()),
            exclude_scope: Some("Other".into()),
            scope_title: true,
            sort_order: 10,
            ..Default::default()
        })
        .await
        .unwrap();
    let second = service
        .save(ReplaceRule {
            name: "second".into(),
            pattern: "(.*)".into(),
            replacement: "$1".into(),
            sort_order: 1,
            ..Default::default()
        })
        .await
        .unwrap();
    first.enabled = false;
    first.replacement = "updated".into();
    service.save(first.clone()).await.unwrap();
    pool.close().await;
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let service = ReplaceRuleService::new(pool.clone());
    let saved = service.list().await.unwrap();
    assert_eq!(saved.len(), 2);
    assert_eq!(saved[0].id, second.id);
    assert_eq!(
        serde_json::to_value(&saved[1]).unwrap(),
        serde_json::to_value(&first).unwrap()
    );
    service.delete(first.id).await.unwrap();
    assert_eq!(service.list().await.unwrap().len(), 1);
    pool.close().await;
    std::fs::remove_file(path).unwrap();
}
