# Pumpkin-Duels

Pumpkin-Duels is a high-performance 1v1 practice duel plugin built in Rust for the [Pumpkin](https://pumpkinmc.org) Minecraft server. It compiles directly to WebAssembly (`wasm32-wasip2`), executing safely inside the server runtime at near-native speeds.

## Features

- **Isolated World Instancing**: Duplicates arena template directories into isolated match instances (`worlds/duel_<match_id>`) for concurrent matches. Cleans up and deletes temporary directories immediately upon match termination to prevent world data accumulation.
- **Matchmaking and Direct Challenges**: Supports direct player challenges with interactive clickable chat prompts (`[ACCEPT]` and `[DECLINE]`) and automated queue matchmaking per kit.
- **Chest GUI Kit Selector**: Virtual inventory interface allowing players to select kits when challenging opponents or entering the queue.
- **In-Game Kit Management**: Capture your current inventory, armor, and offhand items directly into reusable kits with `/dueladmin kit save <name>`.
- **ELO and Leaderboards**: Standardized ELO rating system tracking player ratings, wins, losses, win rates, and win streaks. View stats with `/duel stats` and top players with `/duel top`.
- **Sound and Visual Feedback**: Synchronized title countdowns, sound effects for hits, countdown ticks, victories, and defeats.
- **Config Hot-Reconciliation**: TOML configuration engine that preserves existing comments and structure while automatically migrating newly added settings.
- **Embedded SQLite Storage**: Built-in SQLite database storing player stats, kits, arena templates, and match history.

## Documentation

Full documentation is available in the `docs/` directory and can be run locally using VitePress:

```bash
npm install
npm run docs:dev
```

To build the static documentation bundle:

```bash
npm run docs:build
```

Documentation topics:
- Getting Started: `docs/getting-started.md`
- Commands and Permissions: `docs/commands.md`
- Configuration Guide: `docs/configuration.md`
- Arena Setup: `docs/arenas.md`
- Kit Management: `docs/kits.md`
- Architecture and Internals: `docs/architecture.md`

## Installation

1. Ensure your Pumpkin server supports WebAssembly plugins (`wasm32-wasip2`).
2. Download the latest `pumpkin_duels.wasm` binary from the GitHub Releases page under the `dev` tag.
3. Place `pumpkin_duels.wasm` inside the `plugins/` directory of your Pumpkin server.
4. Start the server. The plugin will generate its configuration and database files inside `plugins/pumpkin-duels/`.

## Quick Setup

### 1. Set Lobby Location
Stand at your desired duel lobby spawn and run:
```
/dueladmin setlobby
```

### 2. Create an Arena
1. Create a void world template with a starter platform:
   ```
   /arena create classic minecraft:overworld
   ```
2. Teleport to the template world:
   ```
   /arena tp classic
   ```
3. Set the spawn points:
   ```
   /arena setspawn1 classic
   /arena setspawn2 classic
   ```
4. Set spectator position and boundary radius:
   ```
   /arena setspectator classic
   /arena setbounds classic 50.0
   ```
5. Return to the lobby:
   ```
   /arena leave
   ```

### 3. Create a Kit
1. Arrange items, armor, and offhand in your player inventory.
2. Save the kit:
   ```
   /dueladmin kit save nodebuff
   ```

## Commands Summary

### Player Commands
- `/duel <player>`: Send a duel challenge with kit selection GUI.
- `/duel accept [player]`: Accept an incoming duel challenge.
- `/duel decline [player]`: Decline an incoming duel challenge.
- `/queue [kit]`: Enter the matchmaking queue.
- `/leavequeue`: Leave the matchmaking queue.
- `/duel stats [player]`: View duel statistics and ELO rating.
- `/duel top`: View top players on the leaderboard.

### Administrator Commands
- `/arena create <name> [dimension]`: Create a void arena template.
- `/arena setspawn1 <name>`: Set player 1 spawn point.
- `/arena setspawn2 <name>`: Set player 2 spawn point.
- `/arena setspectator <name>`: Set spectator spawn point.
- `/arena setbounds <name> [radius]`: Set arena boundary radius.
- `/arena tp <name>`: Teleport to arena template.
- `/arena leave`: Teleport back to lobby.
- `/arena list`: List configured arenas.
- `/arena delete <name>`: Remove an arena template.
- `/dueladmin kit save <name>`: Save current inventory as a kit.
- `/dueladmin kit list`: List registered kits.
- `/dueladmin kit delete <name>`: Delete a kit.
- `/dueladmin setlobby`: Set the global duel lobby spawn.
- `/dueladmin reload`: Hot-reload configuration files.

## Building from Source

### Prerequisites
- Rust stable toolchain with `wasm32-wasip2` target:
  ```bash
  rustup target add wasm32-wasip2
  ```
- WASI SDK 24.0+ (required by `rusqlite` C amalgamation compilation for WASI):
  ```bash
  mkdir -p /tmp/wasi-sdk
  curl -sSL "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-24/wasi-sdk-24.0-x86_64-linux.tar.gz" | tar -xz -C /tmp/wasi-sdk --strip-components=1
  export WASI_SDK_PATH=/tmp/wasi-sdk
  export CC_wasm32_wasip2=/tmp/wasi-sdk/bin/clang
  ```

### Compilation
Run Cargo to build the release WebAssembly binary:
```bash
cargo build --release --target wasm32-wasip2
```

The resulting binary will be located at:
```
target/wasm32-wasip2/release/pumpkin_duels.wasm
```

## Continuous Integration

Every push to the `main` branch triggers a GitHub Actions workflow (`.github/workflows/build.yml`) that:
1. Configures the Rust toolchain and WASI SDK.
2. Compiles `pumpkin_duels.wasm` in release mode.
3. Updates the rolling `dev` release tag, replacing previous release assets so builds do not accumulate.

## License

This project is licensed under the MIT License.
