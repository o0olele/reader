//! Entity types shared by services, repositories and the IPC layer.

pub mod book;
pub mod reader;
pub mod reading_record;
pub mod replace_rule;
pub mod source;

pub use book::{Book, BookshelfGroup};
pub use reader::{Chapter, ReadingProgress};
pub use reading_record::ReadingRecord;
