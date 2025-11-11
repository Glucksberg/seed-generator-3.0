use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const CONFIG_FILE: &str = "../gpuseed_config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub gpu_type: Option<String>,
}

impl Config {
    pub fn load_or_create() -> Option<Self> {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(content) => {
                serde_json::from_str(&content).ok()
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    eprintln!("Warning: Could not read config file: {}", e);
                }
                None
            }
        }
    }
    
    pub fn save_gpu_type(gpu_type: &str) {
        let config = Config {
            gpu_type: Some(gpu_type.to_string()),
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            if let Err(e) = fs::write(CONFIG_FILE, json) {
                eprintln!("Warning: Could not save config file: {}", e);
            }
        }
    }
}

