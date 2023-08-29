use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::util::get_app_dir;
use crate::video::VideoEntry;
use crate::{database, EmitProgress};

pub struct ThumbnailCache {
    base_dir: PathBuf,
    thumbnails: HashMap<String, ThumbnailEntry>,
}

impl ThumbnailCache {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            thumbnails: HashMap::new(),
        }
    }

    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    pub fn add_thumbnail_entry(&mut self, id: &String, path: &PathBuf) {
        self.thumbnails
            .entry(id.clone())
            .or_insert_with(|| ThumbnailEntry::new())
            .add_video(path)
    }

    pub fn get_paths(&self, id: &String) -> Option<&Vec<PathBuf>> {
        self.thumbnails.get(id).map(|t| t.paths())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThumbnailEntry {
    paths: Vec<PathBuf>,
}

impl ThumbnailEntry {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
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
    let path = get_app_dir(app_handle);
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
                thumbnail_cache.add_thumbnail_entry(&id.to_string(), &path);
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

#[derive(Clone, Serialize)]
pub struct EmitTotalProgress {
    current: usize,
    total: usize,
    progress: f64,
    name: Option<String>,
}

impl EmitTotalProgress {
    pub fn new() -> Self {
        Self {
            current: 0,
            total: 0,
            progress: 0.0f64,
            name: None,
        }
    }

    pub fn process(&mut self, emit: EmitProgress) {
        if let Some(t) = emit.total {
            self.total += t;
        }
        if let Some(n) = emit.name {
            self.name = Some(n);
        }
        if !emit.folder {
            self.current += 1;
        }
        self.update_progress();
    }

    fn update_progress(&mut self) {
        self.progress = (self.current as f64 / self.total.max(1) as f64) * 100.0;
    }
}
