//! Entity types shared by services, repositories and the IPC layer.

pub mod book;
pub mod bookmark;
pub mod download;
pub mod reader;
pub mod reading_record;
pub mod replace_rule;
pub mod search_content;
pub mod source;

pub use book::{Book, BookshelfGroup};
pub use bookmark::{Bookmark, BookmarkEntry};
pub use download::DownloadTask;
pub use reader::{Chapter, ReadingProgress};
pub use reading_record::{ReadingHistoryEntry, ReadingRecord, ReadingStats};
pub use search_content::{SearchContentHit, SearchContentProgress, SearchContentResponse};
