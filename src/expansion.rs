use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, ValueEnum, Debug, Deserialize, Clone, Serialize, Default)]
pub(crate) enum Expansion {
    #[default]
    Tww,
    DF,
    SL,
    Bfa,
    L,
}

impl From<Expansion> for u8 {
    fn from(expansion: Expansion) -> Self {
        match expansion {
            Expansion::Tww => 10,
            Expansion::DF => 9,
            Expansion::SL => 8,
            Expansion::Bfa => 7,
            Expansion::L => 6,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Season {
    pub slug: String,
    pub is_main_season: bool,
}

#[derive(Debug, Deserialize)]
pub struct SeasonsResponse {
    pub seasons: Vec<Season>,
}
