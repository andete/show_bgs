// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate show_bgs;
extern crate badlog;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let m = App::new("fetch")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("calculate")
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("filename of the systems.json file"))
        .get_matches();

    let filename = m.value_of("FILE").unwrap();
    badlog::minimal(Some("INFO"));
    let config = show_bgs::read_config(filename);
    show_bgs::calculate::calculate(&config);
}
