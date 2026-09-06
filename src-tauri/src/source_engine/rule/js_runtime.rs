//! Sandboxed JavaScript execution for legado `<js>` rules.
mod bindings;
mod chapter_numbers;
mod implicit;
mod normalize;
mod request_options;
mod runtime;
mod script;
mod statements;
mod time;
mod transport;
mod types;

use crate::error::AppError;
pub use runtime::QuickJsRuntime;
pub use types::{JsContext, JsHttpContext, JsRuntime, JsValue};

fn js_error(error: impl std::fmt::Display) -> AppError {
    AppError::Source(format!("JavaScript 执行失败: {error}"))
}

#[cfg(test)]
#[path = "js_runtime/tests.rs"]
mod tests;
