use chrono::{DateTime,Utc};

#[derive(Debug,Deserialize)]
pub struct EBGSPageV4<T> {
    pub docs:Vec<T>,
    pub page:i64,
    pub pages:i64,
    pub total:i64,
    pub limit:i64,
}

#[derive(Debug,Deserialize)]
pub struct EBGSFactionsV4 {
    pub eddb_id:i64,
    pub government:String,
    pub name:String,
    pub _id:String,
    pub name_lower:String,
    pub updated_at:DateTime<Utc>,
    pub faction_presence:Vec<EBGSFactionPresenceV4>,
    pub allegiance:String,
    pub history:Vec<EBGSFactionHistoryV4>,
}

#[derive(Debug,Deserialize)]
pub struct EBGSFactionPresenceV4 {
    pub system_name:String,
    pub state:String, // TODO: Enum
    pub pending_states:Vec<EBGSStateV4>,
    pub recovering_states:Vec<EBGSStateV4>,
    pub influence: f64,
    pub system_name_lower:String,
}

#[derive(Debug,Deserialize)]
pub struct EBGSStateV4 {
    pub state:String, // TODO: enum
    pub trend:i64,
}

#[derive(Debug,Deserialize)]
pub struct EBGSFactionHistoryV4 {
    pub system:String,
    pub state:String, // TODO: enum
    pub updated_at:DateTime<Utc>,
    pub system_lower:String,
    pub updated_by:String,
    pub pending_states:Vec<EBGSStateV4>,
    pub recovering_states:Vec<EBGSStateV4>,
    pub _id:String,
    pub influence:f64,
}

#[derive(Debug,Deserialize)]
pub struct EBGSSystemsV4 {
    pub eddb_id: i64,
    pub name_lower: String,
    pub allegiance: String, // TODO: enum
    pub _id: String,
    pub population: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub updated_at: DateTime<Utc>,
    pub state: String, // TODO: enum
    pub security: String, // TODO: enum
    pub controlling_minor_faction: String,
    pub primary_economy: String, // TODO: enum
    pub name: String,
    pub government: String, // TODO: enum
    pub factions: Vec<EBGSSystemPresenceV4>,
    pub history: Vec<EBGSSystemHistoryV4>,
}

#[derive(Debug,Deserialize)]
pub struct EBGSSystemPresenceV4 {
    pub name: String,
    pub name_lower: String,
}

#[derive(Debug,Deserialize)]
pub struct EBGSSystemHistoryV4 {
    pub controlling_minor_faction: String,
    pub security: String, // TODO: enum
    pub updated_at: DateTime<Utc>,
    pub state: String, // TODO: enum
    pub government: String, // TODO: enum
    pub population: i64,
    pub updated_by: String,
    pub allegiance: String, // TODO: enum
    pub factions: Vec<EBGSSystemPresenceV4>,
}
