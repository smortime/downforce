use serde::{Serialize, Deserialize};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub workers_count: i8,
    pub worker_addresses: Vec<i32>,
    pub shard_size: u64,
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

        if !check_no_duplicate_ports(&self.worker_addresses) {
            println!("Duplicate ports provided");
            return false;
        }
        true
    }
}

fn check_no_duplicate_ports(ports: &[i32]) -> bool {
    let mut ports_map = HashMap::new();

    for port in ports {
        if ports_map.contains_key(port) {
            return false;
        }
        ports_map.insert(port, 1);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config() {

        let expected_conf: Config = Config {
            workers_count: 3,
            worker_addresses: vec![5001, 5002, 5003],
            shard_size: 800,
            input_dir: "tests/in".to_string(),
            output_dir: "tests/out".to_string(),
            output_file_count: 3
        };

        let path_str = "./tests/config/test_config.yaml";
        let path = Path::new(path_str);
        let conf = Config::read_config(&path).unwrap();
        assert_eq!(conf, expected_conf);
    }

    #[test]
    fn validate_bad_config() {
        let path_str = "./tests/config/test_config.yaml";
        let path = Path::new(path_str);
        let conf = Config::read_config(&path).unwrap();
        assert!(conf.validate(), "Config failed to validate!");
    }

    #[test]
    fn validate_good_config() {
        let path_str = "./tests/config/bad_config.yaml";
        let path = Path::new(path_str);
        let conf = Config::read_config(&path).unwrap();
        assert!(!conf.validate(), "Validate approved bad config!")
    }

    #[test]
    fn validate_no_duplicate_ports() {
        let ports = vec![5001, 5002, 5003];
        assert!(check_no_duplicate_ports(&ports), "Failed to verify no duplicate ports");
        let ports_dup = vec![5002, 5002, 5003];
        assert!(!check_no_duplicate_ports(&ports_dup), "Failed to verify duplicate ports");
    }
}