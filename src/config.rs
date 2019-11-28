use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub workers_count: i8,
}

impl Config {
    pub fn read_config(file_path: &Path) -> Config {
        let f = std::fs::File::open(file_path).expect("Failed to read config.");
        let conf: Config = serde_yaml::from_reader(f).unwrap();
        conf
    }
}