use anyhow::{Context, Result};
use rodio::OutputStreamBuilder;

pub struct AudioBackend;

impl AudioBackend {
    pub fn initialize() -> Result<()> {
        let stream = OutputStreamBuilder::open_default_stream()
            .context("Failed to initialize default audio output device")?;

        println!(
            "Audio Device: {} Hz | {} channels",
            stream.config().sample_rate(),
            stream.config().channel_count()
        );

        Ok(())
    }
}
