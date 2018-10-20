// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc};

use extdata::ebgsv4::EBGSPage;

pub const URL: &'static str = "https://elitebgs.kodeblox.com/api/eddb/v3/";

pub type FactionPage = EBGSPage<Faction>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Faction {
    pub _id: String,
    pub name_lower: String,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub government_id: u8,
    pub government: Government,
    pub allegiance_id: u8,
    pub allegiance: Allegiance,
    pub state_id: u8,
    pub state: State,
    pub home_system_id: Option<i64>,
    pub is_player_faction: bool,
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

/// `State` of a `Faction`
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
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