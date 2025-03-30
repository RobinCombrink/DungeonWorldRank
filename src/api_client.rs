use std::str::FromStr;

use anyhow::Result;
use reqwest::{
    blocking::{Client, ClientBuilder},
    Url,
};

use crate::{
    character::{CharacterResponse, Region},
    from_response::FromResponse,
    mythic_plus_characters::MythicPlusCharactersResponse,
};

pub struct ApiClient {
    client: Client,
    base_url: Url,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: ClientBuilder::new().build().unwrap(),
            base_url: Url::from_str("https://raider.io/")
                .unwrap()
                .join("api/")
                .unwrap(),
        }
    }

    pub fn get_character(
        &self,
        region: Region,
        realm: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<CharacterResponse> {
        let url = self
            .base_url
            .join("v1/")?
            .join("characters/")?
            .join("profile")?;
        let response = self
            .client
            .get(url)
            .query(&[
                ("region", format!("{region}").as_ref()),
                ("realm", realm.into().as_ref()),
                ("name", name.into().as_ref()),
                ("fields", "mythic_plus_ranks"),
            ])
            .send()?;
        CharacterResponse::from_response(response)
    }

    pub fn get_total_characters(&self) -> Result<u32> {
        let first_characters_page = self.get_mythic_plus_characters(0)?;

        let last_characters_page =
            self.get_mythic_plus_characters(first_characters_page.rankings.ui.last_page)?;

        let total_characters_from_pages = last_characters_page.rankings.ui.page_size
            * (last_characters_page.rankings.ui.last_page - 1);
        let total_characters = total_characters_from_pages
            + last_characters_page.rankings.ranked_characters.len() as u32;

        Ok(total_characters)
    }

    pub fn get_mythic_plus_characters(&self, page: u32) -> Result<MythicPlusCharactersResponse> {
        let url = self.base_url.join("mythic-plus/rankings/characters")?;

        let response = self
            .client
            .get(url)
            .query(&[
                ("region", "world"),
                ("season", "season-tww-2"),
                ("class", "all"),
                ("role", "all"),
                ("page", &format!("{:?}", page)),
            ])
            .send()?;

        let url = response.url();
        println!("{}", url);

        MythicPlusCharactersResponse::from_response(response)
    }
}
