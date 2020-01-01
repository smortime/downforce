use crate::config::Config;
use std::fs::{read_dir, metadata, File};
use std::io::{BufReader, BufRead, BufWriter, Write};

// Represents the shard of work to be processed during map phase
struct Shard {
    id: i16,
    file_path: String,
    size: u64,
}

fn get_shards(conf: &Config) -> Vec<Shard> {
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

fn get_total_input_size(conf: &Config) -> u64 {
    let files = read_dir(conf.input_dir.to_string())
        .expect("Unable to read input directory");

    files.fold(0, |sum, file|
        sum + metadata(file
            .expect("Invalid File in input directory")
            .path()).unwrap().len()
    )
}

fn get_shard_count(conf: &Config) -> i16 {
    let total_inputs_size = get_total_input_size(&conf);
    (total_inputs_size / conf.shard_size + 1) as i16
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
pub mod test_utils {

    use super::*;
    use std::fs;
    use glob::glob;

    pub fn get_test_config() -> Config {
        Config {
            workers_count: 3,
            worker_addresses: vec![5001, 5002, 5003],
            shard_size: 800,
            input_dir: "tests/in".to_string(),
            output_dir: "tests/out".to_string(),
            output_file_count: 3
        }
    }

    pub fn delete_temp_files(output_dir: &String) {
        let pattern = format!("{}/temp_*.txt", output_dir);

        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => fs::remove_file(path).expect("Failed to delete file"),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shard::test_utils::{get_test_config, delete_temp_files};

    #[test]
    fn validate_get_input_total_size() {
        let conf: Config = get_test_config();
        let total_size = get_total_input_size(&conf);
        assert_eq!(total_size, 2400);
    }

    #[test]
    fn validate_get_shard_count() {
        let conf = get_test_config();
        let shard_count = get_shard_count(&conf);
        assert_eq!(shard_count, 4);
    }

    #[test]
    fn validate_get_shards() {
        let conf = get_test_config();
        let shard_count = get_shard_count(&conf);
        let shards = get_shards(&conf);
        assert_eq!(shards.len() as i16, shard_count);
        delete_temp_files(&conf.output_dir);
    }
}