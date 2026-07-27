use std::time::Duration;

use rodio::Source;

use crate::audio::Visualizer;

/// Wraps a Rodio source and taps decoded PCM samples
/// for the visualizer while still forwarding them
/// to the audio output.
pub struct VisualizerSource<S>
where
    S: Source<Item = f32>,
{
    inner: S,
    visualizer: Visualizer,
    buffer: Vec<f32>,
}

impl<S> VisualizerSource<S>
where
    S: Source<Item = f32>,
{
    pub fn new(inner: S, visualizer: Visualizer) -> Self {
        Self {
            inner,
            visualizer,
            buffer: Vec::with_capacity(4096),
        }
    }
}

impl<S> Iterator for VisualizerSource<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;

        self.buffer.push(sample);

        // Update roughly every 4096 samples.
        if self.buffer.len() >= 4096 {
            self.visualizer.update(&self.buffer);
            self.buffer.clear();
        }

        Some(sample)
    }
}

impl<S> Source for VisualizerSource<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }

    fn try_seek(&mut self, position: Duration) -> Result<(), rodio::source::SeekError> {
        self.inner.try_seek(position)
    }
}
