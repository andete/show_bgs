
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
    
}
