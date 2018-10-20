// (c) 2018 Joost Yervante Damad <joost@damad.be>


extern crate show_bgs;
extern crate badlog;

fn main() {
    badlog::minimal(Some("INFO"));
    let n = 7;
    show_bgs::fetch::fetch(n);
}
