use downforce::config::Config;
use downforce::shard::{get_shard_count, Shard, get_shards, get_total_inputs_size};

fn get_test_config() -> Config {
    Config {
        workers_count: 3,
        worker_addresses: vec!["5001".to_string(), "5002".to_string(), "5003".to_string()],
        shard_size: 800,
        input_dir: "tests/in".to_string(),
        output_dir: "tests/out".to_string(),
        output_file_count: 3
    }
}

#[test]
fn validate_get_shard_count() {
    let conf = get_test_config();
    let shard_count = get_shard_count(&conf);
    assert_eq!(shard_count, 3);
}

#[test]
fn validate_get_input_total_size() {
    let conf: Config = get_test_config();
    let total_size = get_total_inputs_size(&conf);
    assert_eq!(total_size, 2400);
}

//#[test]
//fn validate_get_shards() {
//    let conf = get_test_config();
//    let mut shards: Vec<Shard> = Vec::new();
//    let shard_count = get_shard_count(&conf);
//    get_shards(&conf, &mut shards);
//    assert_eq!(shards.len() as i16, shard_count);
//}