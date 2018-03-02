use chrono::{DateTime,Utc};

#[derive(Debug,Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Allegiance {
    Independent,
    Federation,
    Alliance,
    Empire,
}

#[derive(Debug,Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    None,
    Expansion,
    War,
    CivilWar,
    Election,
    Boom,
    CivilUnrest,
    Famine,
    Outbreak,
    Lockdown,
    Investment,
    Retreat,
}

// for systems
#[derive(Debug,Deserialize, Serialize)]
pub enum Government {
    #[serde(rename = "$government_corporate;")]
    Corporate,
    #[serde(rename = "$government_cooperative;")]
    Cooperative,
    #[serde(rename = "$government_patronage;")]
    Patronage,
    // TODO: add more as needed
}

// for factions
#[derive(Debug,Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GovernmentFaction {
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

#[derive(Debug,Deserialize, Serialize)]
pub enum Security {
    #[serde(rename = "$system_security_medium;")]
    Medium,
    #[serde(rename = "$system_security_low;")]
    Low,
    #[serde(rename = "$system_security_high;")]
    High,
    #[serde(rename = "$system_security_anarchy;")]
    Anarchy,
    #[serde(rename = "$system_security_lawless;")]
    Lawless,
}
    
#[derive(Debug,Deserialize, Serialize)]
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
}

#[derive(Debug,Deserialize, Serialize)]
pub struct System {
    pub eddb_id:i64,
    pub name:String,
    pub population: i64,
    pub factions:Vec<Faction>,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct Faction {
    pub name:String,
    pub government:GovernmentFaction,
    pub allegiance:Allegiance,
    pub evolution:Vec<FactionData>,
}

// faction data (for in a specific system)
#[derive(Debug,Deserialize, Serialize)]
pub struct FactionData {
    pub date:DateTime<Utc>,
    pub influence:f64,
    pub pending_states:Vec<FactionState>,
    pub recovering_states:Vec<FactionState>,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct FactionState {
    pub state:State,
    pub trend:i64,
}
