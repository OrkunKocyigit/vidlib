use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::video::VideoEntry;

#[derive(Serialize, Deserialize)]
pub struct ThumbnailEntry {
    pub id: String,
    paths: Vec<PathBuf>,
}

impl ThumbnailEntry {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            paths: Vec::new(),
        }
    }

    pub fn add_video(&mut self, path: &PathBuf) {
        let _ = &self.paths.push(path.clone());
    }
}

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
    pub videos: Mutex<Option<Vec<VideoEntry>>>,
    pub thumbnails: Mutex<Option<Vec<ThumbnailEntry>>>,
}

pub fn get_thumbnails(app_handle: &AppHandle) -> Vec<ThumbnailEntry> {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist");
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let thumbnail_path = path.join("/thumbnail");
    fs::create_dir_all(&thumbnail_path).expect("Thumbnail folder creation failed");
    let mut thumbnails: Vec<ThumbnailEntry> = Vec::new();
    for entry in read_dir(thumbnail_path).unwrap() {
        let dir_entry = entry.unwrap();
        if dir_entry.path().is_file() {
            let name = dir_entry.file_name();
            let file_name = name.to_str().unwrap();
            if file_name.contains("_") {
                let mut split = file_name.split("_");
                let id = split.next().unwrap();
                let path = dir_entry.path();
                match thumbnails.iter().position(|thumb| thumb.id.as_str() == id) {
                    Some(index) => thumbnails[index].add_video(&path),
                    _ => {
                        let mut entry = ThumbnailEntry::new(id);
                        entry.add_video(&path);
                        thumbnails.push(entry);
                    }
                }
            }
        }
    }
    thumbnails
}
