use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("source error: {0}")]
    Source(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::Source(value)
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::Source(value.to_owned())
    }
}

impl AppError {
    pub fn database(error: impl Display) -> Self {
        Self::Database(error.to_string())
    }

    pub fn network(error: impl Display) -> Self {
        Self::Network(error.to_string())
    }

    pub fn parse(error: impl Display) -> Self {
        Self::Parse(error.to_string())
    }

    pub fn io(error: impl Display) -> Self {
        Self::Io(error.to_string())
    }

    /// Whether the remote endpoint rejected the request because credentials
    /// are missing or no longer valid. Keeping this classification on the
    /// error avoids every caller parsing localized error text.
    pub fn requires_authentication(&self) -> bool {
        matches!(self, Self::Network(message) if message.contains("HTTP 401") || message.contains("HTTP 403"))
    }
}
