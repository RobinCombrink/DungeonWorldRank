use std::fmt::Display;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, ValueEnum, Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    CN,
    EU,
    US,
    KR,
    TW,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::CN => f.write_str("cn"),
            Region::EU => f.write_str("eu"),
            Region::US => f.write_str("us"),
            Region::KR => f.write_str("kr"),
            Region::TW => f.write_str("tw"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CharacterResponse {
    pub name: String,
    pub race: String,
    pub class: String,
    pub active_spec_name: String,
    pub active_spec_role: String,
    pub gender: String,
    pub faction: String,
    pub achievement_points: u32,
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    #[serde(rename = "last_crawled_at")]
    pub last_crawled_at: String, // ISO date-time string
    pub profile_url: String,
    pub profile_banner: String,
    #[serde(rename = "mythic_plus_ranks")]
    pub mythic_plus_ranks: MythicPlusRanks,
}

#[derive(Debug, Deserialize)]
pub struct MythicPlusRanks {
    pub overall: RankDetails,
    pub class: RankDetails,
    pub dps: RankDetails,
    pub class_dps: RankDetails,
}

#[derive(Debug, Deserialize)]
pub struct RankDetails {
    pub world: u32,
    pub region: u32,
    pub realm: u32,
}
