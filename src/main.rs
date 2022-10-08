use std::env;
use std::process::exit;

use evdev::{Device, EventType};
use processor::EventProcessor;
mod processor;

use utils::*;
mod utils;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        let mut device = find_orbweaver();
        // If ungrab fails the device was likely unplugged or not grabbed.
        match device.ungrab() {
            Ok(_) => {
                println!("Device ungrabbed.");
            }
            Err(_) => {
                println!("Failed to ungrab device.")
            }
        };
    }
}
pub fn is_orbweaver_keyboard(device: &Device) -> bool {
    return device.supported_leds().is_some()
        && device.input_id().vendor() == 0x1532
        && device.input_id().product() == 0x0207;
}

pub fn find_orbweaver() -> Device {
    match evdev::enumerate()
        .map(|t| t.1)
        .find(|d| is_orbweaver_keyboard(d))
    {
        Some(device) => device,
        None => {
            println!("No orbweaver was detected.");
            exit(-1);
        }
    }
}

fn process_events(mut device: Device, config: UserConfig) {
    let mut processor = match EventProcessor::new(config) {
        Ok(p) => p,
        Err(err) => {
            println!("{}", err);
            exit(-1);
        }
    };
    loop {
        for ev in device.fetch_events().unwrap() {
            if ev.event_type() == EventType::KEY {
                processor.process_event(ev);
            }
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("A config file must be specified.");
        exit(-1);
    }
    // Todo - Probably need better way to handle exit.
    let mut device = find_orbweaver();
    let _cleanup = Cleanup;
    let config = load_config(
        env::args()
            .nth(2)
            .expect("Failed to read config cmd line arg."),
    );

    match device.grab() {
        Ok(_) => process_events(device, config),
        Err(_) => {
            println!("Failed to grab device.");
            exit(-1)
        }
    };
}
