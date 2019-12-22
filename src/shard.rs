use crate::config::Config;
use std::fs::{read_dir, metadata, File};
use std::io::{Seek, SeekFrom, BufReader, BufRead};
use std::fmt::Error;

// Represents the shard of work to be processed during map phase
pub struct Shard {
    pub id: i16,
    pub file_path: String,
    pub size: u64,
}

pub fn get_shards(conf: &Config) -> Result<Vec<Shard>, Error> {
    let mut shards: Vec<Shard> = Vec::new();
    let mut index = 0;
    let files = read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    for file_path in files {
        let file = File::open(&file_path.unwrap().path()).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {

        }
    }
    Ok(shards)
}

pub fn get_total_input_size(conf: &Config) -> u64 {
    let files = read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    files.fold(0, |sum, file|
        sum + metadata(file
            .expect("Invalid File in input directory")
            .path()).unwrap().len()
    )
}

pub fn get_shard_count(conf: &Config) -> i16 {
    let total_inputs_size = get_total_input_size(&conf);
    (total_inputs_size / conf.shard_size) as i16
}