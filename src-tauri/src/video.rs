use std::fs::File;
use std::path::Path;

use rsmpeg::ffi::AVMediaType_AVMEDIA_TYPE_VIDEO;
use serde::{Deserialize, Serialize};

use crate::thumbnail;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoCategory {
    id: usize,
    name: String,
    icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoEntry {
    name: String,
    rating: usize,
    notes: String,
    watched: bool,
    category: Option<VideoCategory>,
}

impl VideoEntry {
    pub fn new(name: String, rating: usize, notes: String, watched: bool) -> Self {
        Self {
            name,
            rating,
            notes,
            watched,
            category: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rating(&self) -> usize {
        self.rating
    }
    pub fn notes(&self) -> &str {
        &self.notes
    }
    pub fn watched(&self) -> bool {
        self.watched
    }
    pub fn set_rating(&mut self, rating: usize) {
        self.rating = rating;
    }
    pub fn set_watched(&mut self, watched: bool) {
        self.watched = watched;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }
}

pub const VIDEO_FILE_EXTENSIONS: &[&str] = &[
    "3g2", "3gp", "3gp2", "3gpp", "amv", "asf", "avi", "bik", "dds", "divx", "dpg", "dv", "dvr-ms",
    "evo", "f4v", "flv", "hdmov", "k3g", "m1v", "m2t", "m2ts", "m2v", "m4b", "m4p", "m4v", "mk3d",
    "mkv", "mov", "mp2v", "mp4", "mp4v", "mpe", "mpeg", "mpg", "mpv2", "mpv4", "mqv", "mts", "mxf",
    "nsv", "ogm", "ogv", "qt", "ram", "rm", "rmvb", "skm", "swf", "tp", "tpr", "trp", "ts", "vob",
    "webm", "wm", "wmv", "xvid",
];

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if is_file_openable(path) && has_video_extension(path) {
        has_video_stream(path)
    } else {
        false
    }
}

fn is_file_openable<P: AsRef<Path>>(path: P) -> bool {
    File::open(path).is_ok()
}

fn has_video_extension<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .map(|e| VIDEO_FILE_EXTENSIONS.contains(&e.to_string_lossy().as_ref()))
        .unwrap_or(false)
}

fn has_video_stream<P: AsRef<Path>>(path: P) -> bool {
    match thumbnail::create_input_context(path) {
        Ok(input_context) => input_context
            .streams()
            .into_iter()
            .any(|s| s.codecpar().codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO),
        Err(_) => false,
    }
}
