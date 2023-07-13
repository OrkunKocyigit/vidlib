// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use crate::filescan::FolderInfo;
use crate::service::Response;

mod filescan;
mod gui;
mod service;

#[tauri::command]
async fn file_scan(path: String) -> Result<Response<FolderInfo>, ()> {
    gui::file_scan(path).await
}

#[tauri::command]
fn select_folder() -> Result<Response<PathBuf>, ()> {
    gui::select_folder()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_scan, select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
