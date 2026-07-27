use std::collections::HashMap;

use crate::library::Track;

use super::tokenizer::tokenize;

pub struct SearchIndex {
    // token -> tracks containing token
    index: HashMap<String, Vec<usize>>,

    // every unique searchable word
    vocabulary: Vec<String>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            vocabulary: Vec::new(),
        }
    }

    pub fn build(&mut self, tracks: &[Track]) {
        self.index.clear();
        self.vocabulary.clear();

        for (id, track) in tracks.iter().enumerate() {
            let mut text = track.title.clone();

            if let Some(artist) = &track.artist {
                text.push(' ');
                text.push_str(artist);
            }

            if let Some(album) = &track.album {
                text.push(' ');
                text.push_str(album);
            }

            for token in tokenize(&text) {
                // Insert into vocabulary only once
                if !self.index.contains_key(&token) {
                    self.vocabulary.push(token.clone());
                }

                self.index.entry(token).or_default().push(id);
            }
        }
    }

    pub fn lookup(&self, token: &str) -> Option<&Vec<usize>> {
        self.index.get(token)
    }

    pub fn vocabulary(&self) -> &[String] {
        &self.vocabulary
    }
}
