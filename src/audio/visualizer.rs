use std::sync::{Arc, Mutex};

use rustfft::{Fft, FftPlanner, num_complex::Complex};

const FFT_SIZE: usize = 4096;
const BAR_COUNT: usize = 96;

#[derive(Clone)]
pub struct Visualizer {
    bars: Arc<Mutex<Vec<f32>>>,
    peaks: Arc<Mutex<Vec<f32>>>,
    fft: Arc<dyn Fft<f32>>,
    band_map: Vec<(usize, usize)>,
}

impl Visualizer {
    pub fn new() -> Self {
        let mut planner = FftPlanner::<f32>::new();

        let fft = planner.plan_fft_forward(FFT_SIZE);

        //--------------------------------------------------
        // CAVA logarithmic frequency distribution
        //--------------------------------------------------

        const SAMPLE_RATE: f32 = 44_100.0;
        const LOW_CUTOFF: f32 = 50.0;
        const HIGH_CUTOFF: f32 = 18_000.0;

        let nyquist = SAMPLE_RATE / 2.0;

        let frequency_constant =
            (LOW_CUTOFF / HIGH_CUTOFF).log10() / (1.0 / ((BAR_COUNT + 1) as f32) - 1.0);

        let mut band_map = Vec::with_capacity(BAR_COUNT);

        let mut previous_bin = 1;

        for n in 0..BAR_COUNT {
            let coeff = -frequency_constant
                + ((n + 1) as f32 / (BAR_COUNT + 1) as f32) * frequency_constant;

            let cutoff = HIGH_CUTOFF * 10f32.powf(coeff);

            let mut bin = ((cutoff / nyquist) * (FFT_SIZE as f32 / 2.0)).ceil() as usize;

            bin = bin.max(previous_bin + 1);

            band_map.push((previous_bin, bin));

            previous_bin = bin;
        }

        Self {
            bars: Arc::new(Mutex::new(vec![0.0; BAR_COUNT])),
            peaks: Arc::new(Mutex::new(vec![0.0; BAR_COUNT])),
            fft,
            band_map,
        }
    }
    pub fn update(&self, samples: &[f32]) {
        if samples.len() < FFT_SIZE {
            return;
        }

        //--------------------------------------------------
        // Build FFT input
        //--------------------------------------------------

        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .take(FFT_SIZE)
            .enumerate()
            .map(|(i, sample)| {
                // Hann window
                let window =
                    0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / FFT_SIZE as f32).cos());

                Complex {
                    re: sample * window,
                    im: 0.0,
                }
            })
            .collect();

        //--------------------------------------------------
        // FFT
        //--------------------------------------------------

        self.fft.process(&mut input);

        //--------------------------------------------------
        // Convert frequency bins into bars
        //--------------------------------------------------

        let mut bars = self.bars.lock().unwrap();
        let mut peaks = self.peaks.lock().unwrap();

        for (i, &(start, end)) in self.band_map.iter().enumerate() {
            let mut energy = 0.0;

            for bin in &input[start..end] {
                energy += bin.norm_sqr();
            }

            energy /= (end - start) as f32;

            //----------------------------------------
            // CAVA-style logarithmic scaling
            //----------------------------------------

            let mut value = energy.sqrt();

            value = (value + 1.0).ln();

            //----------------------------------------
            // Equalizer
            //----------------------------------------

            let t = i as f32 / BAR_COUNT as f32;

            // progressively boost highs
            value *= 1.0 + t.powf(1.5) * 2.5;

            //----------------------------------------
            // Autosensitivity
            //----------------------------------------

            value = (value / 5.5).clamp(0.0, 1.0);

            //----------------------------------------
            // Smoothing
            //----------------------------------------

            //bars[i] = bars[i] * 0.78 + value * 0.22;
            if value > bars[i] {
                // Fast attack
                bars[i] = bars[i] * 0.20 + value * 0.80;
            } else {
                // Slow decay
                bars[i] = bars[i] * 0.96 + value * 0.04;
            }

            //----------------------------------------
            // Peak hold
            //----------------------------------------

            if bars[i] > peaks[i] {
                peaks[i] = bars[i];
            } else {
                //peaks[i] *= 0.985;
                const GRAVITY: f32 = 0.015;

                if bars[i] > peaks[i] {
                    peaks[i] = bars[i];
                } else {
                    peaks[i] -= GRAVITY;

                    if peaks[i] < bars[i] {
                        peaks[i] = bars[i];
                    }
                }
            }
        }
    }

    pub fn bars(&self) -> Vec<f32> {
        self.bars.lock().unwrap().clone()
    }
    pub fn peaks(&self) -> Vec<f32> {
        self.peaks.lock().unwrap().clone()
    }
}
