use crate::{domain::source::BookSource, error::AppError};

pub fn build_source_client(
    source: &BookSource,
    timeout_secs: u64,
    global_proxy: Option<&str>,
) -> Result<reqwest::Client, AppError> {
    let mut builder = reqwest::Client::builder()
        .user_agent("Reader Desktop/0.1")
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(timeout_secs));
    if let Some(proxy_url) = source
        .proxy_url
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or(global_proxy)
    {
        let proxy = reqwest::Proxy::all(proxy_url.trim())
            .map_err(|error| AppError::InvalidArgument(format!("代理 URL 无效: {error}")))?;
        builder = builder.proxy(proxy);
    }
    builder.build().map_err(AppError::network)
}
