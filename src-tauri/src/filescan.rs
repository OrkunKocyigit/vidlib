use serde::{Deserialize, Serialize};
use std::fs::{read_dir, ReadDir};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct VideoFile {
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct FolderInfo {
    path: PathBuf,
    folders: Vec<FolderInfo>,
    videos: Vec<VideoFile>,
}

impl FolderInfo {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            folders: Vec::new(),
            videos: Vec::new(),
        }
    }

    pub fn push_folder(&mut self, folder: FolderInfo) {
        self.folders.push(folder);
    }

    pub fn push_video(&mut self, video: VideoFile) {
        self.videos.push(video);
    }

    pub fn read_folder(&mut self) {
        // Read all files
        self.read_files(read_dir(&self.path).unwrap());
        // Read directories
        self.read_directories(read_dir(&self.path).unwrap());
    }

    fn read_files(&mut self, files: ReadDir) {
        for entry in files {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(_) => continue,
            };
            if path.is_file() {
                self.push_video(VideoFile { path })
            }
        }
    }

    fn read_directories(&mut self, files: ReadDir) {
        for entry in files {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(_) => continue,
            };
            if path.is_dir() {
                let mut folder = FolderInfo::new(path);
                folder.read_folder();
                self.push_folder(folder);
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

        let mut root = FolderInfo::new(&self.path);
        root.read_folder();

        Ok(root)
    }
}
