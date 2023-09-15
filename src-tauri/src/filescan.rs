use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{read_dir, read_link, symlink_metadata, DirEntry, File};
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::os::windows::prelude::MetadataExt;
use std::path::{Path, PathBuf};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

use crate::state::{VideoCache, VideoCacheItem};
use crate::video::{is_video, VideoEntry, VideoMetadata};
use crate::{video, EmitProgress};

const CHUNK_SIZE: u64 = 1 * 1024 * 1024;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoFile {
    pub id: String,
    path: PathBuf,
    name: String,
    depth: usize,
    video: Option<VideoEntry>,
    watched: bool,
}

impl VideoFile {
    pub fn new<P: AsRef<Path>>(path: P, depth: usize, c: &mut Option<&mut VideoCache>) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref.file_name().unwrap().to_str().unwrap().to_string();
        let hash = VideoFile::hash_file(&path_ref, c);
        Self {
            id: hash,
            path: path_ref,
            name,
            depth,
            video: None,
            watched: false,
        }
    }

    fn hash_file(path_ref: &PathBuf, c: &mut Option<&mut VideoCache>) -> String {
        let file = File::open(path_ref).expect("Failed to open file");
        let file_size = file.metadata().expect("Failed to get file metadata").len();
        if let Some(v) = c.as_ref().and_then(|c| c.get_video(path_ref)) {
            if file_size == v.filesize() {
                return v.id().into();
            } else {
                c.as_mut().map(|c| c.delete_video(path_ref));
            }
        }

        let mut reader = BufReader::new(file);
        let mut hasher = Xxh3::new();
        let mut buffer = Vec::new();

        if file_size > CHUNK_SIZE {
            reader
                .by_ref()
                .take(CHUNK_SIZE)
                .read_to_end(&mut buffer)
                .unwrap();
            hasher.update(&buffer);
            reader.seek(SeekFrom::End(-(CHUNK_SIZE as i64))).unwrap();
            reader.read_to_end(&mut buffer).unwrap();
            hasher.update(&buffer);
        } else {
            reader.read_to_end(&mut buffer).unwrap();
            hasher.update(&buffer);
        }

        let id = format!("{:x}", hasher.finish());
        c.as_mut()
            .map(|c| c.add_video(path_ref, VideoCacheItem::new(file_size, id.clone())));
        id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_video(&mut self, video: Option<VideoEntry>) {
        self.video = video;
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_watched(&mut self, watched: bool) {
        self.watched = watched;
    }

    pub(crate) fn update_meta(&mut self, p0: Option<&VideoEntry>) {
        if let Some(e) = p0 {
            let _ = &self.set_watched(e.watched());
        }
    }

    pub(crate) async fn get_metadata(&self) -> Result<VideoMetadata, Error> {
        video::create_metadata(&self.path).await
    }
}

#[derive(Deserialize, Serialize)]
pub struct FolderInfo {
    id: String,
    path: PathBuf,
    folders: Vec<FolderInfo>,
    videos: Vec<VideoFile>,
    name: String,
    empty: bool,
    depth: usize,
    watched: bool,
}

impl FolderInfo {
    pub fn new<P: AsRef<Path>>(path: P, depth: usize) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref
            .file_name()
            .unwrap_or_else(|| path_ref.as_os_str())
            .to_str()
            .unwrap()
            .to_string();
        let mut hasher = DefaultHasher::new();
        path_ref.hash(&mut hasher);
        let id = format!("{:x}", hasher.finish());
        Self {
            id,
            path: path_ref,
            folders: Vec::new(),
            videos: Vec::new(),
            name,
            empty: true,
            depth,
            watched: false,
        }
    }

    pub fn push_folder(&mut self, folder: FolderInfo) {
        self.folders.push(folder);
    }

    pub fn push_video(&mut self, video: VideoFile) {
        self.videos.push(video);
    }

    fn count_files_in_folder(&self, emitter: &impl Fn(EmitProgress)) -> usize {
        let count = read_dir(&self.path).map_or(0, |entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    let path = if symlink_metadata(entry.path())
                        .unwrap()
                        .file_type()
                        .is_symlink()
                    {
                        read_link(entry.path()).unwrap()
                    } else {
                        entry.path()
                    };
                    path.metadata().unwrap().is_file() && is_video(path)
                })
                .count()
        });
        emitter(EmitProgress {
            total: Some(count),
            name: Some(self.path.display().to_string()),
            folder: true,
        });
        count
    }

    pub fn read_folder(
        &mut self,
        c: &mut Option<&mut VideoCache>,
        emitter: &impl Fn(EmitProgress),
    ) {
        let _ = &self.count_files_in_folder(&emitter);
        if let Ok(entries) = read_dir(&self.path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && is_video(&path) {
                    let video_file = VideoFile::new(path, self.depth, c);
                    emitter(EmitProgress {
                        name: Some(video_file.name().to_string()),
                        total: None,
                        folder: false,
                    });
                    self.push_video(video_file);
                    self.empty = false;
                } else if path.is_dir() && !is_hidden(&entry) {
                    let mut folder = FolderInfo::new(path, self.depth + 1);
                    folder.read_folder(c, emitter);
                    self.push_folder(folder);
                    self.empty = false;
                }
            }
        }
    }

    pub(crate) fn add_meta(&mut self, p0: &HashMap<String, VideoEntry>) {
        let _ = &self
            .videos
            .iter_mut()
            .for_each(|v| v.update_meta(p0.get(&v.id)));
        let _ = &self.folders.iter_mut().for_each(|f| f.add_meta(p0));
        self.watched = self
            .videos
            .iter()
            .map(|e| e.watched)
            .reduce(|acc, b| acc && b)
            .unwrap_or(false);
    }
}

pub struct FileScan<'a> {
    pub path: PathBuf,
    cache: Option<&'a mut VideoCache>,
}

impl<'a> FileScan<'a> {
    pub fn new<P: AsRef<Path>>(path: P, cache: Option<&'a mut VideoCache>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            cache,
        }
    }

    pub fn run(&mut self, emitter: &impl Fn(EmitProgress)) -> Result<FolderInfo, &str> {
        let is_dir = &self.path.is_dir();
        if !is_dir {
            return Err("Invalid path");
        }

        let mut root = FolderInfo::new(&self.path, 0);
        root.read_folder(&mut self.cache, emitter);

        Ok(root)
    }
}

#[cfg(target_os = "windows")]
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .metadata()
        .map(|metadata| metadata.file_attributes() & 0x2 != 0)
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
