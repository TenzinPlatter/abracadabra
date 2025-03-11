use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use kazaam::{
    audio_processing::{
        WindowFreqInfo, magnitude_data_to_frequency_data, no_window_mag, process_audio,
        samples_to_magnitude_data_windows,
    },
    graphing::{plot_frequency_intensity, write_freq_data},
    mic_utils::{connect_to_mic, use_default_mic},
};

fn main() {
    let mic = connect_to_mic(use_default_mic());
    let mut samples: Vec<f32> = match mic.listen() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Recording error: {}", e);
            return;
        }
    };

    const BIN_SIZE: usize = 8096;

    // let mag_data = match samples_to_magnitude_data_windows(
    //     &mut samples,
    //     BIN_SIZE,
    //     0.8,
    //     mic.config.sample_rate.0,
    // ) {
    //     Ok(x) => x,
    //     Err(e) => {
    //         eprintln!("Recording error: {}", e);
    //         return;
    //     }
    // };

    // let freq_data = magnitude_data_to_frequency_data(mag_data, mic.config.sample_rate.0, BIN_SIZE);

    // let mut data = vec![];

    // for f in freq_data.first().unwrap().frequencies.iter() {
    //     data.push((f.freq as f64, f.intensity as f64));
    // }

    // let mut avgs = vec![0.; freq_data.first().unwrap().frequencies.len()];

    // for f in freq_data.iter() {
    //     for (i, fi) in f.frequencies.iter().enumerate() {
    //         avgs[i] += fi.intensity;
    //     }
    // }

    // let len = freq_data.first().unwrap().frequencies.len();
    // let mut freqs = freq_data.first().unwrap().frequencies.iter();

    // let avgs: Vec<(f64, f64)> = avgs
    //     .iter()
    //     .map(|i| (freqs.next().unwrap().freq as f64, *i as f64 / len as f64))
    //     .collect();

    let mut i = 1;
    while i * 2 <= samples.len() {
        i *= 2
    }

    samples.truncate(i);
    let freq_data = process_audio(&mut samples, mic.config.sample_rate.0);
    for (i, window) in freq_data.iter().enumerate() {
        let fp = format!("assets/freq_intensity{}.png", i);
        let _ = plot_frequency_intensity(&window, &fp);
    }

    // write_avgs(&avgs);

    // if let Err(e) = write_freq_data(&freq_data, "assets/freq_data.txt") {
    //     eprintln!("Writing error: {}", e);
    //     return;
    // }
}

fn write_avgs(avgs: &Vec<(f64, f64)>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open("assets/avgs.txt")
        .unwrap();

    f.write(format!("{:?}", avgs).as_bytes()).unwrap();
}
