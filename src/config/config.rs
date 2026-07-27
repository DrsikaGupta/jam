use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,

    pub volume: u8,

    pub autoplay: bool,

    pub visualizer_fps: u16,

    pub cache_size_mb: u32,

    pub yt_dlp_path: String,

    pub ffmpeg_path: String,
}
