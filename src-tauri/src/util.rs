use log::LevelFilter;
use std::env;
use std::path::PathBuf;

use tauri::AppHandle;
use tauri_plugin_log::LogTarget;

#[cfg(not(debug_assertions))]
pub fn get_app_dir(app_handle: &AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist")
}

#[cfg(debug_assertions)]
pub fn get_app_dir(app_handle: &AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .map(|p| p.join("debug"))
        .expect("App data directory does not exist")
}

fn log_enabled() -> bool {
    cfg!(debug_assertions)
        || env::var("VIDLIB_LOG")
            .ok()
            .and_then(|v| v.parse::<i32>().ok())
            .map_or(false, |n| n >= 1)
}

pub fn get_log_targets() -> Vec<LogTarget> {
    if cfg!(debug_assertions) {
        vec![LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview]
    } else {
        vec![LogTarget::LogDir]
    }
}

pub(crate) fn get_log_level() -> LevelFilter {
    if log_enabled() {
        LevelFilter::Debug
    } else {
        LevelFilter::Error
    }
}
