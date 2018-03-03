use chrono::{DateTime,Utc};
use ebgsv4;

use std::collections::{HashMap,HashSet};

#[derive(Debug,Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Allegiance {
    Independent,
    Federation,
    Alliance,
    Empire,
}

#[derive(Debug,Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum State {
    None,
    Expansion,
    War,
    CivilWar,
    Election,
    Boom,
    CivilUnrest,
    Famine,
    Outbreak,
    Lockdown,
    Investment,
    Retreat,
}

// for systems
#[derive(Debug,Deserialize, Serialize, Clone, Copy)]
pub enum Government {
    #[serde(rename = "$government_corporate;")]
    Corporate,
    #[serde(rename = "$government_cooperative;")]
    Cooperative,
    #[serde(rename = "$government_patronage;")]
    Patronage,
    // TODO: add more as needed
}

// for factions
#[derive(Debug,Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum GovernmentFaction {
    Anarchy,
    Corporate,
    Patronage,
    Communism,
    Confederacy,
    Cooperative,
    Democracy,
    Dictatorship,
    Feudal,
    Imperial,
    PrisonColony,
    Theocracy,
    Workshop,
    None,
}

#[derive(Debug,Deserialize, Serialize, Clone, Copy)]
pub enum Security {
    #[serde(rename = "$system_security_medium;")]
    Medium,
    #[serde(rename = "$system_security_low;")]
    Low,
    #[serde(rename = "$system_security_high;")]
    High,
    #[serde(rename = "$system_security_anarchy;")]
    Anarchy,
    #[serde(rename = "$system_security_lawless;")]
    Lawless,
}
    
#[derive(Debug,Deserialize, Serialize, Clone, Copy)]
pub enum Economy {
    #[serde(rename = "$economy_industrial;")]
    Industrial,
    #[serde(rename = "$economy_extraction;")]
    Extraction,
    #[serde(rename = "$economy_colony;")]
    Colony,
    #[serde(rename = "$economy_agri;")]
    Agriculture,
    #[serde(rename = "$economy_tourism;")]
    Tourism,
    #[serde(rename = "$economy_hightech;")]
    HighTech,
}
#[derive(Debug,Deserialize, Serialize)]
pub struct Systems {
    pub systems: Vec<System>,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct System {
    pub eddb_id:i64,
    pub name:String,
    pub population: i64,
    pub factions:HashMap<String, Faction>,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct Faction {
    pub name:String,
    pub government:GovernmentFaction,
    pub allegiance:Allegiance,
    pub evolution:Vec<FactionData>,
}

// faction data (for in a specific system)
#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct FactionData {
    pub date:DateTime<Utc>,
    pub influence:f64,
    pub state:State,
    pub state_day:u8,
    pub pending_states:Vec<FactionState>,
    pub recovering_states:Vec<FactionState>,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct FactionState {
    pub state:State,
    pub trend:i64,
    pub state_day:u8,
}

impl From<ebgsv4::EBGSSystemsV4> for System {
    fn from(s:ebgsv4::EBGSSystemsV4) -> System {
        System {
            eddb_id:s.eddb_id,
            name:s.name.clone(),
            population:s.population,
            factions:HashMap::new(),
        }
    }
}

impl<'a> From<&'a ebgsv4::EBGSFactionsV4> for Faction {
    fn from(s:&'a ebgsv4::EBGSFactionsV4) -> Faction {
        Faction {
            name:s.name.clone(),
            government:s.government,
            allegiance:s.allegiance,
            evolution:vec![],
        }
    }
}

impl From <ebgsv4::EBGSFactionHistoryV4> for FactionData {
    fn from(h:ebgsv4::EBGSFactionHistoryV4) -> FactionData {
        FactionData {
            date:h.updated_at,
            influence:h.influence,
            state:h.state,
            state_day:0,
            pending_states:h.pending_states.into_iter().map(|s| s.into()).collect(),
            recovering_states:h.recovering_states.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl From <ebgsv4::EBGSStateV4> for FactionState {
    fn from(s:ebgsv4::EBGSStateV4) -> FactionState {
        FactionState {
            state:s.state,
            trend:s.trend,
            state_day:0,
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
    pub fn cleanup_evolution(&mut self) {
        let mut prev_date = None;
        let mut v = vec![];
        for e in &self.evolution {
            let date = e.date.date();
            if Some(date) != prev_date {
                v.push(e.clone());
                prev_date = Some(date);
            }
            
        }
        self.evolution = v;
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
}
