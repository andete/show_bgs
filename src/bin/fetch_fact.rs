// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate show_bgs;
extern crate badlog;


fn main() {
    badlog::minimal(Some("INFO"));
    show_bgs::fetch_fact::fetch_fact(32);
}
