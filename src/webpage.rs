// (c) 2018 Joost Yervante Damad <joost@damad.be>

use serde_json;
use std::fs::{create_dir_all, File, read_dir, copy};
use std::path::PathBuf;
use std::io::Write;
use data::*;
use Config;

pub fn webpage(config:&Config) {
    info!("Generating webpage.");
    let datadir = config.datadir();
    let n = format!("{}/report.json", datadir);
    let f = File::open(&n).unwrap();
    let systems:Systems = serde_json::from_reader(&f).unwrap();
    let templates_fn = format!("{}/../../template/*.tera", datadir);
    let templates = compile_templates!(&templates_fn);
    let outdir = format!("{}/out", datadir);
    let outpath:PathBuf = outdir.clone().into();
    create_dir_all(outdir).unwrap();
    let page = templates.render("index.tera", &systems).unwrap();
    let n = format!("{}/out/index.html", datadir);
    let mut f = File::create(&n).unwrap();
    f.write_all(page.as_bytes()).unwrap();
    let dir = format!("{}/../../template/data", datadir);
    for entry in read_dir(dir).unwrap() {
        let file = entry.unwrap().path();
        let filename:String = file.file_name().unwrap().to_str().unwrap().into();
        let mut op = outpath.clone();
        op.push(filename);
        copy(file, op).unwrap();
    }
}
