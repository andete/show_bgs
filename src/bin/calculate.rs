// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate show_bgs;
extern crate badlog;
extern crate clap;

fn main() {
    use clap::{Arg, App};
    let m = App::new("calculate")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("calculate")
        .arg(Arg::with_name("yesterday")
            .short("y")
            .long("yesterday")
            .help("calculate for BGS day starting yesterday")
        )
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("filename of the systems.json file"))
        .get_matches();

    let filename = m.value_of("FILE").unwrap();
    let yesterday = m.is_present("yesterday");
    badlog::minimal(Some("INFO"));
    let config = show_bgs::read_config(filename);
    show_bgs::calculate::calculate(&config, yesterday);
}
