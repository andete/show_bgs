// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::Utc;
use reqwest;
use serde_json;

use std::collections::BTreeSet;
use std::fs::{create_dir_all, File};

use extdata::ebgsv4;
use data;
use Config;

pub fn fetch(config: &Config, n_days:i64) {
    info!("Fetching system info for last {} days", n_days);
    info!("and discovering minor factions");

    let system_names = config.systems();
    info!("systems: {:?}", system_names);

    let datadir = config.datadir();
    create_dir_all(&datadir).unwrap();

    let client = reqwest::Client::new();

    let mut minor_factions = BTreeSet::<String>::new();

    let now = (Utc::now().timestamp()+100)*1000;
    info!("now: {}", now);
    let then = now - ((n_days+1)*24*60*60*1000);

    // fetch ebgs data for systems
    let mut all_dates = BTreeSet::new();
    let mut system_dates = Vec::new();
    for system_name in &system_names {
        // fetch data
        let url = format!("{}systems?name={}&timemin={}&timemax={}", ebgsv4::URL, system_name, then, now);
        //info!("url: {}", url);
        let ebgs_system_data = client.get(&url).send().unwrap().text().unwrap();
        //info!("data: {}", ebgs_system_data);
        let mut systems_page:ebgsv4::SystemsPage = serde_json::from_str(&ebgs_system_data).unwrap();
        let system_data = systems_page.docs.remove(0);
        let update_date = system_data.updated_at.date();

        // store to file
        create_dir_all(format!("{}/systems/ebgsv4", datadir)).unwrap();
        let n = format!("{}/systems/ebgsv4/{}.json", datadir, system_name);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &system_data).unwrap();
        // collect dates
        all_dates.insert(update_date);
        system_dates.push((system_name, update_date));
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

        let system_data:data::System = system_data.into();
        let n = format!("{}/systems/{}.json", datadir, system_name);
        let mut f = File::create(&n).unwrap();
        serde_json::to_writer_pretty(&f, &system_data).unwrap();
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
