use std::error::Error;

use spectrum_analyzer::{samples_fft_to_spectrum, windows::hann_window};

/// time offset is in ms
pub struct WindowFrequencyInfo {
    pub time_offset: u64,
    pub frequencies: Vec<FrequencyInfo>,
}

pub struct FrequencyInfo {
    pub hertz: u64,
    pub intensity: f64,
}

pub fn process_audio(
    samples: &mut Vec<f32>,
    sample_rate: u32,
    window_size: usize,
    overlap: f32,
) -> Result<Vec<WindowFrequencyInfo>, Box<dyn Error>> {
    let windows = split_samples_into_windows(samples, window_size, overlap)?;

    let step_in_samples: usize = (window_size as f32 * (1. - overlap)).floor() as usize;
    let step_in_ms: u64 = step_in_samples as u64 * 1000 / sample_rate as u64;

    let windows = windows
        .iter()
        .enumerate()
        .map(|(i, window)| {
            let hanned = hann_window(window);
            let window = samples_fft_to_spectrum(
                &hanned,
                sample_rate,
                spectrum_analyzer::FrequencyLimit::All,
                Some(&spectrum_analyzer::scaling::divide_by_N),
            )
            .unwrap()
            .data()
            .iter()
            .map(|(freq, mag)| FrequencyInfo {
                hertz: freq.val() as u64,
                intensity: mag.val() as f64,
            })
            .collect();

            WindowFrequencyInfo {
                time_offset: i as u64 * step_in_ms,
                frequencies: window,
            }
        })
        .collect();

    Ok(windows)
}

/// Splits samples into windows so that when applying fft, frequency data is related to time
/// will ignore samples if window size doesn't perfectly divide samples.len
/// window_size is in samples
/// overlap should be between 0-1 where 0 is no overlap and 1 is completely overlapping
fn split_samples_into_windows(
    samples: &mut Vec<f32>,
    window_size: usize,
    overlap: f32,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    if overlap >= 1. || overlap < 0. {
        return Err("Please give an overlap value between < 1 and >= 0".into());
    }

    let step: usize = (window_size as f32 * (1. - overlap)).floor() as usize;
    let mut windows = Vec::with_capacity(samples.len() / step);

    for i in (0..samples.len()).step_by(step) {
        // will give an indexing error
        if i + window_size >= samples.len() {
            break;
        }

        //TODO: a bit inefficient as it clones, maybe use some take method?
        let window = samples[i..i + window_size].to_vec();
        windows.push(window);
    }

    Ok(windows)
}
