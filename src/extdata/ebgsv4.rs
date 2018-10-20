// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{Date, DateTime, Utc};
use data::{Security, Economy};
use serde::de::{self, Deserialize, Deserializer};

use std::collections::BTreeSet;


pub const URL:&'static str = "https://elitebgs.kodeblox.com/api/ebgs/v4/";

#[derive(Debug, Deserialize)]
pub struct EBGSPage<T> {
    pub docs: Vec<T>,
    pub page: i64,
    pub pages: i64,
    pub total: i64,
    pub limit: i64,
}

pub type FactionsPage = EBGSPage<Faction>;
pub type SystemsPage = EBGSPage<System>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Faction {
    pub eddb_id: i64,
    pub government: Government,
    pub name: String,
    pub _id: String,
    pub name_lower: String,
    //pub is_player_faction:bool,
    pub updated_at: DateTime<Utc>,
    pub faction_presence: Vec<FactionPresence>,
    pub allegiance: Allegiance,
    pub history: Vec<FactionHistory>,
}

impl Faction {
    pub fn bgs_day(&self, system: &str) -> Date<Utc> {
        let mut dates = BTreeSet::new();
        for h in &self.history {
            if &h.system == system {
                dates.insert(h.updated_at.date());
            }
        }
        dates.iter().max().unwrap().clone()
    }

    pub fn systems(&self) -> Vec<String> {
        self.faction_presence.iter().map(|x| x.system_name.clone()).collect()
    }

    pub fn last_state_in_system(&self, system: &str) -> State {
        let mut state = State::None;
        let mut date = None;
        for h in &self.history {
            if &h.system == system {
                if Some(h.updated_at) != date {
                    state = h.state;
                    date = Some(h.updated_at);
                }
            }
        }
        state
    }

    pub fn last_pending_state_in_system(&self, system: &str) -> Option<State> {
        let mut state = None;
        let mut date = None;
        for h in &self.history {
            if &h.system == system {
                if Some(h.updated_at) != date {
                    state = h.pending_states.iter().filter(|x| x.state.is_single_system_state()).next().map(|x| x.state);
                    date = Some(h.updated_at);
                }
            }
        }
        state
    }

    pub fn last_recovering_state_in_system(&self, system: &str) -> Option<State> {
        let mut state = None;
        let mut date = None;
        for h in &self.history {
            if &h.system == system {
                if Some(h.updated_at) != date {
                    state = h.recovering_states.iter().filter(|x| x.state.is_single_system_state()).next().map(|x| x.state);
                    date = Some(h.updated_at);
                }
            }
        }
        state
    }

    // TODO: this doesn't work correctly if some data is dated
    pub fn faction_state(&self) -> (State, Option<String>) {
        let mut p_state = State::None;
        for system in self.systems() {
            let state = self.last_state_in_system(&system);
            if state.is_single_system_state() {
                info!("XXX {} {} {:?}", self.name, system, state);
                return (state, Some(system));
            }
            p_state = state;
        }
        (p_state, None)
    }

    // TODO: this doesn't work correctly if some data is dated
    pub fn faction_pending_state(&self) -> (Option<State>, Option<String>) {
        for system in self.systems() {
            if let Some(state) = self.last_pending_state_in_system(&system) {
                info!("XXX {} {} {:?}", self.name, system, state);
                if state.is_single_system_state() {
                    return (Some(state), Some(system))
                }
            }
        }
        (None, None)
    }

    // TODO: this doesn't work correctly if some data is dated
    pub fn faction_recovering_state(&self) -> (Option<State>, Option<String>) {
        for system in self.systems() {
            if let Some(state) = self.last_recovering_state_in_system(&system) {
                info!("XXX {} {} {:?}", self.name, system, state);
                if state.is_single_system_state() {
                    return (Some(state), Some(system))
                }
            }
        }
        (None, None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FactionPresence {
    pub system_name: String,
    pub state: State,
    pub pending_states: Vec<EBGSState>,
    pub recovering_states: Vec<EBGSState>,
    pub influence: f64,
    pub system_name_lower: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSState {
    pub state: State,
    pub trend: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FactionHistory {
    pub system: String,
    pub state: State,
    pub updated_at: DateTime<Utc>,
    pub system_lower: String,
    pub updated_by: String,
    pub pending_states: Vec<EBGSState>,
    pub recovering_states: Vec<EBGSState>,
    pub _id: String,
    pub influence: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub eddb_id: i64,
    pub name_lower: String,
    pub allegiance: Allegiance,
    pub _id: String,
    pub population: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub updated_at: DateTime<Utc>,
    pub state: State,
    pub security: Security,
    pub controlling_minor_faction: String,
    pub primary_economy: Economy,
    pub name: String,
    pub government: Government,
    pub factions: Vec<SystemPresence>,
    pub history: Vec<SystemHistory>,
}

impl System {
    pub fn bgs_day(&self) -> Option<Date<Utc>> {
        use std::iter;
        iter::once(self.updated_at).chain(
            self.history.iter().map(|x| x.updated_at)).map(|x| x.date()).max()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SystemPresence {
    pub name: String,
    pub name_lower: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemHistory {
    pub controlling_minor_faction: String,
    pub security: Security,
    pub updated_at: DateTime<Utc>,
    pub state: State,
    pub government: Government,
    pub population: i64,
    pub updated_by: String,
    pub allegiance: Allegiance,
    pub factions: Vec<SystemPresence>,
}

// `State` of a `Faction`
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Government {
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

/// `Allegiance` of a `Faction`
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Allegiance {
    Independent,
    Federation,
    Alliance,
    Empire,
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

impl State {

    /// is this a state that only gets active in a single system
    pub fn is_single_system_state(&self) -> bool {
        match *self {
            State::Boom => false,
            State::Bust => false,
            State::CivilUnrest => false,
            State::Outbreak => false,
            State::Famine => false,
            State::Lockdown => false,
            State::None => false,
            State::Retreat => false,
            _ => true,
        }
    }
}