use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicPlusProgressResponse {
    pub characterMythicPlusProgress: CharacterMythicPlusProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicPlusProgress {
    pub season_slug: String,
    pub season: String,
    pub mythic_plus_scores: MythicPlusScores,
    pub mythic_plus_ranks: MythicPlusRanks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicPlusScores {
    pub all: MythicPlusScoreAll,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicPlusScoreAll {
    pub score: f64,
    pub score_color: String,
    pub runs: Vec<Run>,
    pub alternate_runs: Vec<serde_json::Value>,
    pub raw_runs: Vec<RawRun>,
    pub raw_alternate_runs: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub zone_id: i64,
    pub keystone_run_id: i64,
    pub mythic_level: i64,
    pub clear_time_ms: i64,
    pub score: f64,
    pub period: i64,
    pub affixes: Vec<i64>,
    pub logged_run_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRun {
    pub id: i64,
    pub members_checksum: i64,
    pub region_id: i64,
    pub realm_id: i64,
    pub keystone_team_id: i64,
    pub keystone_platoon_id: Option<i64>,
    pub zone_id: i64,
    pub faction: String,
    pub mythic_level: i64,
    pub clear_time_ms: i64,
    pub par_time_ms: i64,
    pub period: i64,
    pub completed_at: String,
    pub affix_0_id: i64,
    pub affix_1_id: i64,
    pub affix_2_id: i64,
    pub affix_3_id: i64,
    pub logged_run_id: i64,
    pub num_members: i64,
    pub group_comp_bitmask: i64,
    pub item_level_min: i64,
    pub item_level_max: i64,
    pub item_level_mean: f64,
    pub bnet_mythic_rating: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicPlusRanks {
    pub overall: Rank,
    #[serde(rename = "class")]
    pub class_rank: Rank,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rank {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}
