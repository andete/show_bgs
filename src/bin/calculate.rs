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
use show_bgs::eddbv3;
use show_bgs::data::*;

fn main() {
    badlog::minimal(Some("INFO"));
    info!("Calculating data");
    let warnings = vec![];
    let mut system_warnings = HashMap::new();
    let system_names = show_bgs::read_config().systems();
    info!("systems to handle: {:?}", system_names);
    let datadir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let mut systems = HashMap::new();
    let mut dates = BTreeSet::new();
    let mut system_dates = Vec::new();
    for system_name in &system_names {
        info!("Looking at {}...", system_name);
        let n = format!("{}/systems/{}.json", datadir, system_name);
        let f = File::open(&n).unwrap();
        let s:ebgsv4::EBGSSystemsV4 = serde_json::from_reader(&f).unwrap();
        if let Some(d) = s.bgs_day() {
            dates.insert(d);
            system_dates.push((system_name, d));
        }
        let system:System = s.into();
        systems.insert(system_name.clone(), system);
    }
    // info!("systems: {:?}", systems);

    info!("dates: {:?}", dates);
    let bgs_day = dates.iter().max().unwrap().clone().pred(); // make "pred" optional
    info!("Current BGS day: {}", bgs_day);
    for &(system,date) in &system_dates {
        if bgs_day != date {
            warn!("System is not up to date: {}: {} {}", system, bgs_day, date);
            //warnings.push(format!("System is not up to date: {}", system));
        }
    }

    let mut global_factions = HashMap::new();
    let n = format!("{}/minor_factions.json", datadir);
    let f = File::open(&n).unwrap();
    let minor_faction_names:BTreeSet<String> = serde_json::from_reader(&f).unwrap();
    for minor_faction_name in minor_faction_names {
        info!("Looking at {}...", minor_faction_name);
        let n = format!("{}/factions/{}.json", datadir, minor_faction_name);
        let f = File::open(&n).unwrap();
        let mut factionv4:ebgsv4::EBGSFactionsV4 = serde_json::from_reader(&f).unwrap();
        let n = format!("{}/factions/eddb/{}.json", datadir, minor_faction_name);
        let f = File::open(&n).unwrap();
        let faction_eddb:eddbv3::Faction = serde_json::from_reader(&f).unwrap();
        let fgs:FactionGlobalState = (&factionv4).into();
        global_factions.insert(minor_faction_name.clone(), fgs);
        for system in factionv4.systems() {
            if !system_names.contains(&system) {
                continue;
            }
            if factionv4.bgs_day(&system) != bgs_day {
                warn!("Faction {} is not up to date in {}: {} {}", minor_faction_name, system, bgs_day, factionv4.bgs_day(&system));
                let v = system_warnings.entry(system.clone()).or_insert(vec![]);
                v.push(format!("Faction {} is not up to date in {}", minor_faction_name, system));
            }
        }
        let faction_template:Faction = (&factionv4).into();
        for history in factionv4.history {
            let mut at_home = false;
            // could be that the system is not in our system list...
            if let Some(system) = systems.get_mut(&history.system) {
                let faction = system.factions.entry(faction_template.name.clone()).or_insert(faction_template.clone());
                faction.eddb = Some(faction_eddb.clone());
                if let Some(home_id) = faction_eddb.home_system_id {
                    if system.eddb_id == home_id {
                        faction.at_home = true;
                        at_home = true;
                    }
                }
                let inf = history.influence*100.0;
                let mut data:FactionData = history.into();
                if !at_home && inf < 2.5 {
                    data.influence_danger = true;
                }
                if inf >= 75.0 {
                    data.influence_danger = true;
                }
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
            faction.fill_in_evolution10(&dates_vec);
            faction.global = global_factions.get(&faction.name).map(|x| x.clone());
            // remove systemname from global if it is the local system
            if let Some(ref mut gl) = faction.global {
                let mut go = false;
                if let Some(ref mut s2) = gl.state_system {
                    if s2 == &system.name {
                        go = true;
                    }
                }
                if go {
                    gl.state_system = None;
                }
            }
        }
        let mut colors:BTreeSet<String> = vec!["blue", "green", "cyan", "orange",
                                               "pink", "grey", "magenta", "yellow", "red"].into_iter().map(|x| x.into()).collect();
        // first fill in using registered colors, but only if no duplicates
        for faction in &mut system.factions.values_mut() {
            if faction.name == "The Order of Mobius" {
                faction.color = "black".into();
                continue
            }
            if let Some(color) = faction_colors.get(&faction.name) {
                //println!("Found existing color: {} for {}", color, faction.name);
                if colors.contains(color) {
                    //println!("Using existing");
                    faction.color = color.clone();
                    colors.remove(color);
                } else {
                    //println!("Skipping existing");
                }
            }
        }
        let mut it = colors.into_iter();
        for faction in &mut system.factions.values_mut() {
            if faction.color.is_empty() {
                faction.color = it.next().unwrap().into();
                faction_colors.insert(faction.name.clone(), faction.color.clone());
            }
        }
        for faction in system.factions.values() {
            system.factions_by_inf.push(faction.clone())
        }
        system.factions_by_inf.sort_by(
            |a,b| b.latest_inf().cmp(&a.latest_inf())
        );
    }

    let n = format!("{}/report.json", datadir);
    let f = File::create(&n).unwrap();

    // order by order in Config
    let mut s2:Vec<System> = vec![];
    for name in &system_names {
        let mut system = systems.remove(name).unwrap();
        system.warnings = system_warnings.remove(name).unwrap_or(vec![]);
        s2.push(system)
    }

    let dates:Vec<String> = dates_vec.iter().skip(1).map(|e| format!("{}", e.format("%d/%m"))).collect();
    let dates2 = dates.clone();
    let dates10 = dates.as_slice();
    let dates10 = dates10.windows(10).last().unwrap().to_vec();
    let systems = Systems {
        systems: s2,
        dates:dates2,
        dates10:dates10,
        warnings:warnings,
        bgs_day:format!("{}", bgs_day.format("%d/%m")),
        factions:global_factions,
    };
    serde_json::to_writer_pretty(&f, &systems).unwrap();
}
