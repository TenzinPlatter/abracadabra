use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

use json::{JsonValue, object};
use plotters::prelude::*;
use sonogram::{ColourGradient, ColourTheme, SpecOptionsBuilder};

use crate::audio_processing::WindowFrequencyInfo;

// pub fn write_freq_data(
//     freq_data: &Vec<WindowFrequencyInfo>,
//     filepath: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let mut f = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(true)
//         .open(filepath)?;

//     let mut json_vals = Vec::with_capacity(freq_data.len());

//     for window in freq_data.iter() {
//         let freqs: Vec<JsonValue> = window
//             .frequencies
//             .iter()
//             .map(|f| object! { intensity: f.intensity, frequency: f.hertz})
//             .collect();

//         let data = object! {
//             time: window.time_offset,
//             frequences: freqs,
//         };

//         json_vals.push(data);
//     }

//     let max = freq_data
//         .first()
//         .unwrap()
//         .frequencies
//         .iter()
//         .map(|f| f.intensity)
//         .fold(0., f64::max);

//     println!("Max itensity: {}", max);

//     let json = object! {
//         values: json_vals,
//     };

//     f.write(json.dump().as_bytes())?;

//     Ok(())
// }

/// input should be frequency, intensity
pub fn plot_frequency_intensity(
    data: &[(f64, f64)],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_intensity = data.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    let max_intensity = data
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Frequency Intensity Plot", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..22050.0, min_intensity..max_intensity)?;

    chart
        .configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Intensity")
        .draw()?;

    chart.draw_series(LineSeries::new(data.iter().copied(), &BLUE))?;

    root.present()?;
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
