use anyhow::Result;

use crate::{
    library::{LibraryScanner, Track, filesystem::library_root},
    search::engine::SearchEngine,
};

pub struct LocalMusicScreen {
    /// Complete library
    pub tracks: Vec<Track>,

    /// Indices of tracks currently visible
    pub filtered: Vec<usize>,

    /// Selected row in filtered
    pub selected: usize,

    /// Current search query
    pub query: String,

    /// Whether search input is active
    pub search_mode: bool,

    /// Search engine
    pub search: SearchEngine,
}

impl LocalMusicScreen {
    pub fn new() -> Result<Self> {
        let root = library_root()?;

        let tracks = LibraryScanner::scan(&root)?;

        let filtered = (0..tracks.len()).collect();

        let search = SearchEngine::new(&tracks);

        Ok(Self {
            tracks,
            filtered,
            selected: 0,
            query: String::new(),
            search_mode: false,
            search,
        })
    }

    pub fn apply_filter(&mut self) {
        if self.query.is_empty() {
            self.filtered = (0..self.tracks.len()).collect();
        } else {
            self.filtered = self.search.search(&self.query);
        }

        self.selected = 0;
    }

    pub fn next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.filtered.len();
    }

    pub fn previous(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        if self.selected == 0 {
            self.selected = self.filtered.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn current(&self) -> Option<&Track> {
        self.filtered
            .get(self.selected)
            .map(|&index| &self.tracks[index])
    }
}
