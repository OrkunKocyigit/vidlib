use std::collections::HashMap;
use std::path::{Path, PathBuf};

use native_dialog::FileDialog;
use rusqlite::Connection;

use crate::filescan::{FileScan, FolderInfo, VideoFile};
use crate::service::{Response, ResponseType};
use crate::state::VideoCache;
use crate::video::VideoEntry;
use crate::{database, EmitProgress};

pub fn file_scan(
    path: String,
    cache: &mut VideoCache,
    x: &HashMap<String, VideoEntry>,
    emitter: impl Fn(EmitProgress),
) -> Result<Response<FolderInfo>, ()> {
    let mut scan = FileScan::new(Path::new(path.as_str()), Some(cache));
    let result = scan.run(&emitter);
    let response = match result {
        Ok(mut folder_info) => {
            folder_info.add_meta(x);
            Response {
                result: ResponseType::SUCCESS,
                response: Some(folder_info),
                error: None,
            }
        }
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
    entries: &HashMap<String, VideoEntry>,
    emitter: impl Fn(EmitProgress),
) -> Result<Response<Vec<FolderInfo>>, ()> {
    let mut folder_infos = Vec::new();
    for folder in folders.into_iter() {
        let path = Path::new(&folder);
        let mut scan = FileScan::new(path, Some(cache));
        let folder_scan = scan.run(&emitter);
        if let Ok(mut folder_info) = folder_scan {
            folder_info.add_meta(entries);
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

pub(crate) fn update_name(
    c: &Connection,
    v: &mut HashMap<String, VideoEntry>,
    f: &VideoFile,
    n: &String,
) -> Result<Response<String>, ()> {
    v.get_mut(&f.id).and_then(|e| {
        e.set_name(n.to_owned());
        database::update_name(c, &f.id, n)
    });
    Ok(Response {
        result: ResponseType::SUCCESS,
        response: Some(n.to_owned()),
        error: None,
    })
}

pub(crate) fn update_notes(
    c: &Connection,
    v: &mut HashMap<String, VideoEntry>,
    f: &VideoFile,
    n: &String,
) -> Result<Response<String>, ()> {
    v.get_mut(&f.id).and_then(|e| {
        e.set_notes(n.to_owned());
        database::update_notes(c, &f.id, n)
    });
    Ok(wrap_success(n.to_owned()))
}

pub(crate) fn wrap_success<T>(response: T) -> Response<T> {
    Response {
        result: ResponseType::SUCCESS,
        response: Some(response),
        error: None,
    }
}
pub(crate) fn wrap_failure<T>(error: String) -> Response<T> {
    Response {
        result: ResponseType::FAILURE,
        response: None,
        error: Some(error),
    }
}

pub(crate) fn validate_path(db: &Connection, path: &str) -> Result<bool, Response<bool>> {
    database::get_paths(db)
        .map(|paths| paths.contains(&path.to_string()))
        .map_err(|e| wrap_failure(e.to_string()))
}

pub(crate) fn delete_path(
    db: &mut Connection,
    cache: &mut VideoCache,
    path: &str,
) -> Response<bool> {
    if let Err(r) = database::delete_path(db, &path) {
        wrap_failure(r.to_string())
    } else {
        match database::get_cache_items_with_path(db, path) {
            Ok(paths) => {
                paths.iter().for_each(|p| cache.delete_video(p));
                cache.commit(db);
                wrap_success(true)
            }
            Err(r) => wrap_failure(r.to_string()),
        }
    }
}

pub(crate) fn open_video(v: VideoFile) -> Result<Response<()>, ()> {
    let path = v.path();
    debug!("Checking for video path: {}", path.display());
    if path.exists() && path.is_file() {
        debug!("File exists");
        match opener::open(path) {
            Ok(_) => {
                debug!("File opened");
                Ok(wrap_success(()))
            }
            Err(e) => {
                error!("Opener failed with error {}", e);
                Ok(wrap_failure(e.to_string()))
            }
        }
    } else {
        error!("File not found");
        Ok(wrap_failure("File not found".into()))
    }
}
