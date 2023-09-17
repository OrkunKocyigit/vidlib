use std::ffi::CString;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{collections, fmt, fs, slice};

use anyhow::{bail, Context, Error};
use rsmpeg::avcodec::{AVCodec, AVCodecContext, AVPacket};
use rsmpeg::avformat::AVFormatContextInput;
use rsmpeg::avutil::{AVFrame, AVFrameWithImage, AVImage};
use rsmpeg::error::RsmpegError;
use rsmpeg::ffi;
use rsmpeg::ffi::{
    av_seek_frame, AVCodecID_AV_CODEC_ID_PNG, AVSEEK_FLAG_BACKWARD, AVSEEK_FLAG_FRAME,
};
use rsmpeg::swscale::SwsContext;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use tokio::sync;

use crate::{state, util};

// Thumbnail Cache
pub struct ThumbnailCache {
    thumbnails: collections::HashMap<String, ThumbnailEntry>,
}
impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            thumbnails: collections::HashMap::new(),
        }
    }

    pub fn add_thumbnail_entry(&mut self, id: &str, path: &PathBuf) {
        self.thumbnails
            .entry(id.to_string())
            .or_insert_with(ThumbnailEntry::new)
            .add_thumbnail(path)
    }

    pub fn get_paths(&self, id: &String) -> Option<&Vec<PathBuf>> {
        self.thumbnails.get(id).map(|t| t.paths())
    }

    pub fn remove_path(&mut self, id: &String) -> Option<ThumbnailEntry> {
        debug!("Remove path {}", id);
        self.thumbnails.remove(id)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ThumbnailEntry {
    paths: Vec<PathBuf>,
}

impl ThumbnailEntry {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn add_thumbnail<P: AsRef<Path>>(&mut self, path: P) {
        let _ = &self.paths.push(path.as_ref().to_path_buf());
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }
}

pub fn create_thumbnail_cache<P: AsRef<Path>>(thumbnail_path: P) -> ThumbnailCache {
    let mut thumbnail_cache = ThumbnailCache::new();
    for entry in fs::read_dir(thumbnail_path).unwrap() {
        let dir_entry = entry.unwrap();
        if dir_entry.path().is_file() {
            let name = dir_entry.file_name();
            let file_name = name.to_str().unwrap();
            if file_name.contains('_') {
                let mut split = file_name.split('_');
                let id = split.next().unwrap();
                let path = dir_entry.path();
                thumbnail_cache.add_thumbnail_entry(id, &path);
            }
        }
    }
    thumbnail_cache
}

pub fn get_thumbnail_save_location(app: &AppHandle) -> PathBuf {
    let path = util::get_app_dir(app);
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let thumbnail_path = path.join("thumbnail");
    fs::create_dir_all(&thumbnail_path).expect("Thumbnail folder creation failed");
    thumbnail_path
}

pub async fn find_thumbnail_path_in_cache(
    state: &tauri::State<'_, state::AppState>,
    id: &String,
) -> Option<Vec<PathBuf>> {
    debug!("Check if we have it in cache");
    if let Some(cache) = state.thumbnail_cache.lock().await.as_mut() {
        if let Some(path) = cache.get_paths(id) {
            debug!("Found thumbnail path in cache {}", id);
            if validate_thumbnail(path) {
                return Some(path.clone());
            } else {
                debug!("Thumbnail path is no longer valid");
                cache.remove_path(id);
            }
        } else {
            debug!("Thumbnail is not in the cache {}", id)
        }
    } else {
        debug!("Cache doesn't exist");
    }
    None
}

fn validate_thumbnail(v: &[PathBuf]) -> bool {
    v.iter().map(|v| v.exists() && v.is_file()).all(|v| v)
}
// Channel
#[derive(Clone)]
pub struct ThumbnailChannelMessage {
    path: PathBuf,
    id: String,
}

impl ThumbnailChannelMessage {
    pub fn new(path: PathBuf, id: String) -> Self {
        Self { path, id }
    }
}

impl fmt::Display for ThumbnailChannelMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}, path: {}", self.id, self.path.display())
    }
}

async fn create_and_send_thumbnail<P: AsRef<Path>>(
    save_location: P,
    input: ThumbnailChannelMessage,
    thumbnail_output_tx: &sync::mpsc::Sender<ThumbnailChannelMessage>,
) -> Result<PathBuf, Error> {
    let thumbnail = create_thumbnail(save_location, &input.id, input.path);
    let success = thumbnail.is_ok();
    let thumbnail_path = thumbnail.unwrap_or(PathBuf::from("./images/image_not_found.webp"));
    if let Err(e) = thumbnail_output_tx
        .send(ThumbnailChannelMessage::new(
            thumbnail_path.clone(),
            input.id,
        ))
        .await
    {
        error!("Failed to send thumbnail output: {}", e);
    }
    match success {
        true => Ok(thumbnail_path),
        false => {
            bail!("Thumbnail creation failed")
        }
    }
}

pub async fn process_thumbnail_input_channels(
    thumbnail_cache: &sync::Mutex<Option<ThumbnailCache>>,
    save_location: &PathBuf,
    mut thumbnail_input_rx: sync::mpsc::Receiver<ThumbnailChannelMessage>,
    thumbnail_output_tx: sync::mpsc::Sender<ThumbnailChannelMessage>,
) -> Result<(), Error> {
    while let Some(input) = thumbnail_input_rx.recv().await {
        debug!("Message received in thumbnail input {}", input);
        let id = &input.id.clone();
        if let Ok(path) =
            create_and_send_thumbnail(save_location, input, &thumbnail_output_tx).await
        {
            debug!("Thumbnail entry created. Adding it to cache");
            if let Some(c) = thumbnail_cache.lock().await.as_mut() {
                c.add_thumbnail_entry(id, &path);
                debug!(
                    "Thumbnail added to cache. id: {}, path: {}",
                    id,
                    path.display()
                );
            }
        }
    }
    Ok(())
}

pub async fn process_thumbnail_output_channels(
    app: &AppHandle,
    mut thumbnail_output_rx: sync::mpsc::Receiver<ThumbnailChannelMessage>,
) -> Result<(), Error> {
    while let Some(output) = thumbnail_output_rx.recv().await {
        debug!("Message received in thumbnail output {}", output);
        let _ = app.emit_all(
            &format!("update_thumbnail_{}", output.id),
            ThumbnailEmitEvent::new(output.path),
        );
    }

    Ok(())
}

// Events
#[derive(Clone, Serialize)]
pub struct ThumbnailEmitEvent {
    path: PathBuf,
}

impl ThumbnailEmitEvent {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

// Creator
fn create_thumbnail<P: AsRef<Path>, R: AsRef<Path>>(
    save_location: P,
    id: &String,
    video_location: R,
) -> Result<PathBuf, Error> {
    let file_name = format!("{}_01.png", id);
    let full_location = save_location.as_ref().join(file_name);
    generate_thumbnail(full_location, video_location)
}

fn generate_thumbnail<P: AsRef<Path>, R: AsRef<Path>>(
    save_location: P,
    video_location: R,
) -> Result<PathBuf, Error> {
    debug!(
        "Generate thumbnail started {}",
        video_location.as_ref().display()
    );
    let mut input_context = create_input_context(video_location)?;
    debug!("Input context created");
    let (video_index, mut decoder_context, seek_location) =
        create_decoder_context(&mut input_context)?;
    debug!("Decoding context created");
    seek_to_middle(&mut input_context, video_index, seek_location);
    debug!("Seeked to middle of the video");
    let thumbnail_frame =
        get_thumbnail_frame(&mut input_context, video_index, &mut decoder_context)?;
    debug!("Found thumbnail frame");
    let encoder_context = create_encoder_context(&mut decoder_context, &thumbnail_frame)?;
    debug!("Encoding context created");
    let scaled_thumbnail_packet =
        get_scaled_thumbnail_packet(decoder_context, &thumbnail_frame, encoder_context)?;
    debug!("Thumbnail image scaled");
    let data = unsafe {
        slice::from_raw_parts(
            scaled_thumbnail_packet.data,
            scaled_thumbnail_packet.size as usize,
        )
    };
    save_image(save_location, data)
}

fn get_scaled_thumbnail_packet(
    decoder_context: AVCodecContext,
    thumbnail_frame: &AVFrame,
    mut encoder_context: AVCodecContext,
) -> Result<AVPacket, Error> {
    let scaled_thumbnail_packet = {
        let mut sws_context = SwsContext::get_context(
            decoder_context.width,
            decoder_context.height,
            decoder_context.pix_fmt,
            encoder_context.width,
            encoder_context.height,
            encoder_context.pix_fmt,
            ffi::SWS_LANCZOS,
        )
        .context("Can't create software scaler context")?;
        let image_buffer = AVImage::new(
            encoder_context.pix_fmt,
            encoder_context.width,
            encoder_context.height,
            1,
        )
        .context("Can't create image buffer")?;
        let mut scaled_thumbnail_frame = AVFrameWithImage::new(image_buffer);
        sws_context.scale_frame(
            thumbnail_frame,
            0,
            decoder_context.height,
            &mut scaled_thumbnail_frame,
        )?;
        encoder_context.send_frame(Some(&scaled_thumbnail_frame))?;
        encoder_context.receive_packet()?
    };
    Ok(scaled_thumbnail_packet)
}

fn create_encoder_context(
    decoder_context: &mut AVCodecContext,
    thumbnail_frame: &AVFrame,
) -> Result<AVCodecContext, Error> {
    let encoder_context = {
        let encoder_codec =
            AVCodec::find_encoder(AVCodecID_AV_CODEC_ID_PNG).context("Can't find the encoder")?;
        let mut encoder_context = AVCodecContext::new(&encoder_codec);
        encoder_context.set_bit_rate(decoder_context.bit_rate);
        let display_aspect_ratio = thumbnail_frame.width as f64 / thumbnail_frame.height as f64;
        encoder_context.set_width((450f64 * display_aspect_ratio) as i32);
        encoder_context.set_height(450);
        encoder_context.set_time_base(ffi::av_inv_q(decoder_context.framerate));
        encoder_context.set_pix_fmt(if let Some(pix_fmts) = encoder_codec.pix_fmts() {
            pix_fmts[0]
        } else {
            decoder_context.pix_fmt
        });
        encoder_context
            .open(None)
            .context("Can't open encoder context")?;
        encoder_context
    };
    Ok(encoder_context)
}

fn get_thumbnail_frame(
    input_context: &mut AVFormatContextInput,
    video_index: usize,
    decoder_context: &mut AVCodecContext,
) -> Result<AVFrame, Error> {
    let thumbnail_frame = loop {
        let thumbnail_packet = loop {
            match input_context.read_packet()? {
                Some(x) if x.stream_index != video_index as i32 => {}
                x => break x,
            }
        };
        debug!("Sending packet to decoder");
        decoder_context.send_packet(thumbnail_packet.as_ref())?;
        match decoder_context.receive_frame() {
            Ok(x) => break x,
            Err(RsmpegError::DecoderDrainError) => {}
            Err(e) => bail!(e),
        }
        if thumbnail_packet.is_none() {
            bail!("Can't find video cover frame");
        }
    };
    Ok(thumbnail_frame)
}

fn seek_to_middle(
    input_context: &mut AVFormatContextInput,
    video_index: usize,
    seek_location: i64,
) {
    unsafe {
        av_seek_frame(
            input_context.as_mut_ptr(),
            video_index as i32,
            seek_location,
            (AVSEEK_FLAG_BACKWARD | AVSEEK_FLAG_FRAME) as i32,
        )
    };
}

pub fn create_input_context<P: AsRef<Path>>(
    video_location: P,
) -> Result<AVFormatContextInput, Error> {
    let input_context = AVFormatContextInput::open(
        &CString::new(video_location.as_ref().to_string_lossy().as_bytes())
            .context("Video location can't be converted")?,
    )
    .context("Video file input context failed")?;
    Ok(input_context)
}

fn create_decoder_context(
    input_context: &mut AVFormatContextInput,
) -> Result<(usize, AVCodecContext, i64), Error> {
    let (video_index, video_codec) = input_context
        .find_best_stream(ffi::AVMediaType_AVMEDIA_TYPE_VIDEO)
        .context("Video stream can't be find")?
        .context("Video stream codec can't be find")?;
    let (decoder_context, seek_location) = {
        let video_stream = input_context
            .streams()
            .get(video_index)
            .context("Video stream with index can't be find")?;
        let mut codec_context = AVCodecContext::new(&video_codec);
        let codec_parameters = video_stream.codecpar();
        codec_context
            .apply_codecpar(&codec_parameters)
            .context("Failed to apply codec parameters")?;
        codec_context
            .open(None)
            .context("Can't open the codec context")?;
        (codec_context, video_stream.duration / 2)
    };
    Ok((video_index, decoder_context, seek_location))
}

fn save_image<P: AsRef<Path>>(save_location: P, data: &[u8]) -> Result<PathBuf, Error> {
    debug!("Start saving image");
    let save_location = save_location.as_ref();
    let mut file = File::create(save_location)?;
    file.write_all(data)?;
    info!("Thumbnail created successfully {}", save_location.display());
    Ok(save_location.to_path_buf())
}
