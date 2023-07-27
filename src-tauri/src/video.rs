extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Error;
use derive_more::{Display, Error};
use gstreamer::prelude::*;
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
    pub id: String,
    name: String,
    rating: usize,
    notes: String,
    watched: bool,
    category: Option<VideoCategory>,
}

impl VideoEntry {
    pub fn new(id: String, name: String, rating: usize, notes: String, watched: bool) -> Self {
        Self {
            id,
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
) -> Result<gst::Pipeline, anyhow::Error> {
    let pipeline = gst::parse_launch(&format!(
        "uridecodebin uri={video_url} ! videoconvert ! appsink name=sink"
    ))?
    .downcast::<gst::Pipeline>()
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
