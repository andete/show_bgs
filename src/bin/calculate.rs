extern crate reqwest;
extern crate show_bgs;
extern crate badlog;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate serde_json;

use chrono::{Date,Utc};

use std::collections::{BTreeSet,HashMap};
use std::fs::File;
use show_bgs::ebgsv4;
use show_bgs::eddbv3;
use show_bgs::data::*;

fn main() {
    badlog::minimal(Some("INFO"));
    show_bgs::calculate::calculate();
}
