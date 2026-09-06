use super::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[tokio::test]
async fn bookmarks_survive_reopening_and_follow_chapter_lifetime() {
    let path = std::env::temp_dir().join(format!("reader-bookmark-{}.db", uuid::Uuid::new_v4()));
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    sqlx::query("INSERT INTO books (id, title, path) VALUES (1, 'One', 'one'), (2, 'Two', 'two')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chapters (id, book_id, number, title, content) VALUES (1, 1, 0, 'Chapter', 'body')")
        .execute(&pool).await.unwrap();
    let repository = SqliteBookmarkRepository::new(pool.clone());
    let mut bookmark = Bookmark {
        book_id: 1,
        chapter_id: 1,
        offset: 125,
        mode: "scroll".into(),
    };
    repository.save(&bookmark).await.unwrap();
    bookmark.offset = 500;
    bookmark.mode = "paged".into();
    repository.save(&bookmark).await.unwrap();
    bookmark.book_id = 2;
    assert!(repository.save(&bookmark).await.is_err());
    bookmark.book_id = 1;
    bookmark.offset = -1;
    assert!(repository.save(&bookmark).await.is_err());
    bookmark.offset = 500;
    bookmark.mode = "invalid".into();
    assert!(repository.save(&bookmark).await.is_err());
    pool.close().await;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let repository = SqliteBookmarkRepository::new(pool.clone());
    let saved = repository.get(1, 1).await.unwrap().unwrap();
    assert_eq!(saved.offset, 500);
    assert_eq!(saved.mode, "paged");
    repository.delete(2, 1).await.unwrap();
    assert!(repository.get(1, 1).await.unwrap().is_some());
    repository.delete(1, 1).await.unwrap();
    assert!(repository.get(1, 1).await.unwrap().is_none());
    repository.save(&saved).await.unwrap();
    sqlx::query("DELETE FROM chapters WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();
    assert!(repository.get(1, 1).await.unwrap().is_none());
    pool.close().await;
    std::fs::remove_file(path).unwrap();
}
