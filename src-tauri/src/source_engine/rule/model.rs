use regex::Regex;
use std::collections::HashMap;

pub type RuleAlternatives = Vec<Vec<SourceRule>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleMode {
    Default,
    XPath,
    Json,
    Js,
    Regex,
    WebJs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleJoin {
    Chain,
    Concat,
    Interleave,
}

#[derive(Clone, Debug)]
pub struct RuleReplacement {
    pub pattern: Regex,
    pub value: String,
    pub first_only: bool,
}

#[derive(Clone, Debug)]
pub struct SourceRule {
    pub mode: RuleMode,
    pub rule: String,
    pub join: RuleJoin,
    pub reverse: bool,
    pub replace: Option<RuleReplacement>,
    pub put: HashMap<String, String>,
    pub get: Vec<String>,
    pub templates: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RuleContext {
    values: HashMap<String, String>,
}

impl RuleContext {
    pub fn new(values: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum RuleParseError {
    #[error("rule contains an unclosed {0}")]
    Unclosed(&'static str),
    #[error("rule contains an empty branch around {0}")]
    EmptyBranch(&'static str),
    #[error("invalid replacement regex: {0}")]
    InvalidRegex(String),
    #[error("invalid @put object: {0}")]
    InvalidPut(String),
}
