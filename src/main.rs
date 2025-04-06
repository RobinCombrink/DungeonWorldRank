use anyhow::Result;
use api_client::ApiClient;
use clap::Parser;

mod api_client;
mod character;
mod from_response;
mod mythic_plus_characters;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(short = 'n', long, default_value = "meleekick")]
    name: String,
    #[clap(short = 'r', long, default_value = "ravencrest")]
    realm: String,
    #[clap(long, default_value = "eu")]
    region: character::Region,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let client = ApiClient::new();

    let character = client.get_character(args.region, args.realm, args.name)?;
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
