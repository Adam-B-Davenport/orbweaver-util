#![feature(buf_read_has_data_left)]

use std::process::exit;

use evdev::{Device, EventType};
use processor::EventProcessor;
mod processor;

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
    return evdev::enumerate()
        .map(|t| t.1)
        .find(|d| is_orbweaver_keyboard(d))
        .expect("No orbweaver was detected.");
}

fn process_events(mut device: Device) {
    let mut processor = match EventProcessor::new() {
        Ok(p) => p,
        Err(err) => {
            println!("{}", err);
            return;
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
    // Todo - Probably need better way to handle exit.
    let mut device = find_orbweaver();
    let _cleanup = Cleanup;

    match device.grab() {
        Ok(_) => process_events(device),
        Err(_) => {
            println!("Failed to grab device.");
            exit(-1)
        }
    };
}
