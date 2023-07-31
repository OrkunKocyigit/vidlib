use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::database;
use crate::video::VideoEntry;

pub struct ThumbnailCache {
    path: PathBuf,
    thumbnails: Vec<ThumbnailEntry>,
}

impl ThumbnailCache {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            thumbnails: Vec::new(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn thumbnails(&self) -> &Vec<ThumbnailEntry> {
        &self.thumbnails
    }

    pub fn add_video(&mut self, entry: ThumbnailEntry) {
        let _ = &self.thumbnails.push(entry);
    }
}

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

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }
}

pub struct VideoCache {
    items: HashMap<String, VideoCacheItem>,
    add: HashMap<String, VideoCacheItem>,
    delete: Vec<String>,
}

impl VideoCache {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            add: HashMap::new(),
            delete: Vec::new(),
        }
    }

    pub fn get_video(&self, p: &PathBuf) -> Option<&VideoCacheItem> {
        let p = p.display().to_string();
        self.items.get(&p)
    }

    pub fn add_video<P: AsRef<Path>>(&mut self, p: P, v: VideoCacheItem) {
        let _ = &self.add.insert(p.as_ref().display().to_string(), v);
    }

    pub fn delete_video<P: AsRef<Path>>(&mut self, p: P) {
        let _ = &self.delete.push(p.as_ref().display().to_string());
    }

    pub fn commit(&mut self, connection: &Connection) {
        for p in &self.delete {
            database::delete_video_cache(connection, p);
            let _ = &self.items.remove(p);
        }
        let _ = &self.delete.clear();
        for (p, v) in &self.add {
            database::add_video_cache(connection, p, &v.filesize, &v.id);
            let _ = &self.items.insert(p.clone(), v.clone());
        }
        let _ = &self.add.clear();
    }
}

#[derive(Clone)]
pub struct VideoCacheItem {
    filesize: u64,
    id: String,
}

impl VideoCacheItem {
    pub fn new(filesize: u64, id: String) -> Self {
        Self { filesize, id }
    }

    pub fn filesize(&self) -> u64 {
        self.filesize
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
    pub videos: Mutex<Option<HashMap<String, VideoEntry>>>,
    pub thumbnail_cache: Mutex<Option<ThumbnailCache>>,
    pub video_cache: Mutex<Option<VideoCache>>,
}

pub fn get_thumbnails(app_handle: &AppHandle) -> ThumbnailCache {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist");
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let thumbnail_path = path.join("thumbnail");
    fs::create_dir_all(&thumbnail_path).expect("Thumbnail folder creation failed");
    let mut thumbnail_cache = ThumbnailCache::new(&thumbnail_path);
    for entry in read_dir(&thumbnail_path).unwrap() {
        let dir_entry = entry.unwrap();
        if dir_entry.path().is_file() {
            let name = dir_entry.file_name();
            let file_name = name.to_str().unwrap();
            if file_name.contains("_") {
                let mut split = file_name.split("_");
                let id = split.next().unwrap();
                let path = dir_entry.path();
                match thumbnail_cache
                    .thumbnails
                    .iter()
                    .position(|thumb| thumb.id.as_str() == id)
                {
                    Some(index) => thumbnail_cache.thumbnails[index].add_video(&path),
                    _ => {
                        let mut entry = ThumbnailEntry::new(id);
                        entry.add_video(&path);
                        thumbnail_cache.thumbnails.push(entry);
                    }
                }
            }
        }
    }
    thumbnail_cache
}

pub fn get_video_cache(connection: &Connection) -> VideoCache {
    let mut cache = VideoCache::new();
    let _ = database::get_video_cache_items(connection).and_then(|items| Ok(cache.items = items));
    cache
}
