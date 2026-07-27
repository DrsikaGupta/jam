use crate::library::Track;

#[derive(Default)]
pub struct PlaybackState {
    pub current_track: Option<Track>,

    pub paused: bool,

    pub elapsed_seconds: u64,

    pub duration_seconds: u64,

    pub volume: f32,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            current_track: None,
            paused: false,
            elapsed_seconds: 0,
            duration_seconds: 0,
            volume: 1.0,
        }
    }
}
