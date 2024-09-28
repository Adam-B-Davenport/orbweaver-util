use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, EventType, InputEvent, Key,
};
use rand::Rng;

use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
    time::Duration,
};
use std::{io, thread};

use std::sync::mpsc;

use crate::utils::{create_str_map, ConfigStruct, KeyStruct, KeyType, UserConfig};

const DELAY: u64 = 50;
const DELTA: u64 = 25;

// The default code map for the orbweaver to map to the number on the gamepad
fn default_code_map() -> HashMap<u16, u16> {
    return HashMap::from(
        [
            (Key::KEY_GRAVE, 1),
            (Key::KEY_1, 2),
            (Key::KEY_2, 3),
            (Key::KEY_3, 4),
            (Key::KEY_4, 5),
            (Key::KEY_TAB, 6),
            (Key::KEY_Q, 7),
            (Key::KEY_W, 8),
            (Key::KEY_E, 9),
            (Key::KEY_R, 10),
            (Key::KEY_CAPSLOCK, 11),
            (Key::KEY_A, 12),
            (Key::KEY_S, 13),
            (Key::KEY_D, 14),
            (Key::KEY_F, 15),
            (Key::KEY_LEFTSHIFT, 16),
            (Key::KEY_Z, 17),
            (Key::KEY_X, 18),
            (Key::KEY_C, 19),
            (Key::KEY_V, 20),
            (Key::KEY_UP, 21),
            (Key::KEY_LEFT, 22),
            (Key::KEY_DOWN, 23),
            (Key::KEY_RIGHT, 24),
            (Key::KEY_SPACE, 25),
            (Key::KEY_LEFTALT, 26),
        ]
        .map(|(key, code)| (key.code(), code)),
    );
}

fn create_key_set(key_map: &HashMap<String, u16>) -> AttributeSet<Key> {
    let mut attribute_set: AttributeSet<Key> = AttributeSet::new();
    for code in key_map.values() {
        attribute_set.insert(Key(*code));
    }
    return attribute_set;
}

fn load_keymap(
    default_map: &HashMap<u16, u16>,
    user_keymap: &HashMap<u16, ConfigStruct>,
    key_codes: &HashMap<String, u16>,
) -> HashMap<u16, KeyStruct> {
    // default_map => user_map => key_codes
    let mut keymap: HashMap<u16, KeyStruct> = HashMap::new();
    for (key, value) in default_map.iter() {
        let map_struct = match user_keymap.get(&value) {
            Some(v) => v,
            None => {
                panic!("Invalid list of key codes.")
            }
        };
        let key_code = match key_codes.get(&map_struct.key_str) {
            Some(code) => code,
            None => {
                panic!("Invalid keymap.")
            }
        };
        keymap.insert(
            key.clone(),
            KeyStruct {
                code: key_code.clone(),
                key_type: map_struct.key_type,
            },
        );
    }

    return keymap;
}

pub struct EventProcessor {
    pub key_map: HashMap<u16, KeyStruct>,
    pub output_device: Arc<Mutex<VirtualDevice>>,
    pub thread_map: HashMap<u16, Sender<()>>,
}

impl EventProcessor {
    pub fn new(config: UserConfig) -> io::Result<EventProcessor> {
        let key_set = create_str_map();
        let key_map = load_keymap(&default_code_map(), &config, &key_set);
        let output_device = VirtualDeviceBuilder::new()?
            .name("Virtual Orbweaver")
            .with_keys(&create_key_set(&key_set))?
            .build()?;
        Ok(EventProcessor {
            key_map,
            output_device: Arc::new(Mutex::new(output_device)),
            thread_map: HashMap::new(),
        })
    }

    pub fn process_event(&mut self, event: InputEvent) {
        match self.key_map.get(&event.code()) {
            Some(ks) => match ks.key_type {
                KeyType::Repeat => {
                    self.repeat_key(ks.code, event.value());
                }
                KeyType::Regular => {
                    self.regular_key(ks.code, event.value());
                }
            },
            None => {
                println!("Key is left unmapped. {}", event.code())
            }
        }
    }

    fn stop_thread(&mut self, code: u16) {
        // check that there is no existing thread
        match self.thread_map.get(&code) {
            Some(tx) => {
                let _ = tx.send(());
                self.thread_map.remove(&code.clone());
            }
            None => {}
        }
    }

    pub fn repeat_key(&mut self, code: u16, value: i32) {
        if value == 0 {
            self.stop_thread(code);
        } else {
            // If the thread is already running nothing needs done.
            match self.thread_map.get(&code) {
                Some(_) => {}
                None => {
                    self.start_repeat(code);
                }
            };
        }
    }

    pub fn start_repeat(&mut self, code: u16) {
        let (tx, rx) = mpsc::channel::<()>();
        let output_device = Arc::clone(&self.output_device);
        thread::spawn(move || {
            let mut down = true;
            let mut rng = rand::thread_rng();
            loop {
                match output_device.lock().unwrap().emit(&[InputEvent::new(
                    EventType::KEY,
                    code,
                    if down { 1 } else { 0 },
                )]) {
                    Ok(_) => {}
                    Err(_) => {
                        print!("Failed to send event.")
                    }
                };
                // Check if the user has released the triggering key
                match rx.try_recv() {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {}
                }
                down = !down;
                thread::sleep(Duration::from_millis(DELAY + rng.gen_range(0..DELTA)));
            }
            // Ensure the key up signal is sent when the thread stops
            if down {
                match output_device.lock().unwrap().emit(&[InputEvent::new(
                    EventType::KEY,
                    code,
                    0,
                )]) {
                    Ok(_) => {}
                    Err(_) => {
                        print!("Failed to send event.")
                    }
                };
            }
        });
        self.thread_map.insert(code, tx);
    }

    fn regular_key(&self, code: u16, value: i32) {
        match self.output_device.lock().unwrap().emit(&[InputEvent::new(
            EventType::KEY,
            code,
            value,
        )]) {
            Ok(_) => {}
            Err(_) => {
                print!("Failed to send event.")
            }
        };
    }
}
