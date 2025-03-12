use std::{collections::BTreeMap, fs::OpenOptions, io::Write};

use kazaam::{
    audio_processing::{interleaved_to_single_channel, process_audio},
    graphing::plot_frequency_intensity,
    mic_utils::{connect_to_mic, use_default_mic},
    song_utils::save_song_to_db,
    // song_utils::{SongInfo, save_song_to_db},
};

fn main() {
    let info = SongInfo {
        uid: 1,
        name: String::from("Too late to turn back now"),
        artist: String::from("Cornelius Brother & Sister Rose"),
        mp3_path: String::from("assets/songs/too_late_to_turn_back_now.mp3"),
    };

    save_song_to_db(info).unwrap();

    // get_user_input();
}

fn get_user_input() {
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

    if mic.config.channels == 2 {
        samples = interleaved_to_single_channel(samples);
    }

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

    // frequency, intensity
    let avgs = average_intensity_per_frequency(data);
    let avgs_f64: Vec<(f64, f64)> = avgs.iter().map(|(i, j)| (*i as f64, *j)).collect();

    let mut max_f = 0;
    let mut max_i: f64 = 0.;

    for a in avgs {
        if a.1 > max_i {
            max_f = a.0;
            max_i = a.1;
        }
    }

    println!("Highest intensity of {} at {}Hz", max_i, max_f);

    // plot_frequency_intensity(avgs_f64.as_slice(), "assets/frequency_avgs.png").unwrap();
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
