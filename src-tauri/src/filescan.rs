use std::path::{Path, PathBuf};

pub struct VideoFile {
    pub path: PathBuf,
}

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

    pub async fn run(&self) -> FolderInfo {
        let is_dir = &self.path.is_dir();
        if !is_dir {
            panic!("Invalid path")
        }

        FolderInfo {
            path: PathBuf::from(&*self.path.to_path_buf()),
            folders: Vec::new(),
            videos: Vec::new()
        }
    }
}