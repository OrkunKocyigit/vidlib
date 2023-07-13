use crate::filescan::{FileScan, FolderInfo};
use crate::service::{Response, ResponseType};
use native_dialog::FileDialog;
use std::path::{Path, PathBuf};

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
    let path = FileDialog::new().show_open_single_dir();
    let response = match path {
        Ok(path) => Response {
            result: ResponseType::SUCCESS,
            response: Some(path.unwrap()),
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
