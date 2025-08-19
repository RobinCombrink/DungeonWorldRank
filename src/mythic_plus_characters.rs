use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicPlusCharactersResponse {
    pub rankings: Rankings,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub ranked_characters: Vec<RankedCharacter>,
    pub ui: Ui,
    pub region: Region,
    pub realm: Option<Realm>, // realm is null in this example so we use Option
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedCharacter {
    pub rank: u32,
    pub score: f64,
    pub score_color: String,
    pub runs: Vec<Run>,
    pub character: Character,
    pub guild: Option<Guild>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub zone_id: u32,
    pub keystone_run_id: u64,
    pub clear_time_ms: u64,
    pub mythic_level: u32,
    pub score: f64,
    pub period: u32,
    pub affixes: Vec<Value>,
    pub logged_run_id: u64,
    pub num_chests: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: u64,
    #[serde(rename = "persona_id")]
    pub persona_id: u64,
    pub name: String,
    pub class: Class,
    pub race: Race,
    pub faction: String,
    pub level: u32,
    pub path: String,
    pub realm: Realm,
    pub region: Region,
    // The stream field is null here; if its structure varies, you might eventually define a proper type.
    pub stream: Option<Value>,
    // recruitmentProfiles is an empty array in this sample,
    // so we use a vector of generic Value.
    pub recruitment_profiles: Vec<Value>,
    pub talent_loadout_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: u32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub faction: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Spec {
    pub id: u32,
    pub name: String,
    pub slug: String,
    #[serde(rename = "class_id")]
    pub class_id: u32,
    pub role: String,
    #[serde(rename = "is_melee")]
    pub is_melee: bool,
    pub patch: String,
    pub ordinal: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realm {
    pub id: u32,
    pub connected_realm_id: u32,
    pub wow_realm_id: Option<u32>,
    pub wow_connected_realm_id: u32,
    pub name: String,
    pub alt_name: Option<String>,
    pub slug: String,
    pub alt_slug: Option<String>,
    pub locale: String,
    pub is_connected: bool,
    pub realm_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub slug: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    pub region: String,
    pub season: String,
    pub class: String,
    pub role: String,
    pub page: u32,
    pub page_size: u32,
    pub last_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: u64,
    pub name: String,
    pub faction: String,
    pub realm: Realm,
    pub region: Region,
    pub path: String,
}