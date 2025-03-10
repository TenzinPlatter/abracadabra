use std::{error::Error, fs::File, io::BufWriter};

use plotters::prelude::*;
use sonogram::{ColourGradient, ColourTheme, SpecOptionsBuilder};

pub mod audio_processing;
pub mod mic_utils;

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
