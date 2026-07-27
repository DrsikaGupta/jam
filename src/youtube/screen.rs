use anyhow::Result;

use super::{client::YoutubeClient, models::YoutubeTrack};

pub struct YoutubeScreen {
    /// Current search query
    pub query: String,

    /// True while editing the query
    pub search_mode: bool,

    /// True while yt-dlp is searching
    pub loading: bool,

    /// Selected search result
    pub selected: usize,

    /// Search results
    pub results: Vec<YoutubeTrack>,

    /// Youtube client
    client: YoutubeClient,
}

impl YoutubeScreen {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            search_mode: false,
            loading: false,
            selected: 0,
            results: Vec::new(),
            client: YoutubeClient::new(),
        }
    }

    pub fn search(&mut self) -> Result<()> {
        if self.query.trim().is_empty() {
            self.results.clear();
            self.selected = 0;
            return Ok(());
        }

        self.loading = true;

        match self.client.search(&self.query) {
            Ok(results) => {
                self.results = results;
                self.selected = 0;
            }

            Err(e) => {
                eprintln!("YouTube search failed: {e}");
                self.results.clear();
                self.selected = 0;
            }
        }

        self.loading = false;

        Ok(())
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.results.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn current(&self) -> Option<&YoutubeTrack> {
        self.results.get(self.selected)
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.results.clear();
        self.selected = 0;
        self.loading = false;
        self.search_mode = false;
    }
}
