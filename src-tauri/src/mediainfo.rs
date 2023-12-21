use std::path::{Path, PathBuf};

use anyhow::Error;
use rsmpeg::avcodec::{AVCodecContext, AVCodecParametersRef, AVCodecRef};
use rsmpeg::avutil::av_q2d;
use rsmpeg::ffi::{
    av_get_bits_per_sample, AVMediaType_AVMEDIA_TYPE_AUDIO, AVMediaType_AVMEDIA_TYPE_VIDEO,
    AV_TIME_BASE,
};
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
    bitrate: Option<String>,
    length: String,
    #[builder(default = "None")]
    codec: Option<String>,
    #[builder(default = "None")]
    abitrate: Option<String>,
    #[builder(default = "None")]
    acodec: Option<String>,
    #[builder(default = "None")]
    asample: Option<String>,
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
    debug!("Media info input channel started");
    while let Some(input) = mediainfo_input_rx.recv().await {
        debug!("Media info input message received");
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
    debug!("Media info output channel started");
    while let Some(input) = mediainfo_output_rx.recv().await {
        debug!("Media info output message received");
        let emit_message = VideoMediaInfoEmitEvent::new(input.info.unwrap());
        let _ = app.emit_all(&format!("update_mediainfo_{}", input.id), emit_message);
        debug!("Media info output message send for: {}", input.id);
    }

    Ok(())
}

// Creator
fn create_media_info<P: AsRef<Path>>(video_path: P) -> Result<VideoMediaInfo, Error> {
    let mut builder = VideoMediaInfoBuilder::create_empty();
    let video_path = video_path.as_ref();
    debug!("Media info creation started for {}", video_path.display());
    builder.filesize(Some(util::format_file_size(
        video_path.metadata().map(|m| m.len()).unwrap_or(0),
    )));
    let input_context = thumbnail::create_input_context(video_path)?;
    debug!("Input context created");
    builder.length(format_duration(input_context.duration));
    if input_context.bit_rate > 0 {
        builder.bitrate(Some(format!("{} kb/s", input_context.bit_rate / 1000)));
    }
    if let Ok(Some((index, codec))) = input_context.find_best_stream(AVMediaType_AVMEDIA_TYPE_VIDEO)
    {
        if let Some(video_stream) = input_context.streams().get(index) {
            debug!("Video stream found video information will be created");
            builder.codec(get_codec_name(&codec));
            let params = video_stream.codecpar();
            if params.width > 0 {
                builder.width(Some(params.width));
                builder.height(Some(params.height));
            }
            if params.bit_rate > 0 {
                builder.bitrate(Some(format!("{} kb/s", params.bit_rate / 1000)));
            } else {
                calculate_bit_rate(&mut builder, &codec, &params);
            }
            let fps = av_q2d(video_stream.avg_frame_rate);
            if fps > 0.0 {
                builder.framerate(Some((fps * 100.0).round() / 100.0));
            }
            debug!("Video info creation done");
        }
    }
    if let Ok(Some((index, codec))) = input_context.find_best_stream(AVMediaType_AVMEDIA_TYPE_AUDIO)
    {
        if let Some(audio_stream) = input_context.streams().get(index) {
            debug!("Audio stream found video information will be created");
            builder.acodec(get_codec_name(&codec));
            let params = audio_stream.codecpar();
            if params.sample_rate > 0 {
                builder.asample(Some(format!("{} Hz", params.sample_rate)));
            }

            let mut bit_rate;
            let bits_per_sample;
            unsafe {
                bits_per_sample = av_get_bits_per_sample(codec.id) as i64;
            }
            if bits_per_sample > 0 {
                bit_rate = params.sample_rate as i64 * params.ch_layout.nb_channels as i64;
                if bit_rate > i64::MAX / bits_per_sample {
                    bit_rate = 0;
                } else {
                    bit_rate *= bits_per_sample;
                }
            } else {
                bit_rate = params.bit_rate;
            }
            if bit_rate > 0 {
                builder.abitrate(Some(format!("{} kb/s", bit_rate / 1000)));
            } else {
                calculate_bit_rate(&mut builder, &codec, &params);
            }
            debug!("Audio info creation done");
        }
    }
    Ok(builder.build()?)
}

fn calculate_bit_rate(
    builder: &mut VideoMediaInfoBuilder,
    codec: &AVCodecRef,
    params: &AVCodecParametersRef,
) {
    let mut codec_context = AVCodecContext::new(codec);
    if codec_context.apply_codecpar(params).is_ok()
        && codec_context.open(None).is_ok()
        && codec_context.rc_max_rate > 0
    {
        builder.bitrate(Some(format!("{} kb/s", codec_context.rc_max_rate / 1000)));
    }
}

fn get_codec_name(codec: &AVCodecRef) -> Option<String> {
    let mut codec_name = codec.long_name().to_string_lossy().to_string();
    if codec_name.contains('/') {
        let new_codec_name = codec_name
            .split_once('/')
            .map_or(codec.name().to_string_lossy().to_string(), |c| {
                c.0.trim().to_string()
            });
        codec_name = new_codec_name;
    }
    Some(codec_name)
}

fn format_duration(duration: i64) -> String {
    let duration = duration + (if duration <= i64::MAX - 5000 { 5000 } else { 0 });
    let mut seconds = duration / AV_TIME_BASE as i64;
    let us = duration % AV_TIME_BASE as i64;
    let milliseconds = (100 * us) / AV_TIME_BASE as i64;
    let mut minutes = seconds / 60;
    seconds %= 60;
    let hours = minutes / 60;
    minutes %= 60;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}
