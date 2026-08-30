//! Thin Tauri IPC adapters.
//!
//! Business logic is being migrated out of `command.rs`; these adapters keep
//! the public IPC contract structured while the legacy implementation moves.

pub mod book;
pub mod bookshelf;
pub mod reader;
pub mod search;
pub mod settings;
pub mod source;
