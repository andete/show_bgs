// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::Utc;
use reqwest;
use std::collections::BTreeSet;
use std::fs::{create_dir_all, File};

use ebgsv4;
use serde_json;
use Config;

pub fn fetch(config: &Config, n:i32) {
    info!("Fetching system info for last {} days", n);
    info!("and discovering minor factions");

    let systems = config.systems();
    info!("systems: {:?}", systems);

    let datadir = config.datadir();
    create_dir_all(&datadir).unwrap();

    let client = reqwest::Client::new();

    let mut minor_factions = BTreeSet::<String>::new();

    let now = (Utc::now().timestamp()+100)*1000;
    info!("now: {}", now);

    // fetch ebgs data for systems
    let mut all_dates = BTreeSet::new();
    let mut system_dates = Vec::new();
    for system in &systems {
        // fetch data
        let url = format!("{}systems?name={}&timemax={}", ebgsv4::URL, system, now);
        let ebgs_system_data = client.get(&url).send().unwrap().text().unwrap();
        let json:ebgsv4::EBGSSystemsPageV4 = serde_json::from_str(&ebgs_system_data).unwrap();
        let system_data = &json.docs[0];
        let update_date = system_data.updated_at.date();

        // store to file
        let n = format!("{}/systems/{}.json", datadir, system);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &system_data).unwrap();
        // collect dates
        all_dates.insert(update_date);
        system_dates.push((system, update_date));
        // present factions in system
        for faction in &system_data.factions {
            minor_factions.insert(faction.name.clone());
        }
        // historical factions in system
        for history in &system_data.history {
            for faction in &history.factions {
                minor_factions.insert(faction.name.clone());
            }
        }
    }
    // we assume the latest day is bgs day
    let day = all_dates.iter().max().unwrap();
    info!("Current BGS day: {}", day);
    // report systems that are older
    for &(system,date) in &system_dates {
        if *day != date {
            warn!("System is not up to date: {}", system);
        }
    }

    // write list of minor factions to file
    info!("Minor factions involved: {:?}", minor_factions);  
    let n = format!("{}/minor_factions.json", datadir);
    let f = File::create(&n).unwrap();
    serde_json::to_writer_pretty(&f, &minor_factions).unwrap();
}
