
#[derive(Debug,Deserialize, Serialize)]
pub enum Allegiance {
    #[serde(rename = "independent")]
    Independent,
    #[serde(rename = "federation")]
    Federation,
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "empire")]
    Empire,
}

#[derive(Debug,Deserialize, Serialize)]
pub enum State {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "expansion")]
    Expansion,
    #[serde(rename = "war")]
    War,
    #[serde(rename = "civilwar")]
    CivilWar,
    #[serde(rename = "election")]
    Election,
    #[serde(rename = "boom")]
    Boom,
    #[serde(rename = "civilunrest")]
    CivilUnrest,
    #[serde(rename = "famine")]
    Famine,
    #[serde(rename = "outbreak")]
    Outbreak,
    #[serde(rename = "lockdown")]
    Lockdown,
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "retreat")]
    Retreat,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct System {
    
}
