use std::path::{Path, PathBuf};

use native_dialog::FileDialog;
use tauri::Manager;

use crate::filescan::{FileScan, FolderInfo};
use crate::service::{Response, ResponseType};

pub async fn file_scan(path: String) -> Result<Response<FolderInfo>, ()> {
    let scan = FileScan::new(Path::new(path.as_str()));
    let result = scan.run().await;
    let response = match result {
        Ok(folder_info) => Response {
            result: ResponseType::SUCCESS,
            response: Some(folder_info),
            error: None,
        },
        Err(error) => Response {
            result: ResponseType::FAILURE,
            response: None,
            error: Some(error.to_string()),
        },
    };
    Ok(response)
}

pub fn select_folder() -> Result<Response<PathBuf>, ()> {
    let path = FileDialog::new().show_open_single_dir().unwrap();
    let response = match path {
        Some(path) => Response {
            result: ResponseType::SUCCESS,
            response: Some(path),
            error: None,
        },
        None => Response {
            result: ResponseType::CANCELED,
            response: None,
            error: None,
        },
    };
    Ok(response)
}

pub async fn get_folders(folders: &Vec<String>) -> Result<Response<Vec<FolderInfo>>, ()> {
    let mut folder_infos = Vec::new();
    for folder in folders.into_iter() {
        let path = Path::new(&folder);
        let scan = FileScan::new(path);
        let folder_scan = scan.run().await;
        if let Ok(folder_info) = folder_scan {
            folder_infos.push(folder_info);
        }
    }
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(folder_infos),
        error: None,
    })
}
