// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc};

use extdata::ebgsv4;
use extdata::eddbv3;

/// `Allegiance` of a `Faction`
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Allegiance {
    Independent,
    Federation,
    Alliance,
    Empire,
}

// for factions
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
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
    Engineer,
}

impl From<ebgsv4::Government> for Government {
    fn from(g:ebgsv4::Government) -> Government {
        match g {
            ebgsv4::Government::Anarchy => Government::Anarchy,
            ebgsv4::Government::Corporate => Government::Corporate,
            ebgsv4::Government::Patronage => Government::Patronage,
            ebgsv4::Government::Communism => Government::Communism,
            ebgsv4::Government::Confederacy => Government::Confederacy,
            ebgsv4::Government::Cooperative => Government::Cooperative,
            ebgsv4::Government::Democracy => Government::Democracy,
            ebgsv4::Government::Dictatorship => Government::Dictatorship,
            ebgsv4::Government::Feudal => Government::Feudal,
            ebgsv4::Government::Imperial => Government::Imperial,
            ebgsv4::Government::PrisonColony => Government::PrisonColony,
            ebgsv4::Government::Theocracy => Government::Theocracy,
            ebgsv4::Government::Workshop => Government::Workshop,
            ebgsv4::Government::None => Government::None,
            ebgsv4::Government::Engineer => Government::Engineer,
        }
    }
}

/// `State` of a `Faction`
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<ebgsv4::State> for State {
    fn from(s: ebgsv4::State) -> State {
        match s {
            ebgsv4::State::None => State::None,
            ebgsv4::State::Expansion => State::Expansion,
            ebgsv4::State::War => State::War,
            ebgsv4::State::CivilWar => State::CivilWar,
            ebgsv4::State::Election => State::Election,
            ebgsv4::State::Boom => State::Boom,
            ebgsv4::State::Bust => State::Bust,
            ebgsv4::State::CivilUnrest => State::CivilUnrest,
            ebgsv4::State::Famine => State::Famine,
            ebgsv4::State::Outbreak => State::Outbreak,
            ebgsv4::State::Lockdown => State::Lockdown,
            ebgsv4::State::Investment => State::Investment,
            ebgsv4::State::Retreat => State::Retreat,
        }
    }
}

impl State {
    /// maximum length in days
    pub fn max_length(&self) -> u8 {
        match *self {
            State::None => 0,
            State::Expansion => 6,
            State::War => 21,
            State::CivilWar => 21,
            State::Election => 4,
            State::Boom => 28,
            State::Bust => 28,
            State::CivilUnrest => 7,
            State::Famine => 28,
            State::Outbreak => 28,
            State::Lockdown => 14,
            State::Investment => 5,
            State::Retreat => 6,
        }
    }

    /// days recovery
    pub fn recovery(&self) -> u8 {
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

    /// days pending
    pub fn pending(&self) -> u8 {
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

    /// if a state is a danger state
    pub fn danger(&self) -> bool {
        match *self {
            State::Expansion => true,
            State::Investment => true,
            State::Retreat => true,
            _ => false,
        }
    }

    /// if a pending state is a dangerous situation
    pub fn pending_danger(&self) -> bool {
        match *self {
            State::Expansion => true,
            State::Investment => true,
            State::Retreat => true,
            _ => false,
        }
    }

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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Security {
    Medium,
    Low,
    High,
    Anarchy,
    Anarchy2,
    Lawless,
}

impl From<ebgsv4::Security> for Security {
    fn from(s:ebgsv4::Security) -> Security {
        match s {
            ebgsv4::Security::Anarchy => Security::Anarchy,
            ebgsv4::Security::Medium => Security::Medium,
            ebgsv4::Security::Low => Security::Low,
            ebgsv4::Security::High => Security::High,
            ebgsv4::Security::Anarchy2 => Security::Anarchy2,
            ebgsv4::Security::Lawless => Security::Lawless,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Economy {
    Industrial,
    Extraction,
    Colony,
    Agriculture,
    Tourism,
    HighTech,
    Terraforming,
    Refinery,
    Military,
}

impl From<ebgsv4::Economy> for Economy {
    fn from(e:ebgsv4::Economy) -> Economy {
        match e {
            ebgsv4::Economy::Agriculture => Economy::Agriculture,
            ebgsv4::Economy::Extraction => Economy::Extraction,
            ebgsv4::Economy::Colony => Economy::Colony,
            ebgsv4::Economy::Industrial => Economy::Industrial,
            ebgsv4::Economy::Tourism => Economy::Tourism,
            ebgsv4::Economy::HighTech => Economy::HighTech,
            ebgsv4::Economy::Terraforming => Economy::Terraforming,
            ebgsv4::Economy::Refinery => Economy::Refinery,
            ebgsv4::Economy::Military => Economy::Military,

        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Faction {
    pub ebgs_id: String,
    pub name: String,
    pub eddbv3_updated_at: DateTime<Utc>,
    pub government: Government,
    pub allegiance: Allegiance,
    pub state: State,
    pub home_system_id: Option<i64>,
    pub is_player_faction: bool,
}

impl From<eddbv3::Faction> for Faction {
    fn from(f: eddbv3::Faction) -> Faction {
        Faction {
            ebgs_id: f._id,
            name: f.name,
            eddbv3_updated_at: f.updated_at,
            government: f.government,
            allegiance: f.allegiance,
            state: f.state.into(),
            home_system_id: f.home_system_id,
            is_player_faction: f.is_player_faction,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub eddb_id: i64,
    pub ebgs_id: String,
    pub name: String,
    pub allegiance: Allegiance,
    pub population: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub updated_at: DateTime<Utc>,
    pub state: State,
    pub security: Security,
    pub controlling_minor_faction: String,
    pub primary_economy: Economy,
    pub government: Government,
    //pub factions: Vec<SystemPresence>,
    //pub history: Vec<SystemHistory>,
}

impl From<ebgsv4::System> for System {
    fn from(s: ebgsv4::System) -> System {
        System {
            eddb_id: s.eddb_id,
            ebgs_id: s._id,
            name: s.name,
            allegiance: s.allegiance,
            population: s.population,
            x: s.x,
            y: s.y,
            z: s.z,
            updated_at: s.updated_at,
            state: s.state.into(),
            security: s.security.into(),
            controlling_minor_faction: s.controlling_minor_faction,
            primary_economy:s.primary_economy.into(),
            government:s.government.into(),
        }
    }
}