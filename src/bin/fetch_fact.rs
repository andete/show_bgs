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

fn main() {
    badlog::minimal(Some("INFO"));
    info!("Fetching Minor Faction data for last 32 days");
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let client = reqwest::Client::new();
    let n = format!("{}/minor_factions.json", datadir);
    let f = File::open(&n).unwrap();
    let minor_factions:BTreeSet<String> = serde_json::from_reader(&f).unwrap();

    let now = Utc::now().timestamp()*1000;
    let then = now - (32*24*60*60*1000);
    info!("now: {}", now);
 
    for faction in &minor_factions {
        info!("Faction: {}", faction);
        let url = format!("{}factions?name={}&timemin={}&timemax={}", show_bgs::BASE_URL, faction, then, now);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let json:show_bgs::ebgsv4::EBGSFactionsPageV4 = serde_json::from_str(&res).unwrap();
        let n = format!("{}/factions/{}.json", datadir, faction);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &json.docs[0]).unwrap();
    }
}
