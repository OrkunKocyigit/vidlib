// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use tauri::async_runtime::block_on;

use crate::database::load_database;
use crate::filescan::FolderInfo;
use crate::service::Response;
use crate::state::AppState;

mod database;
mod filescan;
mod gui;
mod service;
mod state;

#[tauri::command]
async fn file_scan(path: String) -> Result<Response<FolderInfo>, ()> {
    gui::file_scan(path).await
}

#[tauri::command]
fn select_folder() -> Result<Response<PathBuf>, ()> {
    gui::select_folder()
}

#[tauri::command]
async fn get_folders(app_handle: AppHandle) -> Result<Response<Vec<FolderInfo>>, ()> {
    let mut folders = Vec::new();
    block_on(async {
        let state = app_handle.state::<AppState>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        folders = database::get_paths(&db).expect("Paths not found");
    });
    gui::get_folders(&folders).await
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            file_scan,
            select_folder,
            get_folders
        ])
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
