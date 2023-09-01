// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path::PathBuf;

use gstreamer::init;
use serde::Serialize;
use tauri::{AppHandle, Error, Manager, State};

use crate::database::{get_videos, load_database};
use crate::filescan::{FolderInfo, VideoFile};
use crate::service::{Response, ResponseType};
use crate::state::{AppState, EmitTotalProgress};
use crate::video::VideoMetadata;

mod database;
mod filescan;
mod gui;
mod service;
mod state;
mod util;
mod video;

#[tauri::command]
fn file_scan(
    app: AppHandle,
    state: State<AppState>,
    path: String,
) -> Result<Response<FolderInfo>, ()> {
    debug!("File Scan Start");
    let mut guard = state.video_cache.lock().unwrap();
    let cache = guard.as_mut().unwrap();
    let total = RefCell::new(EmitTotalProgress::new());
    let emitter = |progress: EmitProgress| {
        total.borrow_mut().process(progress);
        let _ = app.emit_all("add_progress", total.borrow().deref());
    };
    let response = gui::file_scan(
        path,
        cache,
        state.videos.lock().unwrap().as_ref().unwrap(),
        emitter,
    );
    cache.commit(state.db.lock().unwrap().as_ref().unwrap());
    debug!("File Scan End");
    response
}

#[tauri::command]
fn select_folder() -> Result<Response<PathBuf>, ()> {
    debug!("Select Folder Start");
    gui::select_folder()
}

#[tauri::command]
fn get_folders(app: AppHandle, state: State<AppState>) -> Result<Response<Vec<FolderInfo>>, Error> {
    debug!("Get Folders Start");
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().unwrap();
    let folders = database::get_paths(&db).expect("Paths not found");
    let mut cache_guard = state.video_cache.lock().unwrap();
    let cache = cache_guard.as_mut().unwrap();
    let total = RefCell::new(EmitTotalProgress::new());
    let emitter = |progress: EmitProgress| {
        total.borrow_mut().process(progress);
        let _ = app.emit_all("add_progress", total.borrow().deref());
    };
    let response = gui::get_folders(
        &folders,
        cache,
        state.videos.lock().unwrap().as_ref().unwrap(),
        emitter,
    );
    cache.commit(db);
    debug!("Get Folders End");
    if response.is_ok() {
        Ok(response.unwrap())
    } else {
        Err(Error::AssetNotFound("error".to_string()))
    }
}

#[tauri::command]
fn add_folder(
    app: AppHandle,
    state: State<AppState>,
    path: String,
) -> Result<Response<FolderInfo>, ()> {
    debug!("Add Folder Start");
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().unwrap();
    database::add_path(db, &path).expect("Paths not found");
    let mut guard = state.video_cache.lock().unwrap();
    let cache = guard.as_mut().unwrap();
    let total = RefCell::new(EmitTotalProgress::new());
    let emitter = |progress: EmitProgress| {
        total.borrow_mut().process(progress);
        let _ = app.emit_all("add_progress", total.borrow().deref());
    };
    let response = gui::file_scan(
        path,
        cache,
        state.videos.lock().unwrap().as_ref().unwrap(),
        emitter,
    );
    cache.commit(db);
    debug!("Add Folder End");
    response
}

#[tauri::command]
fn get_video(state: State<AppState>, mut video: VideoFile) -> Result<Response<VideoFile>, ()> {
    debug!("Get Video Start");
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    gui::get_video(&mut video, videos, connection)
}

#[tauri::command]
fn get_thumbnail(state: State<AppState>, video: VideoFile) -> Result<Response<Vec<PathBuf>>, ()> {
    debug!("Get Thumbnails Start");
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
    debug!("Set Video Rating Start");
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    gui::update_rating(connection, videos, file, rating)
}

fn emit_folder_watched(app: AppHandle, path: &PathBuf, watched: bool) {
    if let Some(parent) = path.parent() {
        let mut hasher = DefaultHasher::new();
        parent.hash(&mut hasher);
        let id = format!("{:x}", hasher.finish());
        let event_name = format!("update_watch_{}", id);
        let _ = app.emit_all(event_name.as_str(), EmitWatched { watched });
    }
}

#[tauri::command]
fn set_watched(
    app: AppHandle,
    state: State<AppState>,
    file: VideoFile,
    watched: bool,
) -> Result<Response<bool>, ()> {
    debug!("Set Watched Start");
    let connection_guard = state.db.lock().unwrap();
    let connection = connection_guard.as_ref().unwrap();
    let mut videos_guard = state.videos.lock().unwrap();
    let videos = videos_guard.as_mut().unwrap();
    let _ = app.emit_all(
        format!("update_watch_{}", file.id.clone()).as_str(),
        EmitWatched { watched },
    );
    emit_folder_watched(app, file.path(), watched);
    gui::update_watched(connection, videos, file, watched)
}

#[tauri::command]
fn set_video_name(
    state: State<AppState>,
    file: VideoFile,
    name: String,
) -> Result<Response<String>, ()> {
    debug!("Set Video Name Start");
    gui::update_name(
        state.db.lock().unwrap().as_ref().unwrap(),
        state.videos.lock().unwrap().as_mut().unwrap(),
        &file,
        &name,
    )
}

#[tauri::command]
fn set_video_notes(
    state: State<AppState>,
    file: VideoFile,
    notes: String,
) -> Result<Response<String>, ()> {
    debug!("Set Video Notes Start");
    gui::update_notes(
        state.db.lock().unwrap().as_ref().unwrap(),
        state.videos.lock().unwrap().as_mut().unwrap(),
        &file,
        &notes,
    )
}

#[tauri::command]
fn open_video(video: VideoFile) -> Result<Response<()>, ()> {
    debug!("Open Video Start");
    gui::open_video(video)
}

#[tauri::command]
async fn get_metadata(video: VideoFile) -> Result<Response<VideoMetadata>, ()> {
    debug!("Get Metadata Start");
    gui::get_metadata(&video).await
}

#[tauri::command]
fn delete_path(app: AppHandle, state: State<AppState>, path: &str) -> Result<Response<bool>, ()> {
    debug!("Delete Path Start");
    let mut db_guard = state.db.lock().unwrap();
    let db = db_guard.as_mut().unwrap();
    if let Err(e) = gui::validate_path(&db, &path) {
        Ok(e)
    } else {
        let mut cache_guard = state.video_cache.lock().unwrap();
        let cache = cache_guard.as_mut().unwrap();
        let response = gui::delete_path(db, cache, &path);
        if response.result == ResponseType::SUCCESS {
            let _ = app.emit_all(
                "path_deleted",
                EmitPathDeleted {
                    path: path.to_string(),
                },
            );
        }
        Ok(response)
    }
}

#[tauri::command]
fn open_path(path: &str, parent: bool) {
    debug!("Open Path Start");
    let path_buf = PathBuf::from(path);
    let path = if parent {
        let parent_path = path_buf.parent().unwrap();
        parent_path.to_str().unwrap()
    } else {
        path
    };
    opener::open(path).unwrap();
}

#[derive(Clone, Serialize)]
struct EmitWatched {
    watched: bool,
}

#[derive(Clone, Serialize)]
struct EmitPathDeleted {
    path: String,
}

#[derive(Clone, Serialize)]
pub struct EmitProgress {
    total: Option<usize>,
    name: Option<String>,
    folder: bool,
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
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(util::get_log_targets())
                .level(util::get_log_level())
                .build(),
        )
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
            get_metadata,
            set_video_notes,
            delete_path,
            open_path
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
