# DungeonWorldRank

Fetches WoW Mythic+ world rankings for a character.

## What it does

CLI tool that queries the Raider.io API to retrieve a World of Warcraft character's Mythic+ dungeon rankings for a given season. Displays world rank, score, and per-dungeon breakdowns.

## Naming

Three different names are in play: the GitHub repo is `DungeonWorldRank`, the Cargo package
is `what-is-my-world-rank`, and the compiled binary is `wmwr` (`[[bin]]` in `Cargo.toml`).
Look for `wmwr` in `target/`, not the repo or package name.

## Running locally

```bash
cargo run -- --character "Name" --realm "Realm" --region "eu"
```

## Known limitations

Some calls (`get_dungeon_run`, `get_dungeon_runs`) rely on Raider.io internal/website
endpoints rather than the documented public API: the season slug `season-tww-2` is
hardcoded (not derived from `get_current_season`) and `get_dungeon_runs` sends a spoofed
browser `referrer` header. Expect silent breakage each new WoW season/expansion until the
slug is updated.

## Tech Stack

- **Rust** with Tokio async runtime
- **reqwest** for HTTP
- **clap** for CLI arguments
- **serde** for JSON deserialization
