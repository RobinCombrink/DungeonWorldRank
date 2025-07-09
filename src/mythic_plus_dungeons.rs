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
    pub keystone_team_id: u64,
    pub season: String,
    pub status: String,
    pub dungeon: GroupDungeon,
    pub keystone_run_id: u64,
    pub mythic_level: u32,
    pub clear_time_ms: u64,
    pub keystone_time_ms: u64,
    pub completed_at: DateTime<Utc>,
    pub num_chests: u32,
    pub time_remaining_ms: u64,
    pub logged_run_id: Option<u64>,
    pub videos: Vec<serde_json::Value>,
    pub weekly_modifiers: Vec<GroupModifier>,
    pub num_modifiers_active: u32,
    pub faction: String,
    pub deleted_at: Option<serde_json::Value>,
    pub keystone_platoon_id: Option<serde_json::Value>,
    pub platoon: Option<serde_json::Value>,
    pub roster: Vec<GroupRosterEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct GroupRosterEntry {
    pub character: GroupCharacter,
    pub old_character: Option<serde_json::Value>,
    pub is_transfer: bool,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupCharacter {
    pub id: u64,
    #[serde(rename = "persona_id")]
    pub persona_id: u64,
    pub name: String,
    pub class: GroupClass,
    pub race: GroupRace,
    pub faction: String,
    pub level: u32,
    pub spec: GroupSpec,
    pub path: String,
    pub realm: GroupRealm,
    pub region: GroupRegion,
    pub stream: Option<serde_json::Value>,
    #[serde(rename = "recruitmentProfiles")]
    pub recruitment_profiles: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupClass {
    pub id: u32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRace {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub faction: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSpec {
    pub id: u32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRealm {
    pub id: u32,
    #[serde(rename = "connectedRealmId")]
    pub connected_realm_id: u32,
    #[serde(rename = "wowRealmId")]
    pub wow_realm_id: u32,
    #[serde(rename = "wowConnectedRealmId")]
    pub wow_connected_realm_id: u32,
    pub name: String,
    #[serde(rename = "altName")]
    pub alt_name: Option<String>,
    pub slug: String,
    #[serde(rename = "altSlug")]
    pub alt_slug: Option<String>,
    pub locale: String,
    #[serde(rename = "isConnected")]
    pub is_connected: bool,
    #[serde(rename = "realmType")]
    pub realm_type: String,
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
    pub faction: String,
    pub realm: String,
    pub period: u32,
    pub recent: bool,
    pub last_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeystoneRunResponse {
    #[serde(rename = "keystoneRun")]
    pub keystone_run: KeystoneRun,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeystoneRun {
    pub season: String,
    pub status: String,
    pub dungeon: GroupDungeon,
    pub keystone_run_id: u64,
    pub mythic_level: u32,
    pub clear_time_ms: u64,
    pub keystone_time_ms: u64,
    pub completed_at: DateTime<Utc>,
    pub num_chests: u32,
    pub time_remaining_ms: u64,
    pub logged_run_id: Option<u64>,
    pub videos: Vec<serde_json::Value>,
    pub weekly_modifiers: Vec<serde_json::Value>,
    pub num_modifiers_active: u32,
    pub faction: String,
    pub deleted_at: Option<serde_json::Value>,
    pub score: u32,
    pub logged_details: Option<serde_json::Value>,
    pub replay_limit: u32,
    pub keystone_team_id: u64,
    pub keystone_platoon_id: Option<serde_json::Value>,
    #[serde(rename = "isTournamentProfile")]
    pub is_tournament_profile: bool,
    pub can_manage_own_videos: bool,
    pub can_manage_others_videos: bool,
    #[serde(rename = "loggedSources")]
    pub logged_sources: Vec<serde_json::Value>,
    #[serde(rename = "isPatron")]
    pub is_patron: bool,
    #[serde(rename = "runPrivacyMode")]
    pub run_privacy_mode: String,
    pub can_view_private_details: bool,
    pub is_viewing_private_details: bool,
}
