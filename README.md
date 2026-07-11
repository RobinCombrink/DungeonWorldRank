# DungeonWorldRank

Fetches WoW Mythic+ world rankings for a character.

## What it does

CLI tool that queries the Raider.io API to retrieve a World of Warcraft character's Mythic+ dungeon rankings for a given season. Displays world rank, score, and per-dungeon breakdowns.

## Running locally

```bash
cargo run -- --character "Name" --realm "Realm" --region "eu"
```

## Tech Stack

- **Rust** with Tokio async runtime
- **reqwest** for HTTP
- **clap** for CLI arguments
- **serde** for JSON deserialization
