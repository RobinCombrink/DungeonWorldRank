use std::str::FromStr;

use anyhow::Result;
use api_client::ApiClient;
use from_response::FromResponse;
use mythic_plus_characters::MythicPlusCharactersResponse;
use reqwest::{blocking::ClientBuilder, Url};

mod api_client;
mod character;
mod from_response;
mod mythic_plus_characters;

fn main() -> Result<()> {
    let client = ApiClient::new();

    let character = client.get_character(character::Region::EU, "ravencrest", "meleekick")?;
    let total_characters = client.get_total_characters()?;

    let world_rank = character.mythic_plus_ranks.overall.world;
    let percentile = world_rank as f64 / total_characters as f64 * 100 as f64;

    println!(
        "Character: {character}\nRealm: {realm}\nRegion: {region}\nWorld Rank: {world_rank}\nTotal Char: {total_characters}\nPercentile: {:.03}%",
        percentile,
        character = character.name,
        realm = character.realm,
        region = character.region,
    );
    return Ok(());
}
