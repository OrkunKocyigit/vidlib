use serde::{Deserialize, Serialize};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct VideoFile {
    path: PathBuf,
    name: String,
}

impl VideoFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref.file_name().unwrap().to_str().unwrap().to_string();
        Self {
            path: path_ref,
            name,
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
}

impl FolderInfo {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref().to_owned();
        let name = path_ref.file_name().unwrap().to_str().unwrap().to_string();
        Self {
            path: path_ref,
            folders: Vec::new(),
            videos: Vec::new(),
            name,
            empty: true,
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
                self.push_video(VideoFile::new(path));
                self.empty = false;
            } else if path.is_dir() {
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
