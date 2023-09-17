#![allow(dead_code)]

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    pub db_connection_string: String,

    pub data_folder: String,
}

pub fn get_config() -> Config {
    let config_file = shellexpand::tilde("~/.portfolio_config/config.toml");
    let mut file = File::open(config_file.to_string()).unwrap_or_else(|_| panic!("Failed to open config file: '{config_file}'"));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    toml::from_str(&contents).unwrap_or_else(|_| panic!("Failed to deserialize config file: '{config_file}'"))
}

pub fn get_file_path(config: &Config, filename: &str) -> Result<String, String> {
    match test_path(filename) {
        Ok(filename) => Ok(filename),
        Err(_) => match Path::new(&config.data_folder).join(filename).to_str() {
            None => panic!("path.to_str() error"),
            Some(filepath) => match test_path(filepath) {
                Ok(filename) => Ok(filename),
                Err(e) => Err(e),
            },
        },
    }
}

pub fn make_file_path(config: &Config, filename: &str) -> Result<String, String> {
    match test_path(filename) {
        Ok(filename) => Ok(filename),
        Err(_) => Ok(Path::new(&config.data_folder).join(filename).to_str().unwrap().to_string())
    }
}

pub fn test_path(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    if path.exists() {
        if path.is_file() {
            match path.to_str() {
                None => panic!("path is not valid utc-8"),
                Some(s) => return Ok(s.to_string()),
            }
        } else {
            return Err(format!("path: '{filename}' exits but is not a file"));
        }
    }
    Err(format!("path: '{filename}' does not exist"))
}
