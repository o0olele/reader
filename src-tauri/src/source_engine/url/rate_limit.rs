use crate::domain::source::BookSource;
use std::{
    collections::HashMap,
    sync::OnceLock,
    time::{Duration, Instant},
};

struct Window {
    started: Instant,
    used: usize,
}

static WINDOWS: OnceLock<tokio::sync::Mutex<HashMap<String, Window>>> = OnceLock::new();

pub async fn wait(source: &BookSource) {
    let Some((limit, interval)) = source.concurrent_rate.as_deref().and_then(parse_rate) else {
        return;
    };
    let key = if source.id > 0 {
        format!("id:{}", source.id)
    } else {
        source.base_url.clone()
    };
    loop {
        let delay = {
            let windows = WINDOWS.get_or_init(|| tokio::sync::Mutex::new(HashMap::new()));
            let mut windows = windows.lock().await;
            let now = Instant::now();
            let window = windows.entry(key.clone()).or_insert(Window {
                started: now,
                used: 0,
            });
            let elapsed = now.saturating_duration_since(window.started);
            if elapsed >= interval {
                window.started = now;
                window.used = 1;
                None
            } else if window.used < limit {
                window.used += 1;
                None
            } else {
                Some(interval - elapsed)
            }
        };
        match delay {
            Some(delay) => tokio::time::sleep(delay).await,
            None => return,
        }
    }
}

fn parse_rate(raw: &str) -> Option<(usize, Duration)> {
    let raw = raw.trim();
    if raw.is_empty() || raw == "0" {
        return None;
    }
    let (limit, interval_ms): (usize, u64) = match raw.split_once('/') {
        Some((limit, interval)) => (limit.trim().parse().ok()?, interval.trim().parse().ok()?),
        None => (1, raw.parse().ok()?),
    };
    if limit == 0 || interval_ms == 0 {
        return None;
    }
    Some((
        limit.min(1_000),
        Duration::from_millis(interval_ms.min(86_400_000)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_legado_rate_spellings() {
        assert_eq!(parse_rate("5/1000"), Some((5, Duration::from_secs(1))));
        assert_eq!(parse_rate("250"), Some((1, Duration::from_millis(250))));
        assert_eq!(parse_rate("0"), None);
        assert_eq!(parse_rate("bad"), None);
    }
}
