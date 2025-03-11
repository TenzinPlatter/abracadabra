use std::error::Error;

/// time offset is in ms
///
struct WindowFrequencyInfo {
    pub time_offset: u64,
    pub frequencies: Vec<FrequencyInfo>,
}

struct FrequencyInfo {
    pub hertz: u64,
    pub intensity: f64,
}

pub fn process_audio(
    samples: &mut Vec<f32>,
    sample_rate: u32,
) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let windows = split_data_into_windows(samples, 2048, 0.8);
    let hanned = spectrum_analyzer::windows::hann_window(samples);
    let spectrum = spectrum_analyzer::samples_fft_to_spectrum(
        &windowed,
        sample_rate,
        spectrum_analyzer::FrequencyLimit::All,
        Some(&spectrum_analyzer::scaling::divide_by_N),
    )
    .unwrap()
    .data()
    .iter()
    .map(|(freq, mag)| (freq.val() as f64, mag.val() as f64))
    .collect();

    Ok(spectrum)
}

/// Splits samples into windows so that when applying fft, frequency data is related to time
/// window_size is in samples
/// overlap should be between 0-1 where 0 is no overlap and 1 is completely overlapping
fn split_data_into_windows(
    samples: &mut Vec<f32>,
    window_size: u32,
    overlap: f32,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    if overlap >= 1 || overlap < 0 {
        return Err("Please give an overlap value between < 1 and >= 0".into());
    }

    Ok(vec![])
}
