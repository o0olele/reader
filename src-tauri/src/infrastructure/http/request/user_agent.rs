//! Shared browser/HTTP identity and client hints.
use std::sync::RwLock;

/// Keep the browser challenge window and every follow-up HTTP request on one
/// User-Agent. Cloudflare binds its clearance cookie to the UA, so any
/// mismatch between the embedded webview and reqwest invalidates the cookie
/// the user just spent a captcha earning.
///
/// Only a fallback for the first launch, before the frontend has reported what
/// the embedded webview actually is. Forcing a made-up UA onto the webview is
/// the wrong direction — the webview keeps sending its own real client hints
/// regardless, so a forced UA just recreates the mismatch one layer down.
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

static USER_AGENT_OVERRIDE: RwLock<Option<String>> = RwLock::new(None);
static DETECTED_USER_AGENT: RwLock<Option<String>> = RwLock::new(None);

fn read(slot: &'static RwLock<Option<String>>) -> Option<String> {
    slot.read().ok().and_then(|value| value.clone())
}

fn write(slot: &'static RwLock<Option<String>>, value: Option<&str>) {
    let value = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    if let Ok(mut slot) = slot.write() {
        *slot = value;
    }
}

/// The User-Agent shared by the auth webview, the rule engine's HTTP calls and
/// every service client. Process-global on purpose: these live in unrelated
/// modules and a per-caller value is exactly the drift this is here to prevent.
///
/// Precedence: explicit user setting, then whatever the embedded webview
/// reported about itself, then the compiled-in fallback.
pub fn user_agent() -> String {
    resolve_user_agent(read(&USER_AGENT_OVERRIDE), read(&DETECTED_USER_AGENT))
}

/// Split out from [`user_agent`] so the precedence can be asserted without
/// writing to process-global state that the other tests read concurrently.
pub(super) fn resolve_user_agent(configured: Option<String>, detected: Option<String>) -> String {
    configured
        .or(detected)
        .unwrap_or_else(|| DEFAULT_USER_AGENT.to_owned())
}

/// The explicit user setting, if any. The auth window consults this to decide
/// whether to override the webview's own UA — when the user has *not* set one,
/// leaving the webview alone is what keeps its UA and its client hints
/// mutually consistent.
pub fn user_agent_override() -> Option<String> {
    read(&USER_AGENT_OVERRIDE)
}

/// Installs the user-configured User-Agent. `None` or blank restores the
/// detected/default value. Called at startup and whenever settings are saved.
pub fn set_user_agent(value: Option<&str>) {
    write(&USER_AGENT_OVERRIDE, value);
}

/// Installs the `navigator.userAgent` the main window reported. This is the
/// real WebView2/WebKitGTK identity that the auth window will present to
/// Cloudflare, so replaying it verbatim from reqwest is what makes the
/// clearance cookie survive the handoff.
pub fn set_detected_user_agent(value: Option<&str>) {
    write(&DETECTED_USER_AGENT, value);
}

/// Chrome sends the three low-entropy client hints on every secure request, so
/// claiming to be Chrome while sending none is itself an inconsistency. The
/// high-entropy hints (`-Arch`, `-Bitness`, `-Full-Version-List`, …) are only
/// sent *after* the server asks via `Accept-CH`; volunteering them unprompted
/// is a bot signal, which is why they are deliberately absent here.
///
/// Returns nothing for a non-Chrome UA — the hints must agree with whatever UA
/// is actually configured rather than with a hardcoded Chrome build.
pub(super) fn client_hints(user_agent: &str) -> Option<[(&'static str, String); 3]> {
    let major = user_agent
        .split("Chrome/")
        .nth(1)?
        .split(['.', ' '])
        .next()
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))?;
    // Android must be tested before Linux: Android UAs contain both.
    let platform = if user_agent.contains("Android") {
        "Android"
    } else if user_agent.contains("Windows") {
        "Windows"
    } else if user_agent.contains("Macintosh") || user_agent.contains("Mac OS X") {
        "macOS"
    } else if user_agent.contains("Linux") || user_agent.contains("X11") {
        "Linux"
    } else {
        return None;
    };
    let mobile = if user_agent.contains("Mobile") {
        "?1"
    } else {
        "?0"
    };
    // WebView2 identifies as Edge, and Edge's hints say so. Claiming Chrome
    // next to an `Edg/` UA is the same class of contradiction this function
    // exists to remove.
    let brand = if user_agent.contains("Edg/") {
        "Microsoft Edge"
    } else {
        "Google Chrome"
    };
    Some([
        (
            "Sec-CH-UA",
            format!(r#""{brand}";v="{major}", "Chromium";v="{major}", "Not_A Brand";v="24""#),
        ),
        ("Sec-CH-UA-Mobile", mobile.to_owned()),
        ("Sec-CH-UA-Platform", format!("\"{platform}\"")),
    ])
}
