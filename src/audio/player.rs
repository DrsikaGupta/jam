use std::{fs::File, io::BufReader, path::Path};

use crate::audio::Visualizer;
use crate::audio::source::VisualizerSource;
use crate::audio::state::PlaybackState;
use crate::library::Track;
use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
pub struct AudioPlayer {
    stream: OutputStream,
    sink: Sink,
    state: PlaybackState,
    visualizer: Visualizer,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()?;

        let sink = Sink::connect_new(stream.mixer());

        Ok(Self {
            stream,
            sink,
            state: PlaybackState::new(),
            visualizer: Visualizer::new(),
        })
    }
    pub fn visualizer(&self) -> &Visualizer {
        &self.visualizer
    }
    pub fn play(&mut self, track: Track) -> Result<()> {
        // Stop the current song
        self.sink.stop();

        // Create a fresh sink attached to the same mixer
        self.sink = Sink::connect_new(self.stream.mixer());

        let file = BufReader::new(File::open(&track.path)?);
        let decoder = Decoder::try_from(file)?;
        let source = VisualizerSource::new(decoder, self.visualizer.clone());
        self.sink.append(source);

        self.sink.play();
        self.state.current_track = Some(track);
        self.state.paused = false;
        self.state.elapsed_seconds = 0;

        Ok(())
    }

    pub fn state(&self) -> &PlaybackState {
        &self.state
    }
    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state.paused = true;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.state.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
