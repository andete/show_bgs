extern crate reqwest;
extern crate show_bgs;
extern crate badlog;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate serde_json;

use chrono::Utc;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use show_bgs::ebgsv4;

fn main() {
    badlog::minimal(Some("INFO"));
    info!("Calculating data");
    let systems = show_bgs::read_config();
    info!("systems: {:?}", systems);
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    for system in &systems {
        let n = format!("{}/systems/{}.json", datadir, system);
        let f = File::open(&n).unwrap();
        let s:ebgsv4::EBGSSystemsV4 = serde_json::from_reader(&f).unwrap();
    }
}
