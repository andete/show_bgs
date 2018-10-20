use chrono::{DateTime,Utc};
use data::*;

use ebgsv4::EBGSPageV4;

pub const URL:&'static str = "https://elitebgs.kodeblox.com/api/eddb/v3/";

pub type FactionPage = EBGSPageV4<Faction>;

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct Faction {
    pub _id:String,
    pub name_lower:String,
    pub name:String,
    pub updated_at:DateTime<Utc>,
    pub government_id:u8,
    pub government:GovernmentFaction,
    pub allegiance_id:u8,
    pub allegiance:Allegiance,
    pub state_id:u8,
    pub state:State,
    pub home_system_id:Option<i64>,
    pub is_player_faction:bool,
}
