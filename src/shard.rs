use crate::config::Config;
use std::fs;
use std::path::Path;

// Represents the shard of work to be processed
pub struct Shard {
    pub id: i16,
    pub pieces: Vec<Piece>,
}

// Is the chunk of file that makes up a shard
pub struct Piece {
    pub file_path: String,
    pub start: i32,
    pub end: i32,
}

pub fn get_shards(conf: &Config, shards: &mut Vec<Shard>) {

}

pub fn get_shard_count(conf: &Config) -> i16 {
    let mut total_file_size: u64 = 0;
    let files = fs::read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    for file in files {
        let metadata = fs::metadata(file
            .expect("Invalid File in input directory")
            .path()).unwrap();
        total_file_size += metadata.len();
    }
    (total_file_size / conf.shard_size as u64) as i16
}