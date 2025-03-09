use cpal::traits::StreamTrait;
use kazaam::{get_silent_stream, mic_utils::connect_to_mic, use_default_mic};

fn main() {
    let mic = connect_to_mic(use_default_mic());
}
