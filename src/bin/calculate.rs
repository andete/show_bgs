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
    let mut faction_colors:HashMap<String,String> = HashMap::new();
    faction_colors.insert("The Order of Mobius".into(), "black".into());
    
    let mut dates = BTreeSet::new();
    for system in systems.values() {
        for faction in system.factions.values() {
            for e in &faction.evolution {
                dates.insert(e.date.date());
            }
        }
    }
    let dates_vec:Vec<Date<Utc>> = dates.iter().cloned().collect();

    for system in &mut systems.values_mut() {
        for faction in &mut system.factions.values_mut() {
            faction.cleanup_evolution(&dates_vec);
            faction.fill_in_state_days();
            faction.fill_in_evolution10();
        }
        let mut colors:BTreeSet<String> = vec!["red", "blue", "green", "cyan", "orange", "pink", "grey", "black"].into_iter().map(|x| x.into()).collect();
        // first fill in using registered colors:
        for faction in &mut system.factions.values_mut() {
            if let Some(color) = faction_colors.get(&faction.name) {
                faction.color = color.clone();
                colors.remove(color);
            }
        }
        let mut it = colors.into_iter();
        for faction in &mut system.factions.values_mut() {
            if faction.color.is_empty() {
                faction.color = it.next().unwrap().into();
                faction_colors.insert(faction.name.clone(), faction.color.clone());
            }
        }
    }

    let n = format!("{}/report.json", datadir);
    let f = File::create(&n).unwrap();
    let mut s2:Vec<System> = systems.into_iter().map(|(_,v)| v).collect();
    s2.sort_by(|a,b| a.name.cmp(&b.name));

    let dates:Vec<String> = dates.iter().map(|e| format!("{}", e.format("%d/%m"))).collect();
    let dates2 = dates.clone();
    let dates10 = dates.as_slice();
    let dates10 = dates10.windows(10).last().unwrap().to_vec();
    let systems = Systems {
        systems: s2,
        dates:dates2,
        dates10:dates10,
    };
    serde_json::to_writer_pretty(&f, &systems).unwrap();
}
