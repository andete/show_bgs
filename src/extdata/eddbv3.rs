// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc};

use extdata::ebgsv4::{Allegiance, State, EBGSPage};

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
    #[serde(rename = "$government_anarchy;")]
    Anarchy,
    #[serde(rename = "$government_communism;")]
    Communism,
    #[serde(rename = "$government_confederacy;")]
    Confederacy,
    // TODO: add more as needed
}
