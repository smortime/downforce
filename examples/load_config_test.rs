use std::{path::Path};

extern crate downforce;
use downforce::config::Config;

fn main() {
    let path_str = "./examples/conf.yaml";
    let path = Path::new(path_str);
    // let conf_file = fs::File::open(path).expect("failed to open config");
    println!("Testing!");
    let conf = Config::read_config(&path).unwrap();
    println!("Worker Count: {} Worker Address: {} Worker Name: {}", conf.workers_count,
             conf.worker_info.address, conf.worker_info.name);
}