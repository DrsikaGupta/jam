use image::DynamicImage;
use std::{path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct Track {
    pub path: PathBuf,

    pub title: String,

    pub artist: Option<String>,

    pub album: Option<String>,

    pub genre: Option<String>,

    pub year: Option<u32>,

    pub duration: Option<Duration>,

    pub sample_rate: Option<u32>,

    pub bitrate: Option<u32>,

    pub artwork: Option<DynamicImage>,
}
