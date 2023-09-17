use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;

use crate::mediainfo::VideoMediaInfoChannelMessage;
use crate::thumbnail::ThumbnailChannelMessage;
use crate::video::VideoEntry;
use crate::{database, thumbnail, EmitProgress};

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
    pub thumbnail_cache: tokio::sync::Mutex<Option<thumbnail::ThumbnailCache>>,
    pub video_cache: Mutex<Option<VideoCache>>,
    pub thumbnail_channel: tokio::sync::Mutex<tokio::sync::mpsc::Sender<ThumbnailChannelMessage>>,
    pub mediainfo_channel:
        tokio::sync::Mutex<tokio::sync::mpsc::Sender<VideoMediaInfoChannelMessage>>,
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
