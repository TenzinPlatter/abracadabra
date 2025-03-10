use std::error::Error;

use apodize::hamming_iter;
use rustfft::{FftPlanner, num_complex::Complex};

pub struct FrequencyInfo {
    pub freq: u32,
    pub intensity: f32,
}

pub fn magnitude_data_to_frequency_data(
    magnitude_data: Vec<Vec<f32>>,
    sample_rate: u32,
    bin_size: usize,
) -> Vec<Vec<FrequencyInfo>> {
    let mut windows: Vec<Vec<FrequencyInfo>> = Vec::with_capacity(magnitude_data.len());

    for section in magnitude_data {
        let mut window: Vec<FrequencyInfo> = Vec::with_capacity(section.len());

        for (i, bin) in section.iter().enumerate() {
            let freq = i as u32 * sample_rate / bin_size as u32;
            window.push(FrequencyInfo {
                freq,
                intensity: bin.clone(),
            });
        }

        windows.push(window);
    }

    windows
}

/// processes audio from samples to windowed frequency data
/// window_size --> number of samples for each window
/// overlap --> percentage overlap between samples, should be between 0-1
/// for music processing 75 - 87.5 is good
pub fn samples_to_magnitude_data_windows(
    samples: &mut Vec<f32>,
    window_size: usize,
    overlap: f32,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let step_size: usize = (window_size as f32 * (1. - overlap)).floor() as usize;
    let mut windows: Vec<Vec<f32>> = Vec::with_capacity(samples.len() / window_size);

    let mut start: usize = 0;
    while start + window_size <= samples.len() {
        let window = &samples[start..start + window_size];
        let mut hamming_iter = hamming_iter(window.len());

        let mut fft_buf: Vec<Complex<f32>> = Vec::with_capacity(window.len());

        for sample in window {
            let sample = sample
                * hamming_iter
                    .next()
                    .expect("give hamming iter correct size so this doesnt happen")
                    as f32;

            fft_buf.push(Complex { re: sample, im: 0. });
        }

        compute_fft(&mut fft_buf);

        let mut window_magnitudes = Vec::with_capacity(window.len());

        for complex in fft_buf {
            window_magnitudes.push(complex.norm());
        }

        windows.push(window_magnitudes);

        start += step_size;
    }

    Ok(windows)
}

fn compute_fft(buffer: &mut Vec<Complex<f32>>) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer.len());
    fft.process(buffer);
}
