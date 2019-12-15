use downforce::config::Config;
use downforce::shard::get_shard_count;

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
