use kazaam::{
    audio_processing::{magnitude_data_to_frequency_data, samples_to_magnitude_data_windows},
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

    const BIN_SIZE: usize = 1024;

    let mag_data = match samples_to_magnitude_data_windows(&mut samples, BIN_SIZE, 0.8) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Recording error: {}", e);
            return;
        }
    };

    let freq_data = magnitude_data_to_frequency_data(mag_data, mic.config.sample_rate.0, BIN_SIZE);

    let len = freq_data[0].len();
    let mut avgs = vec![0.; len];
    for window in freq_data.iter() {
        for (i, f) in window.iter().enumerate() {
            avgs[i] += f.intensity;
        }
    }

    for i in 0..avgs.len() {
        avgs[i] /= len as f32;
    }

    for (i, f) in freq_data[0].iter().enumerate() {
        println!("average intensity {} for freq {}", avgs[i], f.freq);
    }
}
