use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum Expansion {
    TheWarWithin,
    Dragonflight,
    Shadowlands,
    BattleForAzeroth,
    Legion,
}
impl Into<u8> for Expansion {
    fn into(self) -> u8 {
        match self {
            Expansion::TheWarWithin => 10,
            Expansion::Dragonflight => 9,
            Expansion::Shadowlands => 8,
            Expansion::BattleForAzeroth => 7,
            Expansion::Legion => 6,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SeasonDateInfo {
    pub us: String,
    pub eu: String,
    pub tw: String,
    pub kr: String,
    pub cn: String,
}

#[derive(Debug, Deserialize)]
pub struct Season {
    pub slug: String,
    pub name: String,
    pub blizzard_season_id: u32,
    pub is_main_season: bool,
    pub short_name: String,
    pub seasonal_affix: Option<String>,
    pub starts: SeasonDateInfo,
    pub ends: SeasonDateInfo,
}

#[derive(Debug, Deserialize)]
pub struct SeasonsResponse {
    pub seasons: Vec<Season>,
}