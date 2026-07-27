use std::path::Path;

use anyhow::Result;

pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct AudioDecoder;

impl AudioDecoder {
    pub fn decode<P: AsRef<Path>>(path: P) -> Result<DecodedAudio> {
        todo!()
    }
}
