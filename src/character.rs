use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use futures::join;
use serde::{Deserialize, Serialize};

use crate::{api_client::ApiClient, expansion::Expansion};

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
    pub class: Class,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    DeathKnight,
    DemonHunter,
    Druid,
    Evoker,
    Hunter,
    Mage,
    Monk,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:#?}", self))
    }
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

pub struct CharacterRank {
    pub name: String,
    pub realm: String,
    pub region: String,
    pub world_rank: u32,
    pub class_rank: u32,
}

impl From<&CharacterResponse> for CharacterRank {
    fn from(value: &CharacterResponse) -> Self {
        CharacterRank {
            name: value.name.clone(),
            realm: value.realm.clone(),
            region: value.region.clone(),
            world_rank: value.mythic_plus_ranks.overall.world,
            class_rank: value.mythic_plus_ranks.class.world,
        }
    }
}

impl Display for CharacterWorldRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "\n{name}-{realm} - {region}\n\nWorld Rank: {world_rank}/{total_characters}\nWorld Percentile: {world_percentile:.03}%\n\nClass Rank: {class_rank}/{total_class_characters}\nClass Percentile: {class_percentile:.03}%",
        name = self.character_rank.name,
        realm = self.character_rank.realm,
        region = self.character_rank.region,
        world_rank = self.character_rank.world_rank,
        total_characters = self.world_details.total_characters,
        world_percentile = self.calculate_world_percentile(),
        class_rank = self.character_rank.class_rank,
        total_class_characters = self.world_details.total_class_characters,
        class_percentile = self.calcualte_class_percentile(),
    )
    }
}

pub struct CharacterWorldRank {
    pub character_rank: CharacterRank,
    pub world_details: WorldDetails,
}
impl CharacterWorldRank {
    fn new(character_rank: CharacterRank, world_details: WorldDetails) -> Self {
        Self {
            character_rank,
            world_details,
        }
    }
    fn calculate_world_percentile(&self) -> f64 {
        self.character_rank.world_rank as f64 / self.world_details.total_characters as f64
            * 100 as f64
    }
    fn calcualte_class_percentile(&self) -> f64 {
        self.character_rank.class_rank as f64 / self.world_details.total_class_characters as f64
            * 100 as f64
    }
}

pub struct WorldDetails {
    pub total_characters: u32,
    pub total_class_characters: u32,
}

impl WorldDetails {
    fn new(total_characters: u32, total_class_characters: u32) -> Self {
        Self {
            total_characters,
            total_class_characters,
        }
    }
}

impl CharacterResponse {
    pub async fn get_world_rank(
        self,
        client: ApiClient,
        expansion: Expansion,
    ) -> Result<CharacterWorldRank> {
        let character_rank = CharacterRank::from(&self);
        let season = client.get_current_season(expansion).await?;

        let (total_world_characters, total_class_characters) = join!(
            client.get_total_characters(&season),
            client.get_total_characters_by_class(self.class, &season)
        );

        let world_details = WorldDetails::new(total_world_characters?, total_class_characters?);

        Ok(CharacterWorldRank::new(character_rank, world_details))
    }
}
