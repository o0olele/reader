//! Legado rule parsing primitives.

mod analyzer;
mod directive;
mod model;
mod scanner;

pub use analyzer::{expand_template, split_rule};
pub use model::{
    RuleAlternatives, RuleContext, RuleJoin, RuleMode, RuleParseError, RuleReplacement, SourceRule,
};
