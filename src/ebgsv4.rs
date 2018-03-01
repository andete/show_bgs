use chrono::{DateTime,Utc};

#[derive(Debug,Deserialize)]
pub struct EBGSFactionsPageV4 {
    pub docs:Vec<EBGSFactionsV4>,
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
    pub state:String,
    pub pending_states:Vec<EBGSStateV4>,
    pub recovering_states:Vec<EBGSStateV4>,
    pub influence: f64,
    pub system_name_lower:String,
}

#[derive(Debug,Deserialize)]
pub struct EBGSStateV4 {
    pub state:String,
    pub trend:i64,
}

#[derive(Debug,Deserialize)]
pub struct EBGSFactionHistoryV4 {
    pub system:String,
    pub state:String,
    pub updated_at:DateTime<Utc>,
    pub system_lower:String,
    pub updated_by:String,
    pub pending_states:Vec<EBGSStateV4>,
    pub recovering_states:Vec<EBGSStateV4>,
    pub _id:String,
    pub influence:f64,
}
