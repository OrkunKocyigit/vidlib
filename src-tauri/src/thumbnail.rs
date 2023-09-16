use std::fmt::Formatter;
use std::{collections, fmt, fs, path};

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tokio::sync;

use crate::{state, util};

// Thumbnail Cache
pub struct ThumbnailCache {
    thumbnails: collections::HashMap<String, ThumbnailEntry>,
}
impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            thumbnails: collections::HashMap::new(),
        }
    }

    pub fn add_thumbnail_entry(&mut self, id: &String, path: &path::PathBuf) {
        self.thumbnails
            .entry(id.clone())
            .or_insert_with(|| ThumbnailEntry::new())
            .add_thumbnail(path)
    }

    pub fn get_paths(&self, id: &String) -> Option<&Vec<path::PathBuf>> {
        self.thumbnails.get(id).map(|t| t.paths())
    }

    pub fn remove_path(&mut self, id: &String) -> Option<ThumbnailEntry> {
        debug!("Remove path {}", id);
        self.thumbnails.remove(id)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ThumbnailEntry {
    paths: Vec<path::PathBuf>,
}

impl ThumbnailEntry {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn add_thumbnail(&mut self, path: &path::PathBuf) {
        let _ = &self.paths.push(path.clone());
    }

    pub fn paths(&self) -> &Vec<path::PathBuf> {
        &self.paths
    }
}

pub fn create_thumbnail_cache(thumbnail_path: &path::PathBuf) -> ThumbnailCache {
    let mut thumbnail_cache = ThumbnailCache::new();
    for entry in fs::read_dir(&thumbnail_path).unwrap() {
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

pub fn get_thumbnail_save_location(app: &AppHandle) -> path::PathBuf {
    let path = util::get_app_dir(app);
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let thumbnail_path = path.join("thumbnail");
    fs::create_dir_all(&thumbnail_path).expect("Thumbnail folder creation failed");
    thumbnail_path
}

pub async fn find_thumbnail_path_in_cache(
    state: &tauri::State<'_, state::AppState>,
    id: &String,
) -> Option<Vec<path::PathBuf>> {
    debug!("Check if we have it in cache");
    if let Some(cache) = state.thumbnail_cache.lock().await.as_mut() {
        if let Some(path) = cache.get_paths(id) {
            debug!("Found thumbnail path in cache {}", id);
            if validate_thumbnail(path) {
                return Some(path.clone());
            } else {
                debug!("Thumbnail path is no longer valid");
                cache.remove_path(id);
            }
        } else {
            debug!("Thumbnail is not in the cache {}", id)
        }
    } else {
        debug!("Cache doesn't exist");
    }
    None
}

fn validate_thumbnail(v: &Vec<path::PathBuf>) -> bool {
    v.iter()
        .map(|v| v.exists() && v.is_file())
        .fold(true, |acc, v| acc && v)
}
// Channel
#[derive(Clone)]
pub struct ThumbnailChannelMessage {
    path: path::PathBuf,
    id: String,
}

impl ThumbnailChannelMessage {
    pub fn new(path: path::PathBuf, id: String) -> Self {
        Self { path, id }
    }
}

impl fmt::Display for ThumbnailChannelMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}, path: {}", self.id, self.path.display())
    }
}

async fn create_and_send_thumbnail(
    save_location: &path::PathBuf,
    input: ThumbnailChannelMessage,
    thumbnail_output_tx: &sync::mpsc::Sender<ThumbnailChannelMessage>,
) {
    let thumbnail = create_thumbnail(save_location, &input.id, &input.path);
    if let Err(e) = thumbnail_output_tx
        .send(ThumbnailChannelMessage::new(thumbnail, input.id))
        .await
    {
        error!("Failed to send thumbnail output: {}", e);
    }
}

pub async fn process_thumbnail_input_channels(
    thumbnail_cache: &sync::Mutex<Option<ThumbnailCache>>,
    save_location: &path::PathBuf,
    mut thumbnail_input_rx: sync::mpsc::Receiver<ThumbnailChannelMessage>,
    thumbnail_output_tx: sync::mpsc::Sender<ThumbnailChannelMessage>,
) -> Result<(), anyhow::Error> {
    while let Some(input) = thumbnail_input_rx.recv().await {
        debug!("Message received in thumbnail input {}", input);
        create_and_send_thumbnail(save_location, input, &thumbnail_output_tx).await;
    }

    Ok(())
}

pub async fn process_thumbnail_output_channels(
    app: &AppHandle,
    mut thumbnail_output_rx: sync::mpsc::Receiver<ThumbnailChannelMessage>,
) -> Result<(), anyhow::Error> {
    while let Some(output) = thumbnail_output_rx.recv().await {
        debug!("Message received in thumbnail output {}", output);
        let _ = app.emit_all(
            &*format!("update_thumbnail_{}", output.id),
            ThumbnailEmitEvent::new(output.path),
        );
    }

    Ok(())
}

// Events
#[derive(Clone, Serialize)]
pub struct ThumbnailEmitEvent {
    path: path::PathBuf,
}

impl ThumbnailEmitEvent {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }
}

// Creator
fn create_thumbnail(
    save_location: &path::PathBuf,
    id: &String,
    video_location: &path::PathBuf,
) -> path::PathBuf {
    let file_name = &format!("{}_01.png", id);
    let full_location = &save_location.join(file_name);
    let path = generate_thumbnail(file_name, full_location, video_location);
    match path {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to generate thumbnail: {}", e);
            path::PathBuf::from("./images/image_not_found.webp")
        }
    }
}

fn generate_thumbnail(
    save_location: &String,
    file_name: &path::PathBuf,
    video_location: &path::PathBuf,
) -> Result<path::PathBuf, anyhow::Error> {
    Err(anyhow::Error::msg("Test"))
}
