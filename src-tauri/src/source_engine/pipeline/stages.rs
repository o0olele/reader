//! Stage parsers shared by the search, explore, reader and debugger flows.
//!
//! Split per stage so no file crosses the 250-line discipline
//! (ROADMAP-v3 E0 / §6.3 #1). This module is the facade the rest of the crate
//! imports from; the implementations live in the sibling files.

mod catalog;
mod content;
mod explore;
mod info;
mod search;

pub use catalog::parse_catalog_page;
pub use content::parse_content_page;
pub use explore::parse_explore;
pub use info::parse_book_info;
pub use search::{parse_search, parse_search_response};
