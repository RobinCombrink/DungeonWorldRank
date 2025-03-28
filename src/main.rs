use std::str::FromStr;

use anyhow::Result;
use from_response::FromResponse;
use mythic_plus_characters::MythicPlusCharactersResponse;
use reqwest::{blocking::ClientBuilder, Url};

mod from_response;
mod mythic_plus_characters;

fn main() -> Result<()> {
    let client = ClientBuilder::new().build().unwrap();
    let base_url = Url::from_str("https://raider.io/")?.join("api/")?;
    println!("{:#?}", base_url.to_string());
    let mythic_plus_characters_url = base_url.join("mythic-plus/rankings/characters")?;
    println!("{:#?}", mythic_plus_characters_url.to_string());
    let response = client
        .get(mythic_plus_characters_url)
        .query(&[
            ("region", "world"),
            ("season", "season-tww-2"),
            ("class", "all"),
            ("role", "all"),
            ("page", "30922"),
        ])
        .send()
        .unwrap();

    let response = MythicPlusCharactersResponse::from_response(response).unwrap();

    println!("{:#?}", response);
    return Ok(());
}
