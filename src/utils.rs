use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec;

pub struct KeyConfig {
    id: u16,
    key: String,
    key_type: String,
}

pub fn load_config(path: String) -> Vec<KeyConfig> {
    match File::open(path) {
        Ok(file) => {
            let mut config: Vec<KeyConfig> = Vec::new();
            let mut line_number = 1;
            for line in BufReader::new(file).lines() {
                let line = line.unwrap();
                let words: Vec<&str> = line.split(" ").collect();
                if words.len() != 3 {
                    print!("Error on line {} of config.", line_number);
                    exit(-1);
                }
                let id: u16 = words[0].parse().unwrap();
                let key = words[1].to_string();
                let key_type = words[2].to_string();
                config.push(KeyConfig { id, key, key_type });
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
