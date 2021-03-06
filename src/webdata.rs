// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{Date,DateTime,Utc};
use data::{Allegiance, Government, State};
use data;

use std::collections::{BTreeMap,HashMap,HashSet};

use serde::de::{self, Deserialize, Deserializer};

#[derive(Debug,Deserialize, Serialize)]
pub struct Systems {
    pub report_name: String,
    pub systems: Vec<System>,
    pub dates: Vec<String>,
    pub dates10: Vec<String>,
    pub bgs_day: String,
    pub factions: HashMap<String, FactionGlobalState>,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct System {
    pub eddb_id:i64,
    pub name:String,
    pub population: i64,
    pub factions:HashMap<String, Faction>,
    pub factions_by_inf:Vec<Faction>,
    pub warnings:Vec<String>,
    pub controlling:String,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct Faction {
    pub name:String,
    pub government:Government,
    pub allegiance:Allegiance,
    pub evolution:Vec<FactionData>,
    pub evolution10:Vec<FactionData>,
    pub global:Option<FactionGlobalState>,
    pub color:String,
    pub at_home:bool,
    pub controlling:bool,
    pub is_player_faction:bool,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct FactionGlobalState {
    pub name:String,
    pub government:Government,
    pub allegiance:Allegiance,
    pub state_date:DateTime<Utc>,
    pub state_day:Option<u8>,
    pub state_max_length:u8,
    pub state_danger:bool,
    pub state:State,
    pub state_system:Option<String>,
    pub pending_state:Option<State>,
    pub pending_state_system:Option<String>,
    pub recovery_state:Option<State>,
    pub recovery_state_system:Option<String>,
    pub is_player_faction:bool,
}

// faction data (for in a specific system)
#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct FactionData {
    pub date:DateTime<Utc>,
    pub label_date:String,
    pub influence:f64,
    pub state:State,
    pub state_day:u8,
    pub state_max_length:u8,
    pub state_danger:bool,
    pub pending_states:Vec<FactionState>,
    pub recovering_states:Vec<FactionState>,
    pub influence_danger:bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FactionState {
    pub state:State,
    pub state_recovery_length:u8,
    pub state_pending_length:u8,
    pub trend:data::Trend,
    pub trend_display:String,
    pub state_day:u8,
    pub state_pending_danger:bool,
}

impl From<data::System> for System {
    fn from(s:data::System) -> System {
        System {
            eddb_id:s.eddb_id,
            name:s.name.clone(),
            population:s.population,
            factions:HashMap::new(),
            factions_by_inf:vec![],
            warnings:vec![],
            controlling:s.dynamic.controlling_minor_faction,
        }
    }
}

impl<'a> From<&'a data::Faction> for Faction {
    fn from(s:&'a data::Faction) -> Faction {
        Faction {
            name:s.name.clone(),
            government:s.government,
            allegiance:s.allegiance,
            evolution:vec![],
            evolution10:vec![],
            color:"".into(),
            global:None,
            at_home:false,
            controlling:false,
            is_player_faction:s.is_player_faction,
        }
    }
}

impl<'a> From<&'a data::Faction> for FactionGlobalState {
    fn from(s:&'a data::Faction) -> FactionGlobalState {
        let (state, system) = s.faction_state();
        let state:State = state.into();
        let (pending_state, pending_system) = s.faction_pending_single_system_state();
        let (recovery_state, recovery_system) = s.faction_recovering_single_system_state();
        FactionGlobalState {
            name:s.name.clone(),
            government:s.government,
            allegiance:s.allegiance,
            state:state,
            state_system:system,
            state_date:s.dynamic.eddbv3_updated_at,
            state_day:None,
            state_max_length:state.max_length(),
            state_danger:state.danger(),
            pending_state:pending_state,
            pending_state_system:pending_system,
            recovery_state:recovery_state,
            recovery_state_system:recovery_system,
            is_player_faction:s.is_player_faction,
        }
    }
}

impl From <data::FactionHistory> for FactionData {
    fn from(h:data::FactionHistory) -> FactionData {
        let state:State = h.presence.state.into();
        FactionData {
            date:h.updated_at,
            label_date:format!("{}", h.updated_at.format("%d/%m")),
            influence:h.presence.influence,
            state:state,
            state_day:0,
            state_max_length:state.max_length(),
            pending_states:h.presence.pending_states.into_iter().map(|s| s.into()).collect(),
            recovering_states:h.presence.recovering_states.into_iter().map(|s| s.into()).collect(),
            state_danger:state.danger(),
            influence_danger:false,
        }
    }
}

impl From <data::StateTrend> for FactionState {
    fn from(s:data::StateTrend) -> FactionState {
        let d = match s.trend {
            data::Trend::Up => "&uarr;",
            data::Trend::Down => "&darr;",
            data::Trend::Level => "&harr;",
        }.into();
        let state:State = s.state.into();
        FactionState {
            state:state,
            trend:s.trend,
            trend_display:d,
            state_day:0,
            state_recovery_length:state.recovery(),
            state_pending_length:state.pending(),
            state_pending_danger:state.pending_danger(),
        }
    }
}

fn update_states(states:&mut Vec<FactionState>, h:&mut HashMap<State,u8>) {
    let mut seen = HashSet::new();
    for state in states {
        seen.insert(state.state);
        if !h.contains_key(&state.state) {
            h.insert(state.state, 1);
            state.state_day = 1;
        } else {
            let n = h.get(&state.state).unwrap() + 1;
            h.insert(state.state, n);
            state.state_day = n;
        }
    }
    let keys:Vec<State> = h.keys().cloned().collect();
    for k in keys {
        if !seen.contains(&k) {
            h.remove(&k);
        }
    }
}

impl Faction {
    pub fn cleanup_evolution(&mut self, dates:&Vec<Date<Utc>>) {
        
        // only take first changed inf of a day
        // or if all are the same it doesn't matter
        
        // build up a btreemap with a vec of datas per day
        let mut b = BTreeMap::new();
        for e in &self.evolution {
            let date = e.date.date();
            if !b.contains_key(&date) {
                b.insert(date, vec![e.clone()]);
            } else {
                b.get_mut(&date).unwrap().push(e.clone())
            }
        }

        // then take the first inf found for each day
        // that is not equal to the previous day
        let mut v = vec![];
        let mut prev_inf = 0.0;
        for (_day, values) in b {
            let mut found = false;
            for val in &values {
                if val.influence != prev_inf {
                    v.push(val.clone());
                    prev_inf = val.influence;
                    found = true;
                    break;
                }
            }
            // if all were same, just take the first
            if !found {
                info!("{} INF stayed equal", self.name);
                v.push(values[0].clone())
            }
        }
        
        // also _insert_ in between days
        let mut di = dates.iter();
        let mut prev:Option<FactionData> = None;
        let mut v2 = vec![];
        for e in v {
            let mut date = di.next().unwrap();
            while *date != e.date.date() {
                if let Some(e2) = prev {
                    let mut e3 = e2.clone();
                    e3.date = date.and_hms(12,30,0);
                    e3.label_date = format!("{}", date.format("%d/%m"));
                    v2.push(e3.clone());
                    prev = Some(e3);
                } else {
                    // nothing... :(
                }
                date = di.next().unwrap();
            }
            v2.push(e.clone());
            prev = Some(e.clone());
        }
        self.evolution = v2;
    }

    pub fn fill_in_state_days(&mut self) {
        let mut prev_state = State::None;
        let mut recovery_states = HashMap::new();
        let mut pending_states = HashMap::new();
        let mut c:u8 = 1;
        for e in &mut self.evolution {
            if e.state != prev_state {
                prev_state = e.state;
                c = 1;
                e.state_day = c;
            } else {
                c += 1;
                e.state_day = c;
            }
            update_states(&mut e.pending_states, &mut pending_states);
            update_states(&mut e.recovering_states, &mut recovery_states);
        }
    }

    pub fn fill_in_evolution10(&mut self, dates: &Vec<Date<Utc>>) {
        let dates10 = dates.as_slice().windows(10).last().unwrap().to_vec();
        let mut ev = vec![];
        for e in &self.evolution {
            if dates10.contains(&e.date.date()) {
                ev.push(e.clone())
            }
        }
        self.evolution10 = ev;
    }

    pub fn latest_inf(&self) -> i64 {
        (self.evolution.last().unwrap().influence * 1000.0) as i64
    }

    pub fn fill_in_state_other_system(&mut self) {
        // TODO
    }
}

// custom deserializer needed for state to deal with civil unrest vs civilunrest
impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let state = match s.as_str() {
            "expansion" => State::Expansion,
            "war" => State::War,
            "civil unrest" | "civilunrest" => State::CivilUnrest,
            "civil war" | "civilwar" => State::CivilWar,
            "election" => State::Election,
            "boom" => State::Boom,
            "bust" => State::Bust,
            "famine" => State::Famine,
            "lockdown" => State::Lockdown,
            "investment" => State::Investment,
            "retreat" => State::Retreat,
            "outbreak" => State::Outbreak,
            "none" => State::None,
            other => { return Err(de::Error::custom(format!("Invalid state '{}'", other))); },
        };
        Ok(state)
    }
}
