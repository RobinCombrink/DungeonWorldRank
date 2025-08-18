use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, ValueEnum, Debug, Deserialize, Clone, Serialize)]
pub(crate) enum Expansion {
    TWW,
    DF,
    SL,
    BFA,
    L,
}

impl Default for Expansion {
    fn default() -> Self {
        Expansion::TWW
    }
}

impl Into<u8> for Expansion {
    fn into(self) -> u8 {
        match self {
            Expansion::TWW => 10,
            Expansion::DF => 9,
            Expansion::SL => 8,
            Expansion::BFA => 7,
            Expansion::L => 6,
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
