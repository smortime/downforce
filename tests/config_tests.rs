use downforce::config::Config;
use std::path::Path;

#[test]
fn load_config() {

    let expected_conf: Config = Config {
        workers_count: 3,
        worker_addresses: vec!["5001".to_string(), "5002".to_string(), "5003".to_string()],
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