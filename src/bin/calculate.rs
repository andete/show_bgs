extern crate reqwest;
extern crate show_bgs;
extern crate badlog;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate serde_json;

use std::collections::{BTreeSet,HashMap};
use std::fs::File;
// use std::io::Write;
use show_bgs::ebgsv4;
use show_bgs::data::*;

fn main() {
    badlog::minimal(Some("INFO"));
    info!("Calculating data");
    let system_names = show_bgs::read_config();
    info!("systems to handle: {:?}", system_names);
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let mut systems = HashMap::new();
    for system_name in &system_names {
        info!("Looking at {}...", system_name);
        let n = format!("{}/systems/{}.json", datadir, system_name);
        let f = File::open(&n).unwrap();
        let s:ebgsv4::EBGSSystemsV4 = serde_json::from_reader(&f).unwrap();
        let system:System = s.into();
        systems.insert(system_name.clone(), system);
    }
    info!("systems: {:?}", systems);

    let n = format!("{}/minor_factions.json", datadir);
    let f = File::open(&n).unwrap();
    let minor_faction_names:BTreeSet<String> = serde_json::from_reader(&f).unwrap();
    for minor_faction_name in minor_faction_names {
        info!("Looking at {}...", minor_faction_name);
        let n = format!("{}/factions/{}.json", datadir, minor_faction_name);
        let f = File::open(&n).unwrap();
        let factionv4:ebgsv4::EBGSFactionsV4 = serde_json::from_reader(&f).unwrap();
        let faction_template:Faction = (&factionv4).into();
        for history in factionv4.history {
            // could be that the system is not in our system list...
            if let Some(system) = systems.get_mut(&history.system) {
                let faction = system.factions.entry(faction_template.name.clone()).or_insert(faction_template.clone());
                let data:FactionData = history.into();
                faction.evolution.push(data);
            }
        }
    }
    for system in &mut systems.values_mut() {
        let colors = vec!["red", "blue", "green", "black", "cyan", "orange", "pink", "grey"];
        let mut it = colors.into_iter();
        for faction in &mut system.factions.values_mut() {
            faction.cleanup_evolution();
            faction.fill_in_state_days();
            faction.color = it.next().unwrap().into();
        }
    }

    let n = format!("{}/report.json", datadir);
    let f = File::create(&n).unwrap();
    let mut s2:Vec<System> = systems.into_iter().map(|(_,v)| v).collect();
    s2.sort_by(|a,b| a.name.cmp(&b.name));
    let dates = s2[0].factions.iter().next().unwrap().1.evolution.iter().map(|e| format!("{}", e.date.format("%d/%m"))).collect();
    let systems = Systems {
        systems: s2,
        dates:dates,
    };
    serde_json::to_writer_pretty(&f, &systems).unwrap();
}
