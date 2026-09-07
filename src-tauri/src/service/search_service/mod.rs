//! Source search orchestration, single-source probing, and browser-bound retries.

mod browser;
mod grouping;
mod probe;
mod request;
mod search;
mod types;

pub(crate) use browser::{
    browser_body_looks_like_challenge, browser_request, navigate_browser_to_challenge,
};
pub use grouping::SearchResultGroup;
pub use types::{SearchResponse, SourceTestResult};

use crate::{repository::source::SqliteSourceRepository, service::settings_service::SettingsService};

#[derive(Clone)]
pub struct SearchService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}
