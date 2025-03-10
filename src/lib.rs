use std::{error::Error, fs::File, io::BufWriter};

use cpal::StreamConfig;
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};
use sonogram::{ColourGradient, ColourTheme, SpecOptionsBuilder};

pub mod mic_utils;

/// processes audio from samples to windowed frequency data
/// window_size --> number of samples for each window
/// overlap --> percentage overlap between samples, should be between 0-1
/// for music processing 75 - 87.5 is good
pub fn process_audio(
    samples: &Vec<i16>,
    config: StreamConfig,
    window_size: u16,
    overlap: f32,
) -> Result<(), Box<dyn Error>> {
    //TODO: make better fn name

    // step size must be an integer
    let step_size: u32 = (window_size as f32 * (1. - overlap)).floor() as u32;

    let windows = vec![];

    for i in (0..(samples.len() as u16 - window_size)).step_by(window_size.into()) {}

    Ok(())
}

pub fn plot_spectrum(magnitudes: &[f32], file_path: &str) {
    let root = BitMapBackend::new(file_path, (2048, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_magnitude = magnitudes.iter().cloned().fold(0.0, f32::max);
    let x_axis_max = magnitudes.len() / 2; // Plot only half (mirrored spectrum)

    let mut chart = ChartBuilder::on(&root)
        .caption("Frequency Spectrum", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..x_axis_max, 0.0..max_magnitude)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..x_axis_max).map(|x| (x, magnitudes[x])),
            &BLUE,
        ))
        .unwrap();

    root.present().unwrap();
}

pub fn save_samples_to_wav(
    samples: &Vec<i16>,
    channels: u16,
    sample_rate: u32,
    filepath: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(&filepath)?;

    let writer = BufWriter::new(file);
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav_writer = hound::WavWriter::new(writer, spec)?;

    for &sample in samples.iter() {
        wav_writer.write_sample(sample)?;
    }

    wav_writer.finalize()?;

    Ok(())
}

pub fn save_spectrograph(samples: Vec<i16>, sample_rate: u32, filepath: &str) {
    let mut spectrograph = SpecOptionsBuilder::new(2048)
        .load_data_from_memory(samples, sample_rate)
        .build()
        .unwrap();

    let mut spectrograph = spectrograph.compute();

    let mut gradient = ColourGradient::create(ColourTheme::Default);

    let png_file = std::path::Path::new(filepath);
    if let Err(e) = spectrograph.to_png(
        &png_file,
        sonogram::FrequencyScale::Linear,
        &mut gradient,
        2048,
        2048,
    ) {
        eprintln!("Error generating spectrograph: {}", e);
    } else {
        println!("Saved spectrograph to: {}", filepath);
    }
}
