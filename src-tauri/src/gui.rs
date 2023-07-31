use std::collections::HashMap;
use std::path::{Path, PathBuf};

use native_dialog::FileDialog;
use rusqlite::Connection;

use crate::database;
use crate::filescan::{FileScan, FolderInfo, VideoFile};
use crate::service::{Response, ResponseType};
use crate::state::{ThumbnailCache, ThumbnailEntry, VideoCache};
use crate::video::VideoEntry;

pub fn file_scan(path: String, cache: &mut VideoCache) -> Result<Response<FolderInfo>, ()> {
    let mut scan = FileScan::new(Path::new(path.as_str()), Some(cache));
    let result = scan.run();
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

pub fn get_folders(
    folders: &Vec<String>,
    cache: &mut VideoCache,
) -> Result<Response<Vec<FolderInfo>>, ()> {
    let mut folder_infos = Vec::new();
    for folder in folders.into_iter() {
        let path = Path::new(&folder);
        let mut scan = FileScan::new(path, Some(cache));
        let folder_scan = scan.run();
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

pub fn get_video(
    video: &mut VideoFile,
    videos: &mut HashMap<String, VideoEntry>,
    connection: &Connection,
) -> Result<Response<VideoFile>, ()> {
    let e = videos.entry(video.id.clone()).or_insert_with(|| {
        let new_video = VideoEntry::new(video.name().clone().to_string(), 0, "".to_string(), false);
        database::add_video(connection, &video.id, &new_video).expect("Add video failed");
        new_video
    });
    video.set_video(Some(e.clone()));
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(video.clone()),
        error: None,
    })
}

pub fn get_thumbnail(
    video: VideoFile,
    thumbnail_cache: &mut ThumbnailCache,
) -> Result<Response<Vec<PathBuf>>, ()> {
    let thumbnails = get_thumbnails(video, thumbnail_cache);
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(thumbnails),
        error: None,
    })
}

fn get_thumbnails(video: VideoFile, thumbnail_cache: &mut ThumbnailCache) -> Vec<PathBuf> {
    let id = video.id.clone();
    let thumbnails = thumbnail_cache.thumbnails();
    match thumbnails.iter().position(|thumbnail| thumbnail.id == id) {
        Some(index) => thumbnails[index].paths().to_owned(),
        _ => {
            let mut entry = ThumbnailEntry::new(id.as_str());
            let thumbnail_paths: Vec<PathBuf> = video.create_thumbnails(thumbnail_cache.path());
            thumbnail_paths
                .iter()
                .for_each(|path| entry.add_video(path));
            thumbnail_cache.add_video(entry);
            thumbnail_paths
        }
    }
}

pub fn update_rating(
    connection: &Connection,
    videos: &mut HashMap<String, VideoEntry>,
    video: VideoFile,
    new_rating: usize,
) -> Result<Response<usize>, ()> {
    videos.get_mut(&video.id).and_then(|v| {
        v.set_rating(new_rating);
        database::update_rating(connection, &video.id, new_rating)
    });
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(new_rating),
        error: None,
    })
}

pub(crate) fn update_watched(
    connection: &Connection,
    videos: &mut HashMap<String, VideoEntry>,
    video: VideoFile,
    watched: bool,
) -> Result<Response<bool>, ()> {
    videos.get_mut(&video.id).and_then(|v| {
        v.set_watched(watched);
        database::update_watched(connection, &video.id, watched)
    });
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(watched),
        error: None,
    })
}
