use serde::{de::IgnoredAny, Deserialize};

#[derive(Debug, Deserialize)]
pub struct MythicPlusCharactersResponse {
    pub rankings: Rankings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub ranked_characters: Vec<IgnoredAny>,
    pub ui: Ui,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    pub page_size: u32,
    pub last_page: u32,
}
