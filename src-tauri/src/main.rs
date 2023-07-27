// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::mpsc;

use gstreamer::init;
use tauri::{AppHandle, Error, Manager, State};

use crate::database::{get_videos, load_database};
use crate::filescan::{FolderInfo, VideoFile};
use crate::service::Response;
use crate::state::AppState;
use crate::video::VideoEntry;

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

#[tauri::command]
fn get_video(state: State<AppState>, mut video: VideoFile) -> Result<Response<VideoFile>, ()> {
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let response = gui::get_video(&mut video, videos, connection);
    return response;
}

#[tauri::command]
fn get_thumbnail(state: State<AppState>, video: VideoFile) -> Result<Response<Vec<PathBuf>>, ()> {
    let mut thumbnails_guard = state.thumbnail_cache.lock().unwrap();
    let thumbnails = thumbnails_guard.as_mut().unwrap();
    gui::get_thumbnail(video, thumbnails)
}

#[tauri::command]
fn set_video_rating(
    state: State<AppState>,
    video: VideoEntry,
    rating: usize,
) -> Result<Response<usize>, ()> {
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    gui::update_rating(connection, videos, video, rating)
}

fn main() {
    init().unwrap();
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            videos: Default::default(),
            thumbnail_cache: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            file_scan,
            select_folder,
            get_folders,
            add_folder,
            get_video,
            get_thumbnail,
            set_video_rating
        ])
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            let db = load_database(&handle).expect("Load database failed");
            let videos = get_videos(&db).expect("Load videos failed");
            let thumbnails = state::get_thumbnails(&handle);
            *state.videos.lock().unwrap() = Some(videos);
            *state.db.lock().unwrap() = Some(db);
            *state.thumbnail_cache.lock().unwrap() = Some(thumbnails);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
