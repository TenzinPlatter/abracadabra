use std::error::Error;

use apodize::hamming_iter;
use rustfft::{FftPlanner, num_complex::Complex};

pub struct WindowFreqInfo {
    pub time_offset: u64,
    pub frequencies: Vec<FrequencyInfo>,
}

pub struct FrequencyInfo {
    pub freq: u64,
    pub intensity: f32,
}

pub struct MagnitudeInfo {
    pub intensity: f32,
    pub time_offset: u64,
}

pub fn magnitude_data_to_frequency_data(
    magnitude_data: Vec<Vec<MagnitudeInfo>>,
    sample_rate: u32,
    bin_size: usize,
) -> Vec<WindowFreqInfo> {
    let mut windows: Vec<WindowFreqInfo> = Vec::with_capacity(magnitude_data.len());

    for section in magnitude_data {
        let mut window: Vec<FrequencyInfo> = Vec::with_capacity(section.len());
        let time = section.first().unwrap().time_offset;

        for (i, bin) in section.iter().enumerate() {
            let freq = i as u64 * sample_rate as u64 / bin_size as u64;

            window.push(FrequencyInfo {
                freq,
                intensity: bin.intensity.clone(),
            });
        }

        windows.push(WindowFreqInfo {
            time_offset: time,
            frequencies: window,
        });
    }

    windows
}

pub fn no_window_mag(
    samples: &mut Vec<f32>,
    window_size: usize,
    overlap: f32,
    sample_rate: u32,
) -> Result<Vec<MagnitudeInfo>, Box<dyn Error>> {
    let mut fft_buf: Vec<Complex<f32>> = Vec::with_capacity(samples.len());

    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    let mut hamming = hamming_iter(samples.len());

    for sample in samples.iter() {
        // sample - mean to remove DC bias
        fft_buf.push(Complex {
            re: (*sample - mean) * hamming.next().unwrap() as f32,
            im: 0.,
        });
    }

    compute_fft(&mut fft_buf);

    let mut window_magnitudes = Vec::with_capacity(samples.len());

    for complex in fft_buf[..fft_buf.len() / 2].iter() {
        window_magnitudes.push(MagnitudeInfo {
            intensity: complex.norm(),
            time_offset: 0,
        });
    }

    Ok(window_magnitudes)
}

/// processes audio from samples to windowed frequency data
/// window_size --> number of samples for each window
/// overlap --> percentage overlap between samples, should be between 0-1
/// for music processing 75 - 87.5 is good
/// Return --> Vec<Vec<(magnitude, timeoffset)>>
pub fn samples_to_magnitude_data_windows(
    samples: &mut Vec<f32>,
    window_size: usize,
    overlap: f32,
    sample_rate: u32,
) -> Result<Vec<Vec<MagnitudeInfo>>, Box<dyn Error>> {
    let step_size: usize = (window_size as f32 * (1. - overlap)).floor() as usize;
    let mut windows: Vec<Vec<MagnitudeInfo>> = Vec::with_capacity(samples.len() / window_size);

    let mut start: usize = 0;
    while start + window_size <= samples.len() {
        let window = &samples[start..start + window_size];

        let mut fft_buf: Vec<Complex<f32>> = Vec::with_capacity(window.len());

        let mean: f32 = window.iter().sum::<f32>() / window.len() as f32;

        for (i, sample) in window.iter().enumerate() {
            // sample - mean to remove DC bias
            let sample = (*sample - mean)
                * (0.5
                    * (1. - f32::cos(2. * std::f32::consts::PI * i as f32 / window.len() as f32))
                        as f32);

            fft_buf.push(Complex { re: sample, im: 0. });
        }

        compute_fft(&mut fft_buf);

        let mut window_magnitudes = Vec::with_capacity(window.len());

        // cut in half for nyquist limit
        for (i, complex) in fft_buf[..fft_buf.len() / 2].iter().enumerate() {
            // offset in seconds from beginning of audio sample
            let time = i as u64 * step_size as u64 / sample_rate as u64;

            window_magnitudes.push(MagnitudeInfo {
                intensity: complex.norm(),
                time_offset: time,
            });
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
