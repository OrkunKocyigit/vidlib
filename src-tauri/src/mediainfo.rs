use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Builder, Serialize, Deserialize)]
pub struct VideoMediaInfo {
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

pub struct VideoMediaInfoChannelMessage {
    path: PathBuf,
    info: Option<VideoMediaInfo>,
}

impl VideoMediaInfoChannelMessage {
    pub fn new(path: PathBuf, info: Option<VideoMediaInfo>) -> Self {
        Self { path, info }
    }
}

pub async fn process_mediainfo_input_channels(
    mediainfo_input_rx: Receiver<VideoMediaInfoChannelMessage>,
    mediainfo_output_tx: Sender<VideoMediaInfoChannelMessage>,
) -> Result<(), Error> {
    todo!()
}

pub async fn process_mediainfo_output_channels(
    app: &AppHandle,
    mediainfo_output_rx: Receiver<VideoMediaInfoChannelMessage>,
) -> Result<(), Error> {
    todo!()
}
