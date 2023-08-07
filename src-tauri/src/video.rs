extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Error;
use derive_builder::Builder;
use derive_more::{Display, Error};
use gstreamer::prelude::*;
use gstreamer::{ClockTime, MessageType, Pipeline, State};
use serde::{Deserialize, Serialize};
use url::Url;

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

pub fn create_thumbnail_video_pipeline(
    video_url: Url,
    save_path: &PathBuf,
) -> Result<Pipeline, anyhow::Error> {
    let pipeline = gst::parse_launch(&format!(
        "uridecodebin uri={video_url} ! videoconvert ! appsink name=sink"
    ))?
    .downcast::<Pipeline>()
    .expect("Video can't be opened");

    let appsink = pipeline
        .by_name("sink")
        .expect("Sink element not found")
        .downcast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");
    appsink.set_property("sync", false);
    appsink.set_caps(Some(
        &gst_video::VideoCapsBuilder::new()
            .format(gst_video::VideoFormat::Rgbx)
            .build(),
    ));

    let mut got_snapshot = false;
    let out_path = save_path.clone();

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |appsink| {
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    gst::element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                if got_snapshot {
                    return Err(gst::FlowError::Eos);
                }
                got_snapshot = true;

                let caps = sample.caps().expect("Sample without caps");
                let info = gst_video::VideoInfo::from_caps(caps).expect("Failed to parse caps");

                let frame = gst_video::VideoFrameRef::from_buffer_ref_readable(buffer, &info)
                    .map_err(|_| {
                        gst::element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                let display_aspect_ratio = (frame.width() as f64 * info.par().numer() as f64)
                    / (frame.height() as f64 * info.par().denom() as f64);
                let target_height = 480;
                let target_width = target_height as f64 * display_aspect_ratio;

                let img = image::FlatSamples::<&[u8]> {
                    samples: frame.plane_data(0).unwrap(),
                    layout: image::flat::SampleLayout {
                        channels: 3,
                        channel_stride: 1,
                        width: frame.width(),
                        width_stride: 4,
                        height: frame.height(),
                        height_stride: frame.plane_stride()[0] as usize,
                    },
                    color_hint: Some(image::ColorType::Rgb8),
                };

                let scaled_img = image::imageops::thumbnail(
                    &img.as_view::<image::Rgb<u8>>()
                        .expect("couldn't create image view"),
                    target_width as u32,
                    target_height as u32,
                );

                scaled_img.save(&out_path).map_err(|err| {
                    gst::element_error!(
                        appsink,
                        gst::ResourceError::Write,
                        (
                            "Failed to write thumbnail file {}: {}",
                            &out_path.display(),
                            err
                        )
                    );

                    gst::FlowError::Error
                })?;

                Err(gst::FlowError::Eos)
            })
            .build(),
    );
    Ok(pipeline)
}

pub fn create_thumbnail(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Paused)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    let mut seeked = false;

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::AsyncDone(..) => {
                if !seeked {
                    // AsyncDone means that the pipeline has started now and that we can seek
                    let duration = pipeline.query_duration::<gst::ClockTime>().unwrap();
                    let seek_position = duration / 2;
                    println!("Got AsyncDone message, seeking to {}s", seek_position);

                    if pipeline
                        .seek_simple(gst::SeekFlags::FLUSH, seek_position)
                        .is_err()
                    {
                        println!("Failed to seek, taking first frame");
                    }

                    pipeline.set_state(gst::State::Playing)?;
                    seeked = true;
                } else {
                    println!("Got second AsyncDone message, seek finished");
                }
            }
            MessageView::Eos(..) => {
                // The End-of-stream message is posted when the stream is done, which in our case
                // happens immediately after creating the thumbnail because we return
                // gst::FlowError::Eos then.
                println!("Got Eos message, done");
                break;
            }
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| gstreamer::glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

#[derive(Builder, Serialize, Deserialize)]
pub struct VideoMetadata {
    width: Option<u32>,
    height: Option<u32>,
    framerate: Option<String>,
    filesize: Option<u64>,
    bitrate: Option<u64>,
    length: Option<String>,
    codec: Option<String>,
    abitrate: Option<u32>,
    acodec: Option<String>,
    asample: Option<String>,
}

pub(crate) async fn create_metadata(url: Url) -> Result<VideoMetadata, Error> {
    // Create a pipeline to read the file and get its information
    let pipeline =
        gstreamer::parse_launch(&format!("filesrc location={} ! decodebin ! fakesink", url))?
            .downcast::<Pipeline>()
            .unwrap();
    let bus = pipeline.bus().unwrap();

    // Set the pipeline to the PAUSED state to get its information
    pipeline.set_state(State::Paused).unwrap();

    // Wait for the pipeline to preroll
    bus.timed_pop_filtered(ClockTime::NONE, &[MessageType::AsyncDone])
        .unwrap();

    // Get the video and audio information from the pipeline
    let mut video_info = None;
    let mut audio_info = None;
    for element_result in pipeline.iterate_elements() {
        if let Ok(element) = element_result {
            let pads = element.sink_pads();
            for pad in pads {
                if let Some(caps) = pad.caps() {
                    if caps.structure(0).unwrap().name().starts_with("video") {
                        video_info =
                            Some(gstreamer_video::VideoInfo::from_caps(caps.as_ref()).unwrap());
                    } else if caps.structure(0).unwrap().name().starts_with("audio") {
                        audio_info =
                            Some(gstreamer_audio::AudioInfo::from_caps(caps.as_ref()).unwrap());
                    }
                }
            }
        }
    }

    let mut m = VideoMetadataBuilder::create_empty();
    if let Some(video_info) = video_info {
        m.width(Some(video_info.width()));
        m.height(Some(video_info.height()));
        m.framerate(Some(format!(
            "{}.{}",
            video_info.fps().numer(),
            video_info.fps().denom()
        )));
        let filesize = url.to_file_path().unwrap().metadata()?.len();
        m.filesize(Some(filesize));
        let duration = pipeline.query_duration::<ClockTime>().unwrap().seconds() as f64;
        let bitrate = (filesize as f64 / duration) as u64;
        m.bitrate(Some(bitrate));
        m.length(Some((duration as u64).to_string()));
        m.codec(Some(video_info.format_info().to_string()));
    }

    if let Some(audio_info) = audio_info {
        m.acodec(Some(audio_info.format_info().to_string()));
        m.abitrate(Some(audio_info.rate()));
    }

    // Stop the pipeline
    pipeline.set_state(State::Null).unwrap();
    match m.build() {
        Ok(r) => Ok(r),
        Err(r) => Err(Error::msg(r.to_string())),
    }
}
