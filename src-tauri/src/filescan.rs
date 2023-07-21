use std::fs::{read_dir, File};
use std::hash::Hasher;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

use crate::video::{is_video, VideoEntry};

const CHUNK_SIZE: u64 = 1 * 1024 * 1024;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoFile {
    pub id: String,
    path: PathBuf,
    name: String,
    depth: usize,
    video: Option<VideoEntry>,
    thumbnails: Option<Vec<PathBuf>>,
}

impl VideoFile {
    pub fn new<P: AsRef<Path>>(path: P, depth: usize) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref.file_name().unwrap().to_str().unwrap().to_string();
        let hash = VideoFile::hash_file(&path_ref);
        Self {
            id: hash,
            path: path_ref,
            name,
            depth,
            video: None,
            thumbnails: None,
        }
    }

    fn hash_file(path_ref: &PathBuf) -> String {
        let file = File::open(path_ref).expect("Failed to open file");
        let file_size = file.metadata().expect("Failed to get file metadata").len();
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

        format!("{:x}", hasher.finish())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_video(&mut self, video: Option<VideoEntry>) {
        self.video = video;
    }

    pub fn set_thumbnails(&mut self, thumbnails: Option<Vec<PathBuf>>) {
        self.thumbnails = thumbnails;
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

    pub fn read_folder(&mut self) {
        let dir = read_dir(&self.path).unwrap();
        for entry in dir {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(_) => continue,
            };
            if path.is_file() {
                if is_video(&path) {
                    self.push_video(VideoFile::new(path, self.depth));
                    self.empty = false;
                }
            } else if path.is_dir() {
                let mut folder = FolderInfo::new(path, self.depth + 1);
                folder.read_folder();
                self.push_folder(folder);
                self.empty = false;
            }
        }
    }
}

pub struct FileScan {
    pub path: PathBuf,
}

impl FileScan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub async fn run(&self) -> Result<FolderInfo, &str> {
        let is_dir = &self.path.is_dir();
        if !is_dir {
            return Err("Invalid path");
        }

        let mut root = FolderInfo::new(&self.path, 0);
        root.read_folder();

        Ok(root)
    }
}
