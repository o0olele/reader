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

#[tokio::test]
async fn list_joins_book_and_chapter_and_orders_by_most_recent() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    sqlx::query("INSERT INTO books (id, title, author, path) VALUES (1, 'Alpha', 'Author A', 'a'), (2, 'Beta', NULL, 'b')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO chapters (id, book_id, number, title, content) VALUES
         (1, 1, 0, 'One', 'body'), (2, 1, 1, 'Two', 'body'), (3, 2, 0, 'Three', 'body')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let repository = SqliteBookmarkRepository::new(pool.clone());
    for (book_id, chapter_id, offset, mode) in [
        (1, 1, 10, "scroll"),
        (1, 2, 20, "paged"),
        (2, 3, 30, "scroll"),
    ] {
        repository
            .save(&Bookmark {
                book_id,
                chapter_id,
                offset,
                mode: mode.into(),
            })
            .await
            .unwrap();
    }
    // Pin the timestamps so the ordering assertion does not depend on the
    // second-resolution default that the three saves share.
    for (chapter_id, stamp) in [
        (1, "2000-01-01 00:00:00"),
        (2, "2000-01-02 00:00:00"),
        (3, "2000-01-03 00:00:00"),
    ] {
        sqlx::query("UPDATE bookmarks SET updated_at = ? WHERE chapter_id = ?")
            .bind(stamp)
            .bind(chapter_id)
            .execute(&pool)
            .await
            .unwrap();
    }

    let entries = repository.list().await.unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry.chapter_id)
            .collect::<Vec<_>>(),
        vec![3, 2, 1],
        "newest bookmark first"
    );
    let newest = &entries[0];
    assert_eq!(newest.book_id, 2);
    assert_eq!(newest.book_title, "Beta");
    assert_eq!(newest.book_author, None);
    assert_eq!(newest.chapter_title, "Three");
    assert_eq!(newest.chapter_number, 0);
    assert_eq!(newest.offset, 30);
    assert_eq!(newest.updated_at, "2000-01-03 00:00:00");
    let middle = &entries[1];
    assert_eq!(middle.book_author.as_deref(), Some("Author A"));
    assert_eq!(middle.mode, "paged");

    // Saving again moves the bookmark to the front: the page reads as a
    // "recently bookmarked" list.
    repository
        .save(&Bookmark {
            book_id: 1,
            chapter_id: 1,
            offset: 99,
            mode: "scroll".into(),
        })
        .await
        .unwrap();
    let entries = repository.list().await.unwrap();
    assert_eq!(entries[0].chapter_id, 1);
    assert_eq!(entries[0].offset, 99);

    // A deleted chapter takes its bookmark with it through the cascade.
    sqlx::query("DELETE FROM chapters WHERE id = 2")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        repository
            .list()
            .await
            .unwrap()
            .iter()
            .map(|entry| entry.chapter_id)
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
}
