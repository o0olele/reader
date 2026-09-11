//! Tracing destination.
//!
//! Dev builds log to stdout, which is what `tauri dev` shows. Release builds
//! must not allocate a console — `main.rs` links with
//! `windows_subsystem = "windows"` — so they append to
//! `<app_data_dir>/logs/reader-desktop.log`, capped at [`MAX_LOG_BYTES`] with a
//! single `.1` generation.

use std::path::Path;

#[cfg(not(debug_assertions))]
use std::{fs, fs::File, io, sync::Mutex};

#[cfg(not(debug_assertions))]
const MAX_LOG_BYTES: u64 = 5 * 1024 * 1024;
#[cfg(not(debug_assertions))]
const FILE_NAME: &str = "reader-desktop.log";

/// `RUST_LOG` wins when set; otherwise the level stays at `info`.
pub fn init(log_dir: &Path) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let builder = tracing_subscriber::fmt().with_env_filter(filter);

    #[cfg(debug_assertions)]
    {
        let _ = log_dir;
        builder.init();
    }

    #[cfg(not(debug_assertions))]
    match open(log_dir) {
        Ok(file) => builder.with_ansi(false).with_writer(Mutex::new(file)).init(),
        // A GUI process has no console to fall back to, and logging must never
        // stop the app from starting.
        Err(_) => builder.init(),
    }
}

/// Appends to the log file, rotating to `.1` once it grows past the cap.
#[cfg(not(debug_assertions))]
fn open(log_dir: &Path) -> io::Result<File> {
    fs::create_dir_all(log_dir)?;
    let path = log_dir.join(FILE_NAME);
    let oversized = fs::metadata(&path)
        .map(|meta| meta.len() > MAX_LOG_BYTES)
        .unwrap_or(false);
    if oversized {
        let rotated = path.with_extension("log.1");
        let _ = fs::remove_file(&rotated);
        let _ = fs::rename(&path, rotated);
    }
    File::options().create(true).append(true).open(path)
}
