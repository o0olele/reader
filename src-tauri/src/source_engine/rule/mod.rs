//! Legado rule parsing and execution.

mod analyzer;
mod directive;
mod engine;
mod evaluator;
mod js_runtime;
mod jsoup;
mod model;
mod scanner;
mod step;
mod xpath;

pub use analyzer::{expand_template, split_rule};
pub use engine::{evaluate, evaluate_first, execute_alternatives};
pub use evaluator::{execute_json, execute_regex, execute_rule};
pub use js_runtime::{JsContext, JsHttpContext, JsRuntime, JsValue, QuickJsRuntime};
pub use jsoup::{execute_jsoup, Extraction};
pub use model::{
    RuleAlternatives, RuleContext, RuleExecutionError, RuleJoin, RuleMode, RuleParseError,
    RuleReplacement, SourceRule,
};
pub use xpath::execute_xpath;
