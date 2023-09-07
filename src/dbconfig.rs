use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    #[allow(dead_code)]
    pub(crate) url: String,
}

pub fn get_db_config() -> DbConfig {
    let mut file = File::open("/Users/john/.db/dbconfig.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    toml::from_str(&contents).expect("Failed to deserialize DbConfig.toml")
}
