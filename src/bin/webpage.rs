extern crate reqwest;
extern crate show_bgs;
extern crate badlog;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate tera;

use std::collections::{BTreeSet,HashMap};
use std::fs::File;
use std::io::Write;
use show_bgs::ebgsv4;
use show_bgs::data::*;

fn main() {
    badlog::minimal(Some("INFO"));
    info!("Generating webpage.");
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let n = format!("{}/report.json", datadir);
    let f = File::open(&n).unwrap();
    let systems:HashMap<String,System> = serde_json::from_reader(&f).unwrap();

    let templates_fn = format!("{}/../template/*.tera", datadir);
    let templates = compile_templates!(&templates_fn);

    let page = templates.render("index.tera", &systems).unwrap();
    info!("page: {}", page);
    let n = format!("{}/../out/index.html", datadir);
    let mut f = File::create(&n).unwrap();
    f.write_all(page.as_bytes()).unwrap();
}
