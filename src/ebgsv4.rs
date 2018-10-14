use chrono::{Date, DateTime, Utc};
use data::*;

use std::collections::BTreeSet;

#[derive(Debug, Deserialize)]
pub struct EBGSPageV4<T> {
    pub docs: Vec<T>,
    pub page: i64,
    pub pages: i64,
    pub total: i64,
    pub limit: i64,
}

pub type EBGSFactionsPageV4 = EBGSPageV4<EBGSFactionsV4>;
pub type EBGSSystemsPageV4 = EBGSPageV4<EBGSSystemsV4>;

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSFactionsV4 {
    pub eddb_id: i64,
    pub government: GovernmentFaction,
    pub name: String,
    pub _id: String,
    pub name_lower: String,
    //pub is_player_faction:bool,
    pub updated_at: DateTime<Utc>,
    pub faction_presence: Vec<EBGSFactionPresenceV4>,
    pub allegiance: Allegiance,
    pub history: Vec<EBGSFactionHistoryV4>,
}

impl EBGSFactionsV4 {
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
            info!("XXX {} {} {:?}", self.name, system, state);
            if state.is_single_system_state() {
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
pub struct EBGSFactionPresenceV4 {
    pub system_name: String,
    pub state: State,
    pub pending_states: Vec<EBGSStateV4>,
    pub recovering_states: Vec<EBGSStateV4>,
    pub influence: f64,
    pub system_name_lower: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSStateV4 {
    pub state: State,
    pub trend: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSFactionHistoryV4 {
    pub system: String,
    pub state: State,
    pub updated_at: DateTime<Utc>,
    pub system_lower: String,
    pub updated_by: String,
    pub pending_states: Vec<EBGSStateV4>,
    pub recovering_states: Vec<EBGSStateV4>,
    pub _id: String,
    pub influence: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSSystemsV4 {
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
    pub factions: Vec<EBGSSystemPresenceV4>,
    pub history: Vec<EBGSSystemHistoryV4>,
}

impl EBGSSystemsV4 {
    pub fn bgs_day(&self) -> Option<Date<Utc>> {
        use std::iter;
        iter::once(self.updated_at).chain(
            self.history.iter().map(|x| x.updated_at)).map(|x| x.date()).max()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSSystemPresenceV4 {
    pub name: String,
    pub name_lower: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EBGSSystemHistoryV4 {
    pub controlling_minor_faction: String,
    pub security: Security,
    pub updated_at: DateTime<Utc>,
    pub state: State,
    pub government: Government,
    pub population: i64,
    pub updated_by: String,
    pub allegiance: Allegiance,
    pub factions: Vec<EBGSSystemPresenceV4>,
}
