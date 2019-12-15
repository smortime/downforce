use crate::config::Config;
use std::fs;

pub struct Shard {

}

pub fn get_shards(conf: Config, shards: &mut Vec<Shard>) {

}

pub fn get_shard_count(conf: &Config) -> i16 {
    let mut total_file_size: u64 = 0;
    let files = fs::read_dir(conf.input_dir.to_string()).unwrap();

    for file in files {
        let metadata = fs::metadata(file.unwrap().path()).unwrap();
        total_file_size += metadata.len();
    }
    (total_file_size / conf.shard_size as u64) as i16
}