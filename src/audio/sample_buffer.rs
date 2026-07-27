use std::sync::{
    Arc,
    Mutex,
};

#[derive(Clone)]
pub struct SampleBuffer {
    inner: Arc<Mutex<Vec<f32>>>,
}

impl SampleBuffer {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, samples: &[f32]) {
        self.inner
            .lock()
            .unwrap()
            .extend_from_slice(samples);
    }

    pub fn take(&self) -> Vec<f32> {
        std::mem::take(
            &mut *self.inner.lock().unwrap()
        )
    }
}