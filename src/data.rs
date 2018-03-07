use chrono::{Date,DateTime,Utc};
use ebgsv4;

use std::collections::{BTreeMap,HashMap,HashSet};

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
    Bust,
    CivilUnrest,
    Famine,
    Outbreak,
    Lockdown,
    Investment,
    Retreat,
}

impl State {
    fn max_length(&self) -> u8 {
        match *self {
            State::None => 0,
            State::Expansion => 6,
            State::War => 21,
            State::CivilWar => 21,
            State::Election => 5,
            State::Boom => 28,
            State::Bust => 28,
            State::CivilUnrest => 7,
            State::Famine => 28,
            State::Outbreak => 28,
            State::Lockdown => 14,
            State::Investment => 5,
            State::Retreat => 5,
        }
    }

    fn recovery(&self) -> u8 {
        match *self {
            State::None => 0,
            State::Expansion => 2,
            State::War => 1,
            State::CivilWar => 1,
            State::Election => 2,
            State::Boom => 3,
            State::Bust => 3,
            State::CivilUnrest => 3,
            State::Famine => 7,
            State::Outbreak => 8,
            State::Lockdown => 1,
            State::Investment => 1,
            State::Retreat => 1,
        }
    }
    
    fn pending(&self) -> u8 {
        match *self {
            State::None => 0,
            State::Expansion => 5,
            State::War => 3,
            State::CivilWar => 3,
            State::Election => 3,
            State::Boom => 1,
            State::Bust => 2,
            State::CivilUnrest => 1,
            State::Famine => 2,
            State::Outbreak => 1,
            State::Lockdown => 1,
            State::Investment => 0,
            State::Retreat => 1,
        }
    }

    fn danger(&self) -> bool {
        match *self {
            State::Expansion => true,
            State::Investment => true,
            State::Retreat => true,
            _ => false,
        }
    }

    fn pending_danger(&self) -> bool {
        match *self {
            State::Expansion => true,
            State::Investment => true,
            State::Retreat => true,
            _ => false,
        }
    }

    pub fn is_single_system_state(&self) -> bool {
        match *self {
            State::Boom => false,
            State::Bust => false,
            State::CivilUnrest => false,
            State::Outbreak => false,
            State::Famine => false,
            State::Lockdown => false,
            State::None => false,
            _ => true,
        }
    }
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
    #[serde(rename = "$government_democracy;")]
    Democracy,
    #[serde(rename = "$government_dictatorship;")]
    Dictatorship,
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
    #[serde(rename = "$economy_terraforming;")]
    Terraforming,
    #[serde(rename = "$economy_refinery;")]
    Refinery,
}
#[derive(Debug,Deserialize, Serialize)]
pub struct Systems {
    pub systems: Vec<System>,
    pub dates: Vec<String>,
    pub dates10: Vec<String>,
    pub warnings: Vec<String>,
    pub bgs_day: String,
    pub factions: HashMap<String, FactionGlobalState>,
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
    pub evolution10:Vec<FactionData>,
    pub global:Option<FactionGlobalState>,
    pub color:String,
    //pub is_player_faction:bool,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct FactionGlobalState {
    pub name:String,
    pub government:GovernmentFaction,
    pub allegiance:Allegiance,
    pub state:State,
    pub state_day:Option<u8>,
    pub state_max_length:u8,
    pub state_danger:bool,
    pub state_system:Option<String>,
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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FactionState {
    pub state:State,
    pub state_recovery_length:u8,
    pub state_pending_length:u8,
    pub trend:i64,
    pub trend_display:String,
    pub state_day:u8,
    pub state_pending_danger:bool,
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
            evolution10:vec![],
            color:"".into(),
            global:None,
            //is_player_faction:s.is_player_faction,
        }
    }
}

impl<'a> From<&'a ebgsv4::EBGSFactionsV4> for FactionGlobalState {
    fn from(s:&'a ebgsv4::EBGSFactionsV4) -> FactionGlobalState {
        let (state, system) = s.faction_state();
        FactionGlobalState {
            name:s.name.clone(),
            government:s.government,
            allegiance:s.allegiance,
            state:state,
            state_system:system,
            state_day:None,
            state_max_length:state.max_length(),
            state_danger:state.danger(),
        }
    }
}

impl From <ebgsv4::EBGSFactionHistoryV4> for FactionData {
    fn from(h:ebgsv4::EBGSFactionHistoryV4) -> FactionData {
        FactionData {
            date:h.updated_at,
            label_date:format!("{}", h.updated_at.format("%d/%m")),
            influence:h.influence,
            state:h.state,
            state_day:0,
            state_max_length:h.state.max_length(),
            pending_states:h.pending_states.into_iter().map(|s| s.into()).collect(),
            recovering_states:h.recovering_states.into_iter().map(|s| s.into()).collect(),
            state_danger:h.state.danger(),
        }
    }
}

impl From <ebgsv4::EBGSStateV4> for FactionState {
    fn from(s:ebgsv4::EBGSStateV4) -> FactionState {
        let d = if s.trend == 1 {
            "&uarr;"
        } else if s.trend == -1 {
            "&darr;"
        } else {
            "&harr;"
        }.into();
        FactionState {
            state:s.state,
            trend:s.trend,
            trend_display:d,
            state_day:0,
            state_recovery_length:s.state.recovery(),
            state_pending_length:s.state.pending(),
            state_pending_danger:s.state.pending_danger(),
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
            for val in values {
                if val.influence != prev_inf {
                    v.push(val.clone());
                    prev_inf = val.influence;
                    break;
                }
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

    pub fn fill_in_evolution10(&mut self) {
        self.evolution10 = self.evolution.as_slice().windows(10).last().unwrap().to_vec();
    }
}
