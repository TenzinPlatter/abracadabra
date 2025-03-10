use cpal::{
    Device, Host, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::{
    io::{self},
    sync::{Arc, Mutex},
};

pub struct MicInfo {
    pub device: Device,
    pub config: StreamConfig,
}

impl MicInfo {
    pub fn listen(&self) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        println!(
            "Recording with {}Hz, {:?}-bit",
            self.config.sample_rate.0, self.config.channels
        );

        let samples = Arc::new(Mutex::new(Vec::<i16>::new()));
        let samples_clone = Arc::clone(&samples);

        let write_data_fn = move |data: &[i16], _: &cpal::InputCallbackInfo| {
            let mut buffer = samples_clone.lock().unwrap();
            buffer.extend_from_slice(data);
        };

        let err_fn = |err| eprintln!("Stream Error: {}", err);

        let stream = self
            .device
            .build_input_stream(&self.config, write_data_fn, err_fn, None)?;

        println!("Started recording, press enter to stop");
        stream.play()?;

        // will block until user presses enter
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // stop recording
        drop(stream);

        let samples: Vec<f32> = samples
            .lock()
            .unwrap()
            .iter()
            // cast both so there is no integer division
            .map(|s| *s as f32 / i16::MAX as f32)
            .collect();

        Ok(samples)
    }
}

fn get_config(mic: &Device) -> StreamConfig {
    mic.default_input_config()
        .expect("No default input config?")
        .into()
}

pub fn connect_to_mic(use_default_mic: bool) -> MicInfo {
    println!("Connecting to mic...");

    let host = cpal::default_host();
    let mic: Device;

    if use_default_mic {
        mic = host.default_input_device().unwrap();
    } else {
        mic = choose_mic(&host);
    }

    println!(
        "Using microphone: {}",
        mic.name().expect("Default device should have name")
    );

    let config = get_config(&mic);

    MicInfo {
        device: mic,
        config,
    }
}

fn choose_mic(host: &Host) -> Device {
    let mut num_of_devices = 0;
    for (i, device) in host.input_devices().expect("What").into_iter().enumerate() {
        println!("{}: {}", i + 1, device.name().expect("Device without name"));
        num_of_devices += 1;
    }

    println!("Please enter the corresponding number for your chosen device");

    let choice: u32;
    loop {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        if let Ok(n) = input_line.trim().parse::<u32>() {
            if n as u32 <= num_of_devices {
                // corrects from 1 based to 0 based index
                choice = n - 1;
                break;
            } else {
                eprintln!("Please enter a valid choice");
            }
        } else {
            eprintln!("Please enter a number");
        }
    }

    let mut mic = host.default_input_device().unwrap();
    for (i, device) in host.input_devices().unwrap().into_iter().enumerate() {
        if i as u32 == choice {
            mic = device;
        }
    }

    mic
}

pub fn use_default_mic() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return true;
    } else {
        // if passing -d flag then don't use default mic
        args[1].trim() != "-d"
    }
}
