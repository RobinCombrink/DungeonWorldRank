use anyhow::Result;
use api_client::ApiClient;
use clap::{Args, Parser};
use log::LevelFilter;
use std::io::Write;

use crate::{character::Region, expansion::Expansion};

mod api_client;
mod character;
mod expansion;
mod from_response;
mod mythic_plus_characters;
mod mythic_plus_dungeons;

#[derive(Parser, Debug)]
enum Command {
    #[clap(
        visible_alias = "c",
        visible_alias = "char",
        visible_alias = "character"
    )]
    CharacterRank(CharacterRankArgs),
    HistoricalDungeonRank(HistoricalDungeonRankArgs),
}

#[derive(Args, Debug)]
#[clap(
    name = "What's My World Rank",
    about = "Get the rank for a specific character for a season from an expansion"
)]
struct CharacterRankArgs {
    #[clap(short = 'n', long, default_value = "meleekick")]
    name: String,
    #[clap(short = 'r', long, default_value = "ravencrest")]
    realm: String,
    #[clap(long, default_value = "eu")]
    region: character::Region,
    #[clap(long, default_value = "tww")]
    expansion: Expansion,
}

impl Default for CharacterRankArgs {
    fn default() -> Self {
        Self {
            name: "meleekick".to_owned(),
            realm: "ravencrest".to_owned(),
            region: Region::EU,
            expansion: Expansion::default(),
        }
    }
}

#[derive(Args, Debug)]
#[clap(about = "Get the world rank of a dungeon when it was completed")]
struct HistoricalDungeonRankArgs {
    #[clap(short = None, long, default_value = "18670877")]
    run_id: String,
}

#[derive(Parser, Debug)]
#[clap(name = "Dotfiles")]
#[command(version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(
        global = true,
        long("debug"),
        help = "Display an execution plan instead of executing on the plan"
    )]
    debug: bool,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    setup_logging(args.debug);

    println!("Calculating world rank");

    let client = ApiClient::new();

    match args.command {
        Some(command) => match command {
            Command::CharacterRank(args) => get_character_rank(client, args).await,
            Command::HistoricalDungeonRank(args) => {
                let historical_dungeon_rank =
                    client.get_historical_dungeon_rank(args.run_id).await?;
                println!("historical dungeon rank: {}", historical_dungeon_rank);
                Ok(())
            }
        },
        None => get_character_rank(client, CharacterRankArgs::default()).await,
    }
}

async fn get_character_rank(client: ApiClient, args: CharacterRankArgs) -> Result<()> {
    let character = client
        .get_character(args.region, args.realm, args.name)
        .await?;
    let world_rank = character.get_world_rank(client, args.expansion).await?;
    println!("{}", world_rank);
    Ok(())
}

fn setup_logging(debug: bool) {
    let level_filter = match debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Warn,
    };

    env_logger::builder()
        .filter_level(level_filter)
        .parse_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
