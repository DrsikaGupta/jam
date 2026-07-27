use crate::config::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".into(),

            volume: 75,

            autoplay: true,

            visualizer_fps: 60,

            cache_size_mb: 1024,

            yt_dlp_path: "yt-dlp".into(),

            ffmpeg_path: "ffmpeg".into(),
        }
    }
}
