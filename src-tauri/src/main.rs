// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::database::load_database;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

use crate::filescan::FolderInfo;
use crate::service::Response;

mod database;
mod filescan;
mod gui;
mod service;

struct AppState {
    pub db: Mutex<Option<Connection>>,
}

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
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![file_scan, select_folder])
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            let db = load_database(&handle).expect("Load database failed");
            *state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
