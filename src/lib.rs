extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;

use std::fs::File;

pub const BASE_URL:&'static str = "https://elitebgs.kodeblox.com/api/ebgs/v4/";

#[derive(Debug,Deserialize)]
struct Config {
    pub systems:Vec<String>,
}

pub fn read_config() -> Vec<String> {
    let n = format!("{}/systems.json", env!("CARGO_MANIFEST_DIR"));
    debug!("config file: {}", n);
    let f = File::open(&n).unwrap();
    let c:Config = serde_json::from_reader(f).unwrap();
    c.systems
}

pub mod ebgsv4;
pub mod data;

