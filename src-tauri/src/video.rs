extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Error;
use derive_more::{Display, Error};
use gstreamer::ClockTime;
use serde::{Deserialize, Serialize};
use url::Url;

macro_rules! get_tag {
    ($tag:expr, $ty:ty) => {
        $tag.get::<$ty>().map(|k| k.get().to_owned())
    };
}

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

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: gstreamer::glib::GString,
    error: gstreamer::glib::Error,
    debug: Option<gstreamer::glib::GString>,
}

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        _ => return false,
    };
    let mut buf = vec![0; 1024];
    let result = file.read_exact(&mut buf);
    if result.is_err() {
        false
    } else {
        infer::is_video(&*buf)
    }
}

#[derive(Builder, Serialize, Deserialize)]
pub struct VideoMetadata {
    #[builder(default = "None")]
    width: Option<u32>,
    #[builder(default = "None")]
    height: Option<u32>,
    #[builder(default = "None")]
    framerate: Option<String>,
    #[builder(default = "None")]
    filesize: Option<String>,
    #[builder(default = "None")]
    bitrate: Option<String>,
    #[builder(default = "None")]
    length: Option<String>,
    #[builder(default = "None")]
    codec: Option<String>,
    #[builder(default = "None")]
    abitrate: Option<String>,
    #[builder(default = "None")]
    acodec: Option<String>,
    #[builder(default = "None")]
    asample: Option<u32>,
}

pub(crate) async fn create_metadata(path: &PathBuf) -> Result<VideoMetadata, Error> {
    let discoverer = gstreamer_pbutils::Discoverer::new(ClockTime::from_seconds(5))?;
    let discoverer = discoverer.discover_uri(&format!(
        "{}",
        Url::from_file_path(&path).expect("Video can't be opened")
    ))?;
    let mut builder = VideoMetadataBuilder::default();
    builder.filesize(Some(path.metadata()?.len()).map(format_file_size));
    if let Some(duration) = discoverer.duration() {
        let seconds = duration.seconds();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;
        let milliseconds = duration.mseconds() % 1000;
        builder.length(Some(format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )));
    }
    if let Some(video) = discoverer.video_streams().first() {
        builder.bitrate(Some(format!("{} kbps", video.bitrate() / 1024)));
        builder.width(Some(video.width()));
        builder.height(Some(video.height()));
        builder.framerate(Some(format!(
            "{}",
            video.framerate().numer() as f32 / video.framerate().denom() as f32
        )));
    }
    if let Some(audio) = discoverer.audio_streams().first() {
        builder.abitrate(Some(format!("{} kbps", audio.bitrate() / 1024)));
        builder.asample(Some(audio.sample_rate()));
    }
    if let Some(tags) = discoverer.tags() {
        builder.codec(get_tag!(tags, gst::tags::VideoCodec));
        builder.acodec(get_tag!(tags, gst::tags::AudioCodec));
    }
    builder.build().map_err(|e| Error::msg(e.to_string()))
}

fn format_file_size(size: u64) -> String {
    let units = ["KB", "MB", "GB", "TB", "PB", "EB"];
    if size < 1024 {
        return format!("{} B", size);
    }
    let mut size = size as f64 / 1024.0;
    for unit in units.iter() {
        if size < 1024.0 {
            return format!("{:.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.1} {}", size, units.last().unwrap())
}
