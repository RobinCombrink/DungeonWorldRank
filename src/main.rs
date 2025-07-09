use anyhow::Result;
use api_client::ApiClient;
use clap::{Args, Parser};
use log::LevelFilter;
use std::io::Write;

mod api_client;
mod character;
mod from_response;
mod mythic_plus_characters;
mod mythic_plus_dungeons;

#[derive(Parser, Debug)]
enum Command {
    CharacterRank(CharacterRankArgs),
    HistoricalDungeonRank(HistoricalDungeonRankArgs),
}

#[derive(Args, Debug)]
#[clap(name = "What's My World Rank")]
struct CharacterRankArgs {
    #[clap(short = 'n', long, default_value = "meleekick")]
    name: String,
    #[clap(short = 'r', long, default_value = "ravencrest")]
    realm: String,
    #[clap(long, default_value = "eu")]
    region: character::Region,
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
    command: Command,
    #[arg(
        global = true,
        long("debug"),
        help = "Display an execution plan instead of executing on the plan"
    )]
    debug: bool,
}
fn main() -> Result<()> {
    let args = Arguments::parse();
    setup_logging(args.debug);

    println!("Calculating world rank");

    let client = ApiClient::new();

    match args.command {
        Command::CharacterRank(args) => {
            let character = client.get_character(args.region, args.realm, args.name)?;

            let world_rank = character.get_world_rank(client)?;

            println!("{}", world_rank);
        }
        Command::HistoricalDungeonRank(args) => {
            let historical_dungeon_rank = client.get_historical_dungeon_rank(args.run_id)?;
            println!("historical dungeon rank: {}", historical_dungeon_rank)
        },
    };

    return Ok(());
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
