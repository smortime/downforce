use crate::config::Config;
use std::fs;
use std::path::Path;

// Represents the shard of work to be processed during map phase
pub struct Shard {
    pub id: i16,
    pub pieces: Vec<Piece>,
}

// Is a chunk of file that makes up a shard
pub struct Piece {
    pub file_path: String,
    pub start: i32,
    pub end: i32,
}

pub fn get_shards(conf: &Config, shards: &mut Vec<Shard>) {
    // loop through number of allocated shards
    // loop through files
    // create pieces based off size and add to shard

}

pub fn get_shard_count(conf: &Config) -> i16 {
    let files = fs::read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    let total_file_size = files.fold(0, |sum, file|
        sum + fs::metadata(file
            .expect("Invalid File in input directory")
            .path()).unwrap().len()
    );

    (total_file_size / conf.shard_size as u64) as i16
}