use rquickjs::Object;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub(super) struct JsHttpRequestOptions {
    pub(super) method: String,
    pub(super) body: Option<String>,
    pub(super) headers: HashMap<String, String>,
    pub(super) timeout_ms: Option<u64>,
}

pub(super) fn parse_request_options<'js>(options: Object<'js>) -> Result<JsHttpRequestOptions, String> {
    let method = options
        .get::<_, Option<String>>("method")
        .map_err(|error| error.to_string())?
        .unwrap_or_else(|| "GET".into());
    let body = options
        .get::<_, Option<String>>("body")
        .map_err(|error| error.to_string())?;
    let timeout_ms = options
        .get::<_, Option<u64>>("timeout")
        .map_err(|error| error.to_string())?;
    let headers = options
        .get::<_, Option<Object>>("headers")
        .map_err(|error| error.to_string())?
        .map(|headers| {
            headers
                .props::<String, String>()
                .collect::<Result<HashMap<_, _>, _>>()
                .map_err(|error| error.to_string())
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
