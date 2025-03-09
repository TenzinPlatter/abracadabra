use cpal::{
    Sample, SampleFormat, Stream,
    traits::{DeviceTrait, HostTrait},
};
use mic_utils::MicInfo;

pub mod mic_utils;
pub mod wav_writer;

pub fn use_default_mic() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return true;
    } else {
        // if passing -d flag then don't use default mic
        args[1].trim() != "-d"
    }
}
