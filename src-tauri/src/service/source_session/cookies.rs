//! Cookie header normalization shared by session loading and persistence.
use std::collections::BTreeMap;

pub(super) fn seed_cookie_jar(jar: &reqwest::cookie::Jar, scope: &str, header: Option<&str>) {
    let (Ok(url), Some(header)) = (reqwest::Url::parse(scope), header) else {
        return;
    };
    for (name, value) in cookie_map(Some(header)) {
        jar.add_cookie_str(&format!("{name}={value}"), &url);
    }
}

pub(super) fn apply_set_cookie_headers(
    cookies: &mut BTreeMap<String, String>,
    headers: &reqwest::header::HeaderMap,
) {
    for value in headers.get_all(reqwest::header::SET_COOKIE) {
        let Ok(value) = value.to_str() else { continue };
        let Some(pair) = value.split(';').next() else {
            continue;
        };
        let Some((name, cookie_value)) = pair.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        let remove = cookie_value.trim().is_empty()
            || value.split(';').any(|part| {
                part.trim()
                    .strip_prefix("Max-Age=")
                    .or_else(|| part.trim().strip_prefix("max-age="))
                    .is_some_and(|age| age.trim().parse::<i64>().is_ok_and(|age| age <= 0))
            });
        if remove {
            cookies.remove(name);
        } else {
            cookies.insert(name.to_owned(), cookie_value.trim().to_owned());
        }
    }
}

pub(super) fn cookie_map(header: Option<&str>) -> BTreeMap<String, String> {
    header
        .into_iter()
        .flat_map(|value| value.split(';'))
        .filter_map(|part| {
            let (name, value) = part.trim().split_once('=')?;
            (!name.is_empty()).then(|| (name.to_owned(), value.to_owned()))
        })
        .collect()
}

pub(super) fn cookie_header(cookies: &BTreeMap<String, String>) -> Option<String> {
    (!cookies.is_empty()).then(|| {
        cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

pub(super) fn normalized_cookie_header(header: Option<&str>) -> Option<String> {
    cookie_header(&cookie_map(header))
}
