use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::library::Track;

#[derive(Clone, Copy, PartialEq)]
pub enum RepeatMode {
    Off,
    All,
    One,
}

pub struct PlaybackManager {
    queue: Vec<Track>,

    // Order songs are played in
    play_order: Vec<usize>,

    // Position inside play_order
    current: usize,

    shuffle: bool,

    repeat: RepeatMode,

    history: Vec<usize>,
}

impl PlaybackManager {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            play_order: Vec::new(),
            current: 0,
            shuffle: false,
            repeat: RepeatMode::Off,
            history: Vec::new(),
        }
    }
    pub fn load(&mut self, tracks: Vec<Track>) {
        self.queue = tracks;
        self.play_order = (0..self.queue.len()).collect();
        self.current = 0;
        self.history.clear();
    }

    fn reshuffle(&mut self) {
        let mut rng = thread_rng();

        self.play_order.shuffle(&mut rng);

        self.current = 0;
    }

    pub fn current(&self) -> Option<&Track> {
        self.play_order
            .get(self.current)
            .and_then(|&i| self.queue.get(i))
    }

    pub fn select(&mut self, index: usize) -> Option<Track> {
        if index >= self.queue.len() {
            return None;
        }

        if self.shuffle {
            if let Some(pos) = self.play_order.iter().position(|&x| x == index) {
                self.current = pos;
            }
        } else {
            self.current = index;
        }

        Some(self.queue[index].clone())
    }

    pub fn next(&mut self) -> Option<Track> {
        if self.queue.is_empty() {
            return None;
        }

        if self.repeat == RepeatMode::One {
            return self.current().cloned();
        }

        self.history.push(self.current);

        self.current += 1;

        if self.current >= self.play_order.len() {
            match self.repeat {
                RepeatMode::Off => {
                    // Wrap to first song
                    self.current = 0;
                }

                RepeatMode::All => {
                    if self.shuffle {
                        let mut rng = thread_rng();
                        self.play_order.shuffle(&mut rng);
                    }

                    self.current = 0;
                }

                RepeatMode::One => {}
            }
        }

        self.current().cloned()
    }

    pub fn previous(&mut self) -> Option<Track> {
        if self.queue.is_empty() {
            return None;
        }

        if self.repeat == RepeatMode::One {
            return self.current().cloned();
        }

        if self.current == 0 {
            self.current = self.play_order.len() - 1;
        } else {
            self.current -= 1;
        }

        self.current().cloned()
    }

    pub fn toggle_shuffle(&mut self) {
        // Remember the currently playing song
        let current_track_index = self.play_order.get(self.current).copied().unwrap_or(0);

        self.shuffle = !self.shuffle;

        if self.shuffle {
            // Create a shuffled play order
            self.play_order = (0..self.queue.len()).collect();

            let mut rng = thread_rng();
            self.play_order.shuffle(&mut rng);

            // Keep the current song playing
            if let Some(pos) = self
                .play_order
                .iter()
                .position(|&i| i == current_track_index)
            {
                self.play_order.swap(0, pos);
            }

            self.current = 0;
        } else {
            // Restore normal order
            self.play_order = (0..self.queue.len()).collect();

            // Continue from the same song
            self.current = current_track_index;
        }
    }

    pub fn cycle_repeat(&mut self) {
        self.repeat = match self.repeat {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        };
    }

    pub fn shuffle(&self) -> bool {
        self.shuffle
    }

    pub fn repeat(&self) -> RepeatMode {
        self.repeat
    }
}
