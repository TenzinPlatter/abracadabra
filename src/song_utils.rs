use std::{error::Error, fs::File, io::BufReader};

use symphonia::core::{
    codecs::{CODEC_TYPE_NULL, DecoderOptions},
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

pub struct SongInfo {
    //TODO:
    // fingerprint: ?
    pub uid: u32,
    pub name: String,
    pub artist: String,
    pub mp3_path: String,
}

//TODO: database storage

/// to be implemented: save to a database, rn will save to assets/songs
pub fn save_song_to_db(song_info: SongInfo) -> Result<(), Box<dyn Error>> {
    let file = File::open(song_info.mp3_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut prober_hint = Hint::new();
    prober_hint.with_extension("mp3");

    let meta_opts: MetadataOptions = Default::default();
    let mut fmt_opts: FormatOptions = Default::default();
    fmt_opts.enable_gapless = true;

    let probed_source =
        symphonia::default::get_probe().format(&prober_hint, mss, &fmt_opts, &meta_opts)?;

    let mut format = probed_source.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("No supported audio tracks");

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("Unsupported codec");

    let track_id = track.id;

    // decode loop
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,

            Err(symphonia::core::errors::Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }

            Err(err) => {
                // unrecoverable error
                panic!("{}", err);
            }
        };

        // consume new metadata
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(_decoded) => {
                //TODO:
            }

            Err(symphonia::core::errors::Error::IoError(_)) => {
                continue;
            }

            Err(symphonia::core::errors::Error::DecodeError(_)) => {
                continue;
            }

            Err(err) => {
                // unrecoverable
                panic!("{}", err);
            }
        }
    }
}
