// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate chrono;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tera;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub short_name: String,
    pub report_name: String,
    pub main_systems: Vec<String>,
    pub other_systems: Vec<String>,
}

impl Config {
    pub fn systems(&self) -> Vec<String> {
        let mut v = self.main_systems.clone();
        for s in &self.other_systems { v.push(s.clone()) }
        v
    }
    pub fn datadir(&self) -> String {
        format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), self.short_name)
    }
}

pub fn read_config(filename: &str) -> Config {
    use std::fs::File;

    let n = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), filename);
    debug!("config file: {}", n);
    let f = File::open(&n).unwrap();
    let mut c: Config = serde_json::from_reader(f).unwrap();
    c.main_systems.sort();
    c.other_systems.sort();
    c
}

pub mod calculate;
pub mod data;
pub mod ebgsv4;
pub mod eddbv3;
pub mod fetch;
pub mod fetch_fact;
pub mod webpage;

