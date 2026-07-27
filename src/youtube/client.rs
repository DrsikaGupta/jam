use anyhow::Result;

use super::{models::YoutubeTrack, search};

pub struct YoutubeClient;

impl YoutubeClient {
    pub fn new() -> Self {
        Self
    }

    pub fn search(&self, query: &str) -> Result<Vec<YoutubeTrack>> {
        search::search(query)
    }
}
