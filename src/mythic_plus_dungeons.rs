use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicPlusGroupsResponse {
    pub rankings: GroupsRankings,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsRankings {
    pub ranked_groups: Vec<RankedGroup>,
    pub ui: GroupsUi,
    pub region: GroupRegion,
    pub dungeon: GroupDungeon,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedGroup {
    pub rank: u32,
    pub score: f64,
    pub run: GroupRun,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRun {
    #[serde(rename = "keystone_team_id")]
    pub keystone_team_id: u64,
    pub season: String,
    pub status: String,
    pub dungeon: GroupDungeon,
    #[serde(rename = "keystone_run_id")]
    pub keystone_run_id: u64,
    #[serde(rename = "mythic_level")]
    pub mythic_level: u32,
    #[serde(rename = "clear_time_ms")]
    pub clear_time_ms: u64,
    #[serde(rename = "keystone_time_ms")]
    pub keystone_time_ms: u64,
    #[serde(rename = "completed_at")]
    pub completed_at: DateTime<Utc>,
    #[serde(rename = "num_chests")]
    pub num_chests: u32,
    #[serde(rename = "time_remaining_ms")]
    pub time_remaining_ms: i128,
    pub logged_run_id: Option<u64>,
    pub videos: Vec<serde_json::Value>,
    #[serde(rename = "weekly_modifiers")]
    pub weekly_modifiers: Vec<GroupModifier>,
    #[serde(rename = "num_modifiers_active")]
    pub num_modifiers_active: u32,
    pub faction: String,
    pub deleted_at: Option<serde_json::Value>,
    #[serde(rename = " keystone_platoon_id")]
    pub keystone_platoon_id: Option<serde_json::Value>,
    pub platoon: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupDungeon {
    #[serde(rename = "type")]
    pub dungeon_type: String,
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub slug: String,
    pub expansion_id: u32,
    pub icon_url: String,
    pub patch: String,
    #[serde(rename = "wowInstanceId")]
    pub wow_instance_id: u32,
    pub map_challenge_mode_id: u32,
    pub keystone_timer_ms: u64,
    pub num_bosses: u32,
    pub group_finder_activity_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupModifier {
    pub id: u32,
    pub icon: String,
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRegion {
    pub name: String,
    pub slug: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsUi {
    pub region: String,
    pub season: String,
    pub dungeon: String,
    pub strict: bool,
    pub page: u32,
    pub limit: u32,
    pub min_mythic_level: u32,
    pub max_mythic_level: u32,
    pub event_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    pub period: u32,
    pub recent: bool,
    pub last_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeystoneRunResponse {
    pub keystone_run: KeystoneRun,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeystoneRun {
    pub season: String,
    pub status: String,
    pub dungeon: GroupDungeon,
    #[serde(rename = "keystone_run_id")]
    pub keystone_run_id: u64,
    #[serde(rename = "mythic_level")]
    pub mythic_level: u32,
    #[serde(rename = "clear_time_ms")]
    pub clear_time_ms: u64,
    #[serde(rename = "keystone_time_ms")]
    pub keystone_time_ms: u64,
    #[serde(rename = "completed_at")]
    pub completed_at: DateTime<Utc>,
    #[serde(rename = "num_chests")]
    pub num_chests: u32,
    #[serde(rename = "time_remaining_ms")]
    pub time_remaining_ms: i128,
    pub logged_run_id: Option<u64>,
    pub videos: Vec<serde_json::Value>,
    #[serde(rename = "weekly_modifiers")]
    pub weekly_modifiers: Vec<serde_json::Value>,
    #[serde(rename = "num_modifiers_active")]
    pub num_modifiers_active: u32,
    pub faction: String,
    pub deleted_at: Option<serde_json::Value>,
    pub score: f64,
    #[serde(rename = "replay_limit")]
    pub replay_limit: u32,
    #[serde(rename = "keystone_team_id")]
    pub keystone_team_id: u64,
    #[serde(rename = "keystone_platoon_id")]
    pub keystone_platoon_id: Option<serde_json::Value>,
    pub is_tournament_profile: bool,
    pub can_manage_own_videos: bool,
    pub can_manage_others_videos: bool,
    pub logged_sources: Vec<serde_json::Value>,
    pub is_patron: bool,
    pub run_privacy_mode: String,
    pub can_view_private_details: bool,
    pub is_viewing_private_details: bool,
}
