// (c) 2018 Joost Yervante Damad <joost@damad.be>

use reqwest;

use serde_json;

use chrono::Utc;
use std::time::Duration;

use std::collections::BTreeSet;
use std::fs::{create_dir_all, File};

use extdata::eddbv3;
use extdata::ebgsv4;
use data;
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
 
    for faction_name in &minor_factions {
        info!("Faction: {}", faction_name);
        // first fetch eddb data
        let url = format!("{}factions?name={}", eddbv3::URL, faction_name);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let mut faction_page:eddbv3::FactionPage = serde_json::from_str(&res).unwrap();
        let eddb_faction = faction_page.docs.remove(0);
        create_dir_all(format!("{}/factions/eddbv3", datadir)).unwrap();
        let n = format!("{}/factions/eddbv3/{}.json", datadir, faction_name);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &eddb_faction).unwrap();


        // then fetch ebgs data
        let url = format!("{}factions?name={}&timemin={}&timemax={}", ebgsv4::URL, faction_name, then, now);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let mut faction_page:ebgsv4::FactionsPage = serde_json::from_str(&res).unwrap();
        let ebgs_faction = faction_page.docs.remove(0);
        create_dir_all(format!("{}/factions/ebgsv4", datadir)).unwrap();
        let n = format!("{}/factions/ebgsv4/{}.json", datadir, faction_name);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &ebgs_faction).unwrap();

        // combine to our data
        let faction:data::Faction = data::Faction::from(eddb_faction, ebgs_faction);
        let n = format!("{}/factions/{}.json", datadir, faction_name);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &faction).unwrap();
    }
}
