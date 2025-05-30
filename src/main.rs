use anyhow::Result;
use api_client::ApiClient;
use clap::Parser;
use log::LevelFilter;
use std::io::Write;

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
    #[clap(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    setup_logging(args.debug);

    let client = ApiClient::new();

    let character = client.get_character(args.region, args.realm, args.name)?;

    let world_rank = character.get_world_rank(client)?;

    println!("{}", world_rank);

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
