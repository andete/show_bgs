extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate reqwest;

use std::fs::File;

#[derive(Debug,Deserialize)]
pub struct Config {
    pub report_name:String,
    pub main_systems:Vec<String>,
    pub other_systems:Vec<String>
}

impl Config {
    pub fn systems(&self) -> Vec<String> {
        let mut v = self.main_systems.clone();
        for s in &self.other_systems { v.push(s.clone()) }
        v
    }
}

pub fn read_config() -> Config {
    let n = format!("{}/systems.json", env!("CARGO_MANIFEST_DIR"));
    debug!("config file: {}", n);
    let f = File::open(&n).unwrap();
    let mut c:Config = serde_json::from_reader(f).unwrap();
    c.main_systems.sort();
    c.other_systems.sort();
    c
}

pub mod ebgsv4;
pub mod eddbv3;
pub mod data;
pub mod fetch;
pub mod fetch_fact;

