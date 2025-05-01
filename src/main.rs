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
    let total_class_characters = client.get_total_characters_by_class(&character.class)?;

    let world_rank = character.mythic_plus_ranks.overall.world;
    let world_percentile = world_rank as f64 / total_characters as f64 * 100 as f64;

    let class_rank = character.mythic_plus_ranks.class.world;
    let class_percetile = class_rank as f64 / total_class_characters as f64 * 100 as f64;

    println!(
        "\n{character}-{realm} - {region}\n\nWorld Rank: {world_rank}/{total_characters}\nWorld Percentile: {:.03}%\n\nClass Rank: {class_rank}/{total_class_characters}\nClass Percentile: {:.03}%",
        world_percentile,
        class_percetile,
        character = character.name,
        realm = character.realm,
        region = character.region,
    );
    return Ok(());
}
