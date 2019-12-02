use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub workers_count: i8,
    pub worker_addresses: Vec<String>,
    pub shard_size: i32,
    pub input_dir: String,
    pub output_dir: String,
    pub output_file_count: i8,
}

impl Config {
    pub fn read_config(file_path: &Path) -> Result<Config, serde_yaml::Error> {
        let f = std::fs::File::open(file_path).expect("Failed to read config.");
        let conf: Config = serde_yaml::from_reader(f)?;
        Ok(conf)
    }

    pub fn validate(&self) -> bool {
        if self.worker_addresses.len() as i8 != self.workers_count {
            println!("Number of Addresses does not match Workers Count");
            return false;
        }

        if !Path::new(&self.output_dir).exists() {
            println!("Output directory does not exist");
            return false;
        }

        if !Path::new(&self.input_dir).exists() {
            println!("Input Directory does not exist");
            return false;
        }

        true
    }
}