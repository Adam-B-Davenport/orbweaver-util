use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use evdev::Key;

#[derive(Copy, Clone)]
pub enum KeyType {
    Regular,
    Repeat,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            Self::Regular => "Regular",
            Self::Repeat => "Repeat",
        };
        write!(f, "{}", output)
    }
}

pub struct KeyStruct {
    pub code: u16,
    pub key_type: KeyType,
}

impl Clone for KeyStruct {
    fn clone(&self) -> Self {
        return KeyStruct {
            code: (self.code.clone()),
            key_type: (self.key_type),
        };
    }
}

pub struct ConfigStruct {
    pub key_str: String,
    pub key_type: KeyType,
}

impl Clone for ConfigStruct {
    fn clone(&self) -> Self {
        return ConfigStruct {
            key_str: (self.key_str.clone()),
            key_type: self.key_type,
        };
    }
}

pub type UserConfig = HashMap<u16, ConfigStruct>;

// Hashmap to translate a string to a Key
pub fn create_str_map() -> HashMap<String, u16> {
    return HashMap::from(
        [
            ("1", Key::KEY_1),
            ("2", Key::KEY_2),
            ("3", Key::KEY_3),
            ("4", Key::KEY_4),
            ("5", Key::KEY_5),
            ("6", Key::KEY_6),
            ("7", Key::KEY_7),
            ("8", Key::KEY_8),
            ("9", Key::KEY_9),
            ("0", Key::KEY_0),
            ("Q", Key::KEY_Q),
            ("W", Key::KEY_W),
            ("E", Key::KEY_E),
            ("R", Key::KEY_R),
            ("T", Key::KEY_T),
            ("Y", Key::KEY_Y),
            ("U", Key::KEY_U),
            ("I", Key::KEY_I),
            ("O", Key::KEY_O),
            ("P", Key::KEY_P),
            ("A", Key::KEY_A),
            ("S", Key::KEY_S),
            ("D", Key::KEY_D),
            ("F", Key::KEY_F),
            ("G", Key::KEY_G),
            ("H", Key::KEY_H),
            ("J", Key::KEY_J),
            ("K", Key::KEY_K),
            ("L", Key::KEY_L),
            ("SEMI", Key::KEY_SEMICOLON),
            ("Z", Key::KEY_Z),
            ("X", Key::KEY_X),
            ("C", Key::KEY_C),
            ("V", Key::KEY_V),
            ("B", Key::KEY_B),
            ("M", Key::KEY_M),
            ("N", Key::KEY_N),
            ("COMMA", Key::KEY_COMMA),
            ("DOT", Key::KEY_DOT),
            ("BSLSH", Key::KEY_SLASH),
            ("F1", Key::KEY_F1),
            ("F2", Key::KEY_F2),
            ("F3", Key::KEY_F3),
            ("F4", Key::KEY_F4),
            ("F5", Key::KEY_F5),
            ("F6", Key::KEY_F6),
            ("F7", Key::KEY_F7),
            ("F8", Key::KEY_F8),
            ("F9", Key::KEY_F9),
            ("F10", Key::KEY_F10),
            ("F11", Key::KEY_F11),
            ("F12", Key::KEY_F12),
            ("F13", Key::KEY_F13),
            ("F14", Key::KEY_F14),
            ("F15", Key::KEY_F15),
            ("F16", Key::KEY_F16),
            ("F17", Key::KEY_F17),
            ("F18", Key::KEY_F18),
            ("F19", Key::KEY_F19),
            ("F20", Key::KEY_F20),
            ("F21", Key::KEY_F21),
            ("F22", Key::KEY_F22),
            ("F23", Key::KEY_F23),
            ("F24", Key::KEY_F24),
            ("SPACE", Key::KEY_SPACE),
            ("CTRL", Key::KEY_LEFTCTRL),
            ("SHIFT", Key::KEY_LEFTSHIFT),
            ("TAB", Key::KEY_TAB),
            ("ESC", Key::KEY_ESC),
        ]
        .map(|(key_str, key)| (key_str.to_string(), key.code())),
    );
}

pub fn load_config(path: String) -> HashMap<u16, ConfigStruct> {
    match File::open(path) {
        Ok(file) => {
            let mut config: HashMap<u16, ConfigStruct> = HashMap::new();
            let mut line_number = 1;
            for line in BufReader::new(file).lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => {
                        // It is unlikely that a line will be unable to be read.
                        print!("Failed to read line {} of config.", line_number);
                        exit(-1);
                    }
                };
                let words: Vec<&str> = line.split(" ").collect();
                if words.len() < 2 || words.len() > 3 {
                    print!("Error on line {} of config.", line_number);
                    exit(-1);
                }
                let id: u16 = match words[0].parse() {
                    Ok(id) => id,
                    Err(_) => {
                        print!(
                            "Error on line {} of config, unable to parse key code.",
                            line_number
                        );
                        exit(-1);
                    }
                };
                let key_str = words[1].to_string();
                let key_type = if words.len() == 2 {
                    KeyType::Regular
                } else {
                    match words[2].to_lowercase().as_str() {
                        "regular" => KeyType::Regular,
                        "repeat" => KeyType::Repeat,
                        _ => KeyType::Regular,
                    }
                };
                config.insert(id, ConfigStruct { key_str, key_type });
                line_number += 1
            }
            return config;
        }
        Err(_) => {
            print!("Failed to load config file.");
            exit(-1);
        }
    };
}
