//! Legado rule parsing primitives.

mod analyzer;
mod directive;
mod evaluator;
mod model;
mod scanner;

pub use analyzer::{expand_template, split_rule};
pub use evaluator::{execute_json, execute_regex, execute_rule, execute_xpath, RuleExecutionError};
pub use model::{
    RuleAlternatives, RuleContext, RuleJoin, RuleMode, RuleParseError, RuleReplacement, SourceRule,
};
