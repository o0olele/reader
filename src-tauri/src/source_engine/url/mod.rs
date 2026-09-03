//! Shared Legado-style URL construction and transport helpers.

mod encoding;
mod options;
mod parser;
mod rate_limit;
mod transport;

pub use parser::{absolutize, build, build_with_base};
pub use transport::{decode_text, fetch_bytes, prepare, send, FetchedBytes};

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub url: reqwest::Url,
    pub method: reqwest::Method,
    pub body: Option<String>,
    pub charset: Option<String>,
    pub headers: Vec<(String, String)>,
    pub origin: Option<String>,
    pub retry: usize,
    pub response_type: Option<String>,
    pub body_js: Option<String>,
}
