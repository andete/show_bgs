// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate chrono;
extern crate serde_json;

use std::fs::File;
use std::io::Write;
use data::*;
use Config;

pub fn webpage(config:&Config) {
    info!("Generating webpage.");
    let datadir = config.datadir();
    let n = format!("{}/report.json", datadir);
    let f = File::open(&n).unwrap();
    let systems:Systems = serde_json::from_reader(&f).unwrap();

    let templates_fn = format!("{}/../template/*.tera", datadir);
    let templates = compile_templates!(&templates_fn);

    let page = templates.render("index.tera", &systems).unwrap();
    let n = format!("{}/../out/index.html", datadir);
    let mut f = File::create(&n).unwrap();
    f.write_all(page.as_bytes()).unwrap();
}
