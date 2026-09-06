use super::template;
use crate::{
    domain::replace_rule::ReplaceRule, error::AppError,
    source_engine::rule::regex_compat::normalize_java_regex,
};
use std::time::{Duration, Instant};

pub(super) enum CompiledReplacement {
    Literal {
        pattern: String,
        value: String,
    },
    Regex {
        name: String,
        regex: Box<fancy_regex::Regex>,
        parts: Vec<template::Part>,
    },
}

impl CompiledReplacement {
    pub fn new(rule: &ReplaceRule) -> Result<Self, AppError> {
        if !rule.is_regex {
            return Ok(Self::Literal {
                pattern: rule.pattern.clone(),
                value: rule.replacement.clone(),
            });
        }
        let regex = fancy_regex::RegexBuilder::new(&normalize_java_regex(&rule.pattern))
            .backtrack_limit(100_000)
            .build()
            .map_err(|error| {
                AppError::InvalidArgument(format!("净化规则「{}」正则无效：{error}", rule.name))
            })?;
        let parts = template::parse(&rule.replacement, &regex)?;
        Ok(Self::Regex {
            name: rule.name.clone(),
            regex: Box::new(regex),
            parts,
        })
    }

    pub fn apply(&self, input: &str) -> Result<String, AppError> {
        let (name, regex, parts) = match self {
            Self::Literal { pattern, value } => return Ok(input.replace(pattern, value)),
            Self::Regex { name, regex, parts } => (name, regex, parts),
        };
        let start = Instant::now();
        let mut output = String::with_capacity(input.len());
        let mut end = 0;
        for captures in regex.captures_iter(input) {
            let captures = captures
                .map_err(|error| AppError::parse(format!("净化规则「{name}」执行失败：{error}")))?;
            if start.elapsed() > Duration::from_secs(2) {
                return Err(AppError::parse(format!("净化规则「{name}」执行超时")));
            }
            let matched = captures.get(0).expect("capture zero always exists");
            output.push_str(&input[end..matched.start()]);
            template::expand(parts, &captures, &mut output);
            end = matched.end();
        }
        output.push_str(&input[end..]);
        Ok(output)
    }
}
