// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gstreamer::init;
use std::path::PathBuf;
use std::sync::mpsc;

use tauri::{AppHandle, Error, Manager};

use crate::database::load_database;
use crate::filescan::FolderInfo;
use crate::service::Response;
use crate::state::AppState;

mod database;
mod filescan;
mod gui;
mod service;
mod state;
mod video;

#[tauri::command]
async fn file_scan(path: String) -> Result<Response<FolderInfo>, ()> {
    gui::file_scan(path).await
}

#[tauri::command]
fn select_folder() -> Result<Response<PathBuf>, ()> {
    gui::select_folder()
}

#[tauri::command]
async fn get_folders(app_handle: AppHandle) -> Result<Response<Vec<FolderInfo>>, Error> {
    let (tx, rx) = mpsc::channel();
    tauri::async_runtime::spawn_blocking(move || {
        let state = app_handle.state::<AppState>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        let paths = database::get_paths(&db).expect("Paths not found");
        tx.send(paths).unwrap();
    })
    .await?;
    let folders = rx.recv().unwrap();
    let response = gui::get_folders(&folders).await;
    if response.is_ok() {
        Ok(response.unwrap())
    } else {
        Err(Error::AssetNotFound("error".to_string()))
    }
}

#[tauri::command]
async fn add_folder(app_handle: AppHandle, path: String) -> Result<Response<FolderInfo>, ()> {
    let path_clone = path.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let state = app_handle.state::<AppState>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        database::add_path(db, &path_clone).expect("Paths not found");
    });
    file_scan(path).await
}

fn main() {
    init().unwrap();
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            file_scan,
            select_folder,
            get_folders,
            add_folder
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
