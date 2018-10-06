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
    let n = 7;
    badlog::minimal(Some("INFO"));
    info!("Fetching system info for last {} days", n);
    info!("and discovering minor factions");
    let systems = show_bgs::read_config().systems();
    info!("systems: {:?}", systems);
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let client = reqwest::Client::new();
    let mut minor_factions = BTreeSet::<String>::new();

    let now = (Utc::now().timestamp()+100)*1000;
    info!("now: {}", now);

    let mut dates = BTreeSet::new();
    let mut system_dates = Vec::new();
    for system in &systems {
        let url = format!("{}systems?name={}&timemax={}", show_bgs::EBGSV4_URL, system, now);
        let res = client.get(&url).send().unwrap().text().unwrap();
        //info!("{:?}", res);
        let n = format!("{}/systems/{}.json", datadir, system);
        let mut f = File::create(&n).unwrap();
        let json:show_bgs::ebgsv4::EBGSSystemsPageV4 = serde_json::from_str(&res).unwrap();
        println!("json: {:?}", json);
        serde_json::to_writer_pretty(&f, &json.docs[0]).unwrap();
        dates.insert(json.docs[0].updated_at.date());
        system_dates.push((system, json.docs[0].updated_at.date()));
        for faction in &json.docs[0].factions {
            minor_factions.insert(faction.name.clone());
        }
        for history in &json.docs[0].history {
            for faction in &history.factions {
                minor_factions.insert(faction.name.clone());
            }
        }
    }

    let day = dates.iter().max().unwrap();
    info!("Current BGS day: {}", day);
    for &(system,date) in &system_dates {
        if *day != date {
            warn!("System is not up to date: {}", system);
        }
    }
    // TODO report systems with missing data for today
    
    info!("Minor factions involved: {:?}", minor_factions);  
    let n = format!("{}/minor_factions.json", datadir);
    let f = File::create(&n).unwrap();
    serde_json::to_writer_pretty(&f, &minor_factions).unwrap();
}
