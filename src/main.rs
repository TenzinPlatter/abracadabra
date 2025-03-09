use kazaam::{mic_utils::connect_to_mic, use_default_mic};

fn main() {
    connect_to_mic(use_default_mic());
}
