use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub workers_count: i8,
    pub worker_info: ConfigSummary,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigSummary {
    pub address: i16,
    pub name: String,
}

impl Config {
    pub fn read_config(file_path: &Path) -> Result<Config, serde_yaml::Error> {
        let f = std::fs::File::open(file_path).expect("Failed to read config.");
        let conf: Config = serde_yaml::from_reader(f)?;
        Ok(conf)
    }
}