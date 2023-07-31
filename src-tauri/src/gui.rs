use std::path::{Path, PathBuf};

use native_dialog::FileDialog;
use rusqlite::Connection;

use crate::database;
use crate::filescan::{FileScan, FolderInfo, VideoFile};
use crate::service::{Response, ResponseType};
use crate::state::{ThumbnailCache, ThumbnailEntry, VideoCache};
use crate::video::VideoEntry;

pub fn file_scan(path: String, cache: &mut VideoCache) -> Result<Response<FolderInfo>, ()> {
    let scan = FileScan::new(Path::new(path.as_str()), Some(cache));
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

pub fn get_folders(folders: &Vec<String>) -> Result<Response<Vec<FolderInfo>>, ()> {
    let mut folder_infos = Vec::new();
    for folder in folders.into_iter() {
        let path = Path::new(&folder);
        let scan = FileScan::new(path, None);
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
    videos: &mut Vec<VideoEntry>,
    connection: &Connection,
) -> Result<Response<VideoFile>, ()> {
    let mut iter = videos.iter();
    let video_entry = match iter.find(|&item| item.id == video.id) {
        Some(video) => video.clone(),
        _ => {
            let new_video = VideoEntry::new(
                video.id.clone(),
                video.name().clone().to_string(),
                0,
                "".to_string(),
                false,
            );
            database::add_video(connection, &new_video).expect("Add video failed");
            videos.push(new_video);
            videos.last().unwrap().clone()
        }
    };
    video.set_video(Some(video_entry));
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
    videos: &mut Vec<VideoEntry>,
    video: VideoEntry,
    new_rating: usize,
) -> Result<Response<usize>, ()> {
    videos
        .iter_mut()
        .find(|item| item.id == video.id)
        .and_then(|item| database::update_rating(connection, item, new_rating))
        .and_then(|item| {
            item.set_rating(new_rating);
            Some(())
        });
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(new_rating),
        error: None,
    })
}

pub(crate) fn update_watched(
    connection: &Connection,
    videos: &mut Vec<VideoEntry>,
    video: VideoEntry,
    watched: bool,
) -> Result<Response<bool>, ()> {
    videos
        .iter_mut()
        .find(|item| item.id == video.id)
        .and_then(|item| database::update_watched(connection, item, watched))
        .and_then(|item| {
            item.set_watched(watched);
            Some(())
        });
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(watched),
        error: None,
    })
}
