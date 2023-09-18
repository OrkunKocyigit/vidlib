use std::path::{Path, PathBuf};

use anyhow::Error;
use rsmpeg::avcodec::AVCodecRef;
use rsmpeg::avutil::{av_q2d, AVRational};
use rsmpeg::ffi::{AVMediaType_AVMEDIA_TYPE_AUDIO, AVMediaType_AVMEDIA_TYPE_VIDEO};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{thumbnail, util};

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct VideoMediaInfo {
    #[builder(default = "None")]
    width: Option<i32>,
    #[builder(default = "None")]
    height: Option<i32>,
    #[builder(default = "None")]
    framerate: Option<f64>,
    #[builder(default = "None")]
    filesize: Option<String>,
    #[builder(default = "None")]
    bitrate: Option<i64>,
    #[builder(default = "None")]
    length: Option<String>,
    #[builder(default = "None")]
    codec: Option<String>,
    #[builder(default = "None")]
    abitrate: Option<i64>,
    #[builder(default = "None")]
    acodec: Option<String>,
    #[builder(default = "None")]
    asample: Option<i32>,
}
// Events
#[derive(Clone, Serialize)]
pub struct VideoMediaInfoEmitEvent {
    media_info: VideoMediaInfo,
}

impl VideoMediaInfoEmitEvent {
    pub fn new(path: VideoMediaInfo) -> Self {
        Self { media_info: path }
    }
}

// Channels
pub struct VideoMediaInfoChannelMessage {
    id: String,
    path: PathBuf,
    info: Option<VideoMediaInfo>,
}

impl VideoMediaInfoChannelMessage {
    pub fn new(id: String, path: PathBuf, info: Option<VideoMediaInfo>) -> Self {
        Self { id, path, info }
    }
}

pub async fn process_mediainfo_input_channels(
    mut mediainfo_input_rx: Receiver<VideoMediaInfoChannelMessage>,
    mediainfo_output_tx: Sender<VideoMediaInfoChannelMessage>,
) -> Result<(), Error> {
    while let Some(input) = mediainfo_input_rx.recv().await {
        let media_info = create_media_info(&input.path);
        if let Ok(m) = media_info {
            debug!("Media info created: {:?}", m);
            if let Err(e) = mediainfo_output_tx
                .send(VideoMediaInfoChannelMessage::new(
                    input.id,
                    input.path,
                    Some(m),
                ))
                .await
            {
                error!("Failed to send media output: {}", e);
            }
        }
    }

    Ok(())
}

pub async fn process_mediainfo_output_channels(
    app: &AppHandle,
    mut mediainfo_output_rx: Receiver<VideoMediaInfoChannelMessage>,
) -> Result<(), Error> {
    while let Some(input) = mediainfo_output_rx.recv().await {
        let emit_message = VideoMediaInfoEmitEvent::new(input.info.unwrap());
        let _ = app.emit_all(&format!("update_mediainfo_{}", input.id), emit_message);
    }

    Ok(())
}

// Creator
fn create_media_info<P: AsRef<Path>>(video_path: P) -> Result<VideoMediaInfo, Error> {
    let mut builder = VideoMediaInfoBuilder::create_empty();
    let video_path = video_path.as_ref();
    builder.filesize(Some(util::format_file_size(
        video_path.metadata().map(|m| m.len()).unwrap_or(0),
    )));
    let input_context = thumbnail::create_input_context(video_path)?;
    if let Ok(Some((index, codec))) = input_context.find_best_stream(AVMediaType_AVMEDIA_TYPE_VIDEO)
    {
        if let Some(video_stream) = input_context.streams().get(index) {
            let params = video_stream.codecpar();
            builder.width(Some(params.width));
            builder.height(Some(params.height));
            let fps = av_q2d(video_stream.avg_frame_rate);
            builder.framerate(Some(fps));
            builder.bitrate(Some(params.bit_rate / 1024));
            let duration = video_stream.duration;
            let time_base = video_stream.time_base;
            builder.length(Some(format_duration(duration, time_base)));
            let mut codec_name = codec.long_name().to_string_lossy().to_string();
            get_codec_name(codec, &mut codec_name);
            builder.codec(Some(codec_name));
        }
    }
    if let Ok(Some((index, codec))) = input_context.find_best_stream(AVMediaType_AVMEDIA_TYPE_AUDIO)
    {
        if let Some(audio_stream) = input_context.streams().get(index) {
            let params = audio_stream.codecpar();
            builder.abitrate(Some(params.bit_rate / 1024));
            let mut codec_name = codec.long_name().to_string_lossy().to_string();
            get_codec_name(codec, &mut codec_name);
            builder.acodec(Some(codec_name));
            builder.asample(Some(params.sample_rate));
        }
    }
    Ok(builder.build()?)
}

fn get_codec_name(codec: AVCodecRef, codec_name: &mut String) {
    if codec_name.contains('/') {
        let new_codec_name = codec_name
            .split_once('/')
            .map_or(codec.name().to_string_lossy().to_string(), |c| {
                c.0.trim().to_string()
            });
        *codec_name = new_codec_name;
    }
}

fn format_duration(duration: i64, timebase: AVRational) -> String {
    let seconds = duration * i64::from(timebase.num) / i64::from(timebase.den);
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let milliseconds = (duration * 1000 * i64::from(timebase.num) / i64::from(timebase.den)) % 1000;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}
