use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeTrack {
    pub id: String,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub uploader: Option<String>,

    #[serde(default)]
    pub duration: Option<u64>,

    #[serde(default)]
    pub thumbnail: Option<String>,
}

use crate::library::Track;
use std::{path::PathBuf, time::Duration};

impl YoutubeTrack {
    pub fn into_track(self, path: PathBuf) -> Track {
        Track {
            path,

            title: self.title,

            artist: self.uploader,

            album: None,

            genre: None,

            year: None,

            duration: self.duration.map(Duration::from_secs),

            sample_rate: None,

            bitrate: None,

            artwork: None,
        }
    }
}
