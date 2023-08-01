use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::hash::Hasher;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

use crate::state::{VideoCache, VideoCacheItem};
use crate::video;
use crate::video::{is_video, VideoEntry};

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
        // Check we can use cached data first
        if let Some(v) = c.as_ref().and_then(|c| c.get_video(path_ref)) {
            if file_size == v.filesize() {
                return v.id().into();
            } else {
                c.as_mut().and_then(|c| Some(c.delete_video(path_ref)));
            }
        }

        let mut reader = BufReader::new(file);
        let mut hasher = Xxh3::new();
        let mut buffer = [0; 1024];

        if file_size > CHUNK_SIZE {
            // Hash first CHUNK_SIZE bytes
            let mut bytes_read: u64 = 0;
            loop {
                if bytes_read >= CHUNK_SIZE {
                    break;
                }
                let count = reader.read(&mut buffer).expect("Failed to read data");
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
                bytes_read += count as u64;
            }

            // Seek to CHUNK_SIZE bytes from end of file
            reader
                .seek(SeekFrom::End(-(CHUNK_SIZE as i64)))
                .expect("Failed to seek");

            // Hash last CHUNK_SIZE bytes
            loop {
                let count = reader.read(&mut buffer).expect("Failed to read data");
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
        } else {
            // Hash entire file
            loop {
                let count = reader.read(&mut buffer).expect("Failed to read data");
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
        }

        let id = format!("{:x}", hasher.finish());
        c.as_mut()
            .and_then(|c| Some(c.add_video(path_ref, VideoCacheItem::new(file_size, id.clone()))));
        id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_video(&mut self, video: Option<VideoEntry>) {
        self.video = video;
    }

    pub fn create_thumbnails<P: AsRef<Path>>(
        &self,
        path: &P,
    ) -> Result<Vec<PathBuf>, anyhow::Error> {
        let p = path.as_ref().to_path_buf();
        let image = p.join(format!("{}_01.png", &self.id));
        let video_url = url::Url::from_file_path(&self.path).expect("Video can't be opened");
        let pipeline = video::create_thumbnail_video_pipeline(video_url, &image)?;
        video::create_thumbnail(pipeline)?;
        Ok(vec![image])
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
}

#[derive(Deserialize, Serialize)]
pub struct FolderInfo {
    path: PathBuf,
    folders: Vec<FolderInfo>,
    videos: Vec<VideoFile>,
    name: String,
    empty: bool,
    depth: usize,
}

impl FolderInfo {
    pub fn new<P: AsRef<Path>>(path: P, depth: usize) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref.file_name().unwrap().to_str().unwrap().to_string();
        Self {
            path: path_ref,
            folders: Vec::new(),
            videos: Vec::new(),
            name,
            empty: true,
            depth,
        }
    }

    pub fn push_folder(&mut self, folder: FolderInfo) {
        self.folders.push(folder);
    }

    pub fn push_video(&mut self, video: VideoFile) {
        self.videos.push(video);
    }

    pub fn read_folder(&mut self, c: &mut Option<&mut VideoCache>) {
        let dir = read_dir(&self.path).unwrap();
        for entry in dir {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(_) => continue,
            };
            if path.is_file() {
                if is_video(&path) {
                    self.push_video(VideoFile::new(path, self.depth, c));
                    self.empty = false;
                }
            } else if path.is_dir() {
                let mut folder = FolderInfo::new(path, self.depth + 1);
                folder.read_folder(c);
                self.push_folder(folder);
                self.empty = false;
            }
        }
    }

    pub(crate) fn add_meta(&mut self, p0: &HashMap<String, VideoEntry>) {
        let _ = &self
            .videos
            .iter_mut()
            .for_each(|v| v.update_meta(p0.get(&v.id)));
        let _ = &self.folders.iter_mut().for_each(|f| f.add_meta(p0));
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

    pub fn run(&mut self) -> Result<FolderInfo, &str> {
        let is_dir = &self.path.is_dir();
        if !is_dir {
            return Err("Invalid path");
        }

        let mut root = FolderInfo::new(&self.path, 0);
        root.read_folder(&mut self.cache);

        Ok(root)
    }
}
