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
    for (i, device) in host.devices().expect("What").into_iter().enumerate() {
        println!("{}: {}", i, device.name().expect("Device without name"));
    }

    println!("Please enter the corresponding number for your chosen device");

    let choice: u32;
    let num_of_devices = host.devices().iter().len();
    loop {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        if let Ok(n) = input_line.trim().parse() {
            if n <= num_of_devices as u32 {
                choice = n;
                break;
            } else {
                eprintln!("Please enter a valid choice");
            }
        } else {
            eprintln!("Please enter a number");
        }
    }

    let mut mic = host.default_input_device().unwrap();
    for (i, device) in host.devices().unwrap().into_iter().enumerate() {
        if i == choice as usize {
            mic = device;
        }
    }

    mic
}
