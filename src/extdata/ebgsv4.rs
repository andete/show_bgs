// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc};
use serde::de::{self, Deserialize, Deserializer};

use data;

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
    pub government: data::Government,
    pub name: String,
    pub _id: String,
    pub name_lower: String,
    //pub is_player_faction:bool,
    pub updated_at: DateTime<Utc>,
    pub faction_presence: Vec<FactionPresence>,
    pub allegiance: data::Allegiance,
    pub history: Vec<FactionHistory>,
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
    pub allegiance: data::Allegiance,
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
    pub allegiance: data::Allegiance,
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
    #[serde(rename = "$government_anarchy;")]
    Anarchy,
    #[serde(rename = "$government_communism;")]
    Communism,
    #[serde(rename = "$government_confederacy;")]
    Confederacy,
    #[serde(rename = "$government_feudal;")]
    Feudal,
    #[serde(rename = "$government_imperial;")]
    Imperial,
    #[serde(rename = "$government_prison_colony;")]
    PrisonColony,
    #[serde(rename = "$government_theocracy;")]
    Theocracy,
    #[serde(rename = "$government_workshop;")]
    Workshop,
    #[serde(rename = "$government_none;")]
    None,
    #[serde(rename = "$government_engineer;")]
    Engineer,
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Security {
    #[serde(rename = "$system_security_medium;")]
    Medium,
    #[serde(rename = "$system_security_low;")]
    Low,
    #[serde(rename = "$system_security_high;")]
    High,
    #[serde(rename = "$system_security_anarchy;")]
    Anarchy,
    #[serde(rename = "$galaxy_map_info_state_anarchy;")]
    Anarchy2,
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
    #[serde(rename = "$economy_military;")]
    Military,
}