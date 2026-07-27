use crate::library::Track;
use crate::tui::widgets::artwork::ArtworkWidget;

pub struct NowPlayingScreen {
    pub artwork: ArtworkWidget,
}

impl NowPlayingScreen {
    pub fn new() -> Self {
        Self {
            artwork: ArtworkWidget::new(),
        }
    }
}
