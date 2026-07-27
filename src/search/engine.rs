use std::collections::{HashMap, HashSet};

use crate::library::Track;

use super::{fuzzy::similarity, index::SearchIndex, tokenizer::tokenize, trie::Trie};

pub struct SearchResult {
    pub track: usize,
    pub score: f32,
}

pub struct SearchEngine {
    index: SearchIndex,
    trie: Trie,
}

impl SearchEngine {
    pub fn new(tracks: &[Track]) -> Self {
        let mut index = SearchIndex::new();
        index.build(tracks);

        let mut trie = Trie::new();

        for word in index.vocabulary() {
            trie.insert(word);
        }

        Self { index, trie }
    }

    pub fn search(&self, query: &str) -> Vec<usize> {
        let mut scores: HashMap<usize, f32> = HashMap::new();

        let query = query.to_lowercase();

        //-----------------------------------
        // Exact
        //-----------------------------------

        for token in tokenize(&query) {
            if let Some(ids) = self.index.lookup(&token) {
                for &id in ids {
                    *scores.entry(id).or_insert(0.0) += 100.0;
                }
            }
        }

        //-----------------------------------
        // Prefix (Trie)
        //-----------------------------------

        for word in self.trie.starts_with(&query) {
            if let Some(ids) = self.index.lookup(&word) {
                for &id in ids {
                    *scores.entry(id).or_insert(0.0) += 80.0;
                }
            }
        }

        //-----------------------------------
        // Substring
        //-----------------------------------

        for word in self.index.vocabulary() {
            if word.contains(&query) {
                if let Some(ids) = self.index.lookup(word) {
                    for &id in ids {
                        *scores.entry(id).or_insert(0.0) += 60.0;
                    }
                }
            }
        }

        //-----------------------------------
        // Fuzzy (only if few results)
        //-----------------------------------

        if scores.len() < 5 {
            let mut seen = HashSet::new();

            for word in self.index.vocabulary() {
                let score = similarity(&query, word);

                if score > 0.82 {
                    if let Some(ids) = self.index.lookup(word) {
                        for &id in ids {
                            if seen.insert(id) {
                                *scores.entry(id).or_insert(0.0) += (score * 50.0) as f32;
                            }
                        }
                    }
                }
            }
        }

        //-----------------------------------
        // Sort
        //-----------------------------------

        let mut ranked: Vec<(usize, f32)> = scores.into_iter().collect();

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        ranked.into_iter().map(|x| x.0).collect()
    }

    pub fn suggestions(&self, prefix: &str) -> Vec<String> {
        self.trie.starts_with(&prefix.to_lowercase())
    }
}
