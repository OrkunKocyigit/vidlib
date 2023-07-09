// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use crate::filescan::{FileScan, FolderInfo};
use crate::service::{Response, ResponseType};

mod filescan;
mod service;

#[tauri::command]
async fn file_scan(path: String) -> Result<Response<FolderInfo>, ()> {
    let scan = FileScan::new(Path::new(path.as_str()));
    let result = scan.run().await;
    let response = match result {
        Ok(folder_info) => Response {
            result: ResponseType::SUCCESS,
            response: Some(folder_info),
            error: None
        },
        Err(error) => Response {
            result: ResponseType::FAILURE,
            response: None,
            error: Some(error.to_string()),
        }
    };
    Ok(response)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
