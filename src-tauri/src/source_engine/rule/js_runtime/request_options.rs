use crate::error::AppError;
use rquickjs::Object;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub(super) struct JsHttpRequestOptions {
    pub(super) method: String,
    pub(super) body: Option<String>,
    pub(super) headers: HashMap<String, String>,
    pub(super) timeout_ms: Option<u64>,
}

pub(super) fn parse_request_options<'js>(
    options: Object<'js>,
) -> Result<JsHttpRequestOptions, AppError> {
    let method = options
        .get::<_, Option<String>>("method")
        .map_err(|error| AppError::InvalidArgument(format!("request options.method: {error}")))?
        .unwrap_or_else(|| "GET".into());
    let body = options
        .get::<_, Option<String>>("body")
        .map_err(|error| AppError::InvalidArgument(format!("request options.body: {error}")))?;
    let timeout_ms = options
        .get::<_, Option<u64>>("timeout")
        .map_err(|error| AppError::InvalidArgument(format!("request options.timeout: {error}")))?;
    let headers = options
        .get::<_, Option<Object>>("headers")
        .map_err(|error| AppError::InvalidArgument(format!("request options.headers: {error}")))?
        .map(|headers| {
            headers
                .props::<String, String>()
                .collect::<Result<HashMap<_, _>, _>>()
                .map_err(|error| AppError::InvalidArgument(format!("request options.headers: {error}")))
        })
        .transpose()?
        .unwrap_or_default();
    Ok(JsHttpRequestOptions {
        method,
        body,
        headers,
        timeout_ms,
    })
}
