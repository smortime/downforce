use crate::config::Config;
use std::fs::{read_dir, metadata, File};
use std::io::{BufReader, BufRead, BufWriter, Write};

// Represents the shard of work to be processed during map phase
pub struct Shard {
    pub id: i16,
    pub file_path: String,
    pub size: u64,
}

pub fn get_shards(conf: &Config) -> Vec<Shard> {
    let mut shards: Vec<Shard> = Vec::new();
    let mut index = 0;
    let mut shard_file_size = 0;
    let mut shard_file_name = format!("{}/temp_shard_{}.txt", conf.output_dir, index);
    let f = File::create(&shard_file_name).expect("Unable to create file");
    let mut writer = BufWriter::new(f);

    shards.push(Shard{
        id: index,
        file_path: shard_file_name,
        size: 0,
    });

    let files = read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    for file_path in files {
        let file = File::open(&file_path.unwrap().path()).unwrap();
        let reader = BufReader::new(file);


        for line in reader.lines() {
            let line_string = line.unwrap();

            if shard_file_size + line_string.len() > conf.shard_size as usize {
                index += 1;
                shard_file_name = format!("{}/temp_shard_{}.txt", conf.output_dir, index);
                let temp = File::create(&shard_file_name).expect("Unable to create file");
                writer = BufWriter::new(temp);
                shard_file_size = line_string.len();

                shards.push(Shard{
                    id: index,
                    file_path: shard_file_name,
                    size: 0,
                });

            } else {
                shard_file_size += line_string.len();
            }
            writer.write_all(format!("{}\n", line_string).as_bytes()).expect("Unable to write to file");
        }
    }
    shards
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
    (total_inputs_size / conf.shard_size + 1) as i16
}