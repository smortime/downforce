use crate::config::Config;
use std::fs;

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

}

pub fn get_total_inputs_size(conf: &Config) -> u64 {
    let files = fs::read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    files.fold(0, |sum, file|
        sum + fs::metadata(file
            .expect("Invalid File in input directory")
            .path()).unwrap().len()
    )
}

pub fn get_shard_count(conf: &Config) -> i16 {
    let total_inputs_size = get_total_inputs_size(&conf);
    (total_inputs_size / conf.shard_size) as i16
}