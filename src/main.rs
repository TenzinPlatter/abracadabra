use std::{collections::BTreeMap, fs::OpenOptions, io::Write};

use kazaam::{
    audio_processing::process_audio,
    graphing::plot_frequency_intensity,
    mic_utils::{connect_to_mic, use_default_mic},
};

fn main() {
    let bin_size: usize = 2048;
    let overlap = 0.8;

    let mic = connect_to_mic(use_default_mic());
    let mut samples: Vec<f32> = match mic.listen() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Recording error: {}", e);
            return;
        }
    };

    let data = process_audio(&mut samples, mic.config.sample_rate.0, bin_size, overlap)
        // map frequency from f64 to u64 for avg function
        .unwrap()
        .iter()
        .map(|window| {
            window
                .frequencies
                .iter()
                .map(|f| (f.hertz as u64, f.intensity))
                .collect()
        })
        .collect();

    let avgs = average_intensity_per_frequency(data);
    let avgs_f64: Vec<(f64, f64)> = avgs.iter().map(|(i, j)| (*i as f64, *j)).collect();

    plot_frequency_intensity(avgs_f64.as_slice(), "assets/frequency_avgs.png").unwrap();
}

fn average_intensity_per_frequency(data: Vec<Vec<(u64, f64)>>) -> Vec<(u64, f64)> {
    let mut freq_map: BTreeMap<u64, (f64, usize)> = BTreeMap::new();

    // Aggregate intensities for each frequency
    for window in data {
        for (freq, intensity) in window {
            let entry = freq_map.entry(freq).or_insert((0.0, 0));
            entry.0 += intensity; // Sum intensities
            entry.1 += 1; // Count occurrences
        }
    }

    // Compute averages and return a sorted Vec
    freq_map
        .into_iter()
        .map(|(freq, (sum_intensity, count))| (freq, sum_intensity / count as f64))
        .collect()
}

fn write_avgs(avgs: &Vec<(f64, f64)>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open("assets/avgs.txt")
        .unwrap();

    f.write(format!("{:?}", avgs).as_bytes()).unwrap();
}
