use kazaam::{
    mic_utils::{connect_to_mic, use_default_mic},
    plot_spectrum, process_audio, save_spectrograph,
};

fn main() {
    let mic = connect_to_mic(use_default_mic());
    let samples: Vec<i16>;
    match mic.listen() {
        Ok(s) => {
            let mut s = s.lock().unwrap();
            samples = std::mem::take(&mut *s);
        }
        Err(e) => {
            eprintln!("Recording error: {}", e);
            return;
        }
    }

    process_audio(&samples, mic.config, 1024, 0.8);

    // let fp = "assets/spectrograph.png";
    // save_spectrograph(samples, mic.config.sample_rate.0, fp);
}
