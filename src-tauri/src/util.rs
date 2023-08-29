use std::path::PathBuf;
use tauri::AppHandle;

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
