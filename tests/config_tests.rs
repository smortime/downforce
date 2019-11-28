use downforce::config::Config;
use std::path::Path;

#[test]
fn load_config() {
    let path_str = "./tests/test_config.yaml";
    let path = Path::new(path_str);
    let conf = Config::read_config(&path).unwrap();
    assert_eq!(conf.workers_count, 8);
    assert_eq!(conf.worker_info.address, 4321);
    assert_eq!(conf.worker_info.name, "testing");
}

