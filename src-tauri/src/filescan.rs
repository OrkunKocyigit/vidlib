use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::{WalkDir};

#[derive(Deserialize, Serialize)]
pub struct VideoFile {
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct FolderInfo {
    pub path: PathBuf,
    pub folders: Vec<FolderInfo>,
    pub videos: Vec<VideoFile>,
}

pub struct FileScan {
    pub path: PathBuf,
}

impl FileScan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf()
        }
    }

    pub async fn run(&self) -> Result<FolderInfo, &str> {
        let is_dir = &self.path.is_dir();
        if !is_dir {
            return Err("Invalid path")
        }

        let mut root = FolderInfo {
            path: PathBuf::from(&*self.path.to_path_buf()),
            folders: Vec::new(),
            videos: Vec::new(),
        };

        let walker = WalkDir::new(&self.path).follow_links(true);
        for entry in walker {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                root.videos.push(VideoFile {
                    path: path.to_path_buf()
                })
            }
            if path.is_dir() {
                root.folders.push(FolderInfo {
                    path: path.to_path_buf(),
                    folders: Vec::new(),
                    videos: Vec::new(),
                })
            }
        }

        Ok(root)
    }
}