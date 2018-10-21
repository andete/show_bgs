// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{DateTime, Utc};

use extdata::ebgsv4::{State, EBGSPage};
use data::Allegiance;
use data::Government;

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
