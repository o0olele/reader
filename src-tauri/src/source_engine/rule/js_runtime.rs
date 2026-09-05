//! Sandboxed JavaScript execution for legado `<js>` rules.
use super::{engine::evaluate, jsoup::Extraction, model::RuleContext};
use crate::error::AppError;
use crate::infrastructure::http::request::{evaluate_sign_script, user_agent};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac as HmacMac};
use md5::{Digest, Md5};
use rquickjs::{context::EvalOptions, CatchResultExt, Context, Ctx, Function, Object, Runtime};
use serde_json::Value as JsonValue;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};
use uuid::Uuid;

include!("js_runtime/runtime.rs");
include!("js_runtime/bindings/ctx.rs");
include!("js_runtime/bindings/crypto.rs");
include!("js_runtime/bindings/net.rs");
include!("js_runtime/bindings/rule.rs");
