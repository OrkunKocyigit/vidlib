// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate derive_builder;

use std::path::PathBuf;

use gstreamer::init;
use serde::Serialize;
use tauri::{AppHandle, Error, Manager, State};

use crate::database::{get_videos, load_database};
use crate::filescan::{FolderInfo, VideoFile};
use crate::service::Response;
use crate::state::AppState;
use crate::video::VideoMetadata;

mod database;
mod filescan;
mod gui;
mod service;
mod state;
mod video;

#[tauri::command]
fn file_scan(state: State<AppState>, path: String) -> Result<Response<FolderInfo>, ()> {
    let mut guard = state.video_cache.lock().unwrap();
    let cache = guard.as_mut().unwrap();
    let response = gui::file_scan(path, cache, state.videos.lock().unwrap().as_ref().unwrap());
    cache.commit(state.db.lock().unwrap().as_ref().unwrap());
    response
}

#[tauri::command]
fn select_folder() -> Result<Response<PathBuf>, ()> {
    gui::select_folder()
}

#[tauri::command]
fn get_folders(state: State<AppState>) -> Result<Response<Vec<FolderInfo>>, Error> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().unwrap();
    let folders = database::get_paths(&db).expect("Paths not found");
    let mut cache_guard = state.video_cache.lock().unwrap();
    let cache = cache_guard.as_mut().unwrap();
    let response = gui::get_folders(
        &folders,
        cache,
        state.videos.lock().unwrap().as_ref().unwrap(),
    );
    cache.commit(db);
    if response.is_ok() {
        Ok(response.unwrap())
    } else {
        Err(Error::AssetNotFound("error".to_string()))
    }
}

#[tauri::command]
fn add_folder(state: State<AppState>, path: String) -> Result<Response<FolderInfo>, ()> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().unwrap();
    database::add_path(db, &path).expect("Paths not found");
    let mut guard = state.video_cache.lock().unwrap();
    let cache = guard.as_mut().unwrap();
    let response = gui::file_scan(path, cache, state.videos.lock().unwrap().as_ref().unwrap());
    cache.commit(db);
    response
}

#[tauri::command]
fn get_video(state: State<AppState>, mut video: VideoFile) -> Result<Response<VideoFile>, ()> {
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    gui::get_video(&mut video, videos, connection)
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
    file: VideoFile,
    rating: usize,
) -> Result<Response<usize>, ()> {
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    gui::update_rating(connection, videos, file, rating)
}

#[tauri::command]
fn set_watched(
    app: AppHandle,
    state: State<AppState>,
    file: VideoFile,
    watched: bool,
) -> Result<Response<bool>, ()> {
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    let _ = app.emit_all(
        "update_watch",
        EmitWatched {
            id: file.id.clone(),
            watched,
        },
    );
    gui::update_watched(connection, videos, file, watched)
}

#[tauri::command]
fn set_video_name(
    state: State<AppState>,
    file: VideoFile,
    name: String,
) -> Result<Response<String>, ()> {
    gui::update_name(
        state.db.lock().unwrap().as_ref().unwrap(),
        state.videos.lock().unwrap().as_mut().unwrap(),
        &file,
        &name,
    )
}

#[tauri::command]
fn open_video(video: VideoFile) -> () {
    opener::open(video.path()).unwrap();
}

#[tauri::command]
async fn get_metadata(video: VideoFile) -> Result<Response<VideoMetadata>, ()> {
    gui::get_metadata(&video).await
}

#[derive(Clone, Serialize)]
struct EmitWatched {
    id: String,
    watched: bool,
}

fn main() {
    init().unwrap();
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            videos: Default::default(),
            thumbnail_cache: Default::default(),
            video_cache: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            file_scan,
            select_folder,
            get_folders,
            add_folder,
            get_video,
            get_thumbnail,
            set_video_rating,
            set_watched,
            open_video,
            set_video_name,
            get_metadata
        ])
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            let db = load_database(&handle).expect("Load database failed");
            let videos = get_videos(&db).expect("Load videos failed");
            let thumbnails = state::get_thumbnails(&handle);
            let video_cache = state::get_video_cache(&db);
            *state.videos.lock().unwrap() = Some(videos);
            *state.db.lock().unwrap() = Some(db);
            *state.thumbnail_cache.lock().unwrap() = Some(thumbnails);
            *state.video_cache.lock().unwrap() = Some(video_cache);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
