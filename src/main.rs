extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

/*

For each system we're interested in, request the faction list for the last 7 days.

For each unique faction request the last 7 days data.


*/

fn main() {
    let mut f = File::open("../faction_oom.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let r:ebgsv4::EBGSPageV4<ebgsv4::EBGSFactionsV4> = serde_json::from_str(&contents).unwrap();
    let r = &r.docs[0];
    for h in &r.history {
        if h.system == "Maausk" {
            println!("{} {}: {}: {} {}", h._id, h.system, h.updated_at, h.influence, h.state);
        }
    }
}

pub mod ebgsv4;
