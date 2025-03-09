use cpal::{
    Device, Host,
    traits::{DeviceTrait, HostTrait},
};
use std::io;

pub fn connect_to_mic(use_default_mic: bool) {
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
}

pub fn choose_mic(host: &Host) -> Device {
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
