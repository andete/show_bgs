// (c) 2018 Joost Yervante Damad <joost@damad.be>

use reqwest;

use serde_json;

use chrono::Utc;
use std::time::Duration;

use std::collections::BTreeSet;
use std::fs::File;

use eddbv3;
use ebgsv4;
use Config;

pub fn fetch_fact(config:&Config, n_days:i64) {

    info!("Fetching Minor Faction data for last {} days", n_days);
    let datadir = config.datadir();
    let client = reqwest::Client::builder().gzip(true).timeout(Duration::from_secs(20)).build().unwrap();
    let n = format!("{}/minor_factions.json", datadir);
    let f = File::open(&n).unwrap();
    let minor_factions:BTreeSet<String> = serde_json::from_reader(&f).unwrap();

    let now = Utc::now().timestamp()*1000;
    let then = now - ((n_days+1)*24*60*60*1000);
    info!("now: {}", now);
 
    for faction in &minor_factions {
        info!("Faction: {}", faction);
        // first fetch eddb data
        let url = format!("{}factions?name={}", eddbv3::URL, faction);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let json:eddbv3::FactionPage = serde_json::from_str(&res).unwrap();
        let n = format!("{}/factions/eddb/{}.json", datadir, faction);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &json.docs[0]).unwrap();

        // then fetch ebgs data
        let url = format!("{}factions?name={}&timemin={}&timemax={}", ebgsv4::URL, faction, then, now);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let json:ebgsv4::EBGSFactionsPageV4 = serde_json::from_str(&res).unwrap();
        let n = format!("{}/factions/{}.json", datadir, faction);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &json.docs[0]).unwrap();
    }
}
