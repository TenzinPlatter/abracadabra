use kazaam::{mic_utils::connect_to_mic, use_default_mic};

fn main() {
    let mic = connect_to_mic(use_default_mic());
    match mic.listen() {
        Ok(fp) => println!("Recording written to: {}", fp),
        Err(e) => eprintln!("Recording error: {}", e),
    }
}
