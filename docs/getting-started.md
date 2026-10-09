# Getting Started

This guide explains how to install, configure, and run Pumpkin-Duels on your Pumpkin server.

## Server Version Requirements

| Component | Minimum Required Version | Notes |
| :--- | :--- | :--- |
| **Pumpkin Server** | `0.2.0` (or `0.2.0+26.3-26.51`+) | Requires WebAssembly WASI (`wasm32-wasip2`) plugin engine. |
| **Minecraft: Java Edition** | `1.21.4`+ (Protocol `777`) | Required for modern native client dialog screens and ActionButtons. |
| **Minecraft: Bedrock Edition** | `1.26.51`+ (Protocol `2193`) | Native cross-play ModalForms and SimpleForms support. |
| **Plugin Target Architecture** | `wasm32-wasip2` | WASI Preview 2 component specification. |
| **Operating System** | Linux (x86_64, aarch64), macOS, Windows | Server host runtime environments. |

---

## Installation

### 1. Download the Plugin

Download the latest compiled WebAssembly binary:

- **Development Release**: Download `pumpkin_duels.wasm` from the GitHub Releases page under the `dev` tag.
- **Manual Compilation**: If building from source, refer to the [Compilation Guide](#building-from-source) below.

### 2. Place Binary in Plugins Directory

Copy `pumpkin_duels.wasm` into the `plugins/` directory of your Pumpkin server:

```bash
mkdir -p plugins
cp pumpkin_duels.wasm plugins/
```

### 3. Start the Server

Start your Pumpkin server:

```bash
./pumpkin
```

On initial startup, Pumpkin loads the plugin and generates the default configuration and storage structure:

```
plugins/pumpkin-duels/
|-- config.toml       # Core engine settings and lobby coordinates
|-- messages.toml     # Chat messages and MiniMessage formatting
`-- pumpkin-duels.db  # SQLite database storing kits, arenas, and player stats
```

---

## Initial Setup Walkthrough

### Step 1: Configure Lobby Coordinates

Stand at the desired location in your lobby world and execute:

```
/pumpkin-duels setlobby
```
(Alias: `/pd setlobby`)

Alternatively, open `plugins/pumpkin-duels/config.toml` and manually set the coordinates where players return after matches:

```toml
lobby_world = "minecraft:overworld"
lobby_x = 0.5
lobby_y = 65.0
lobby_z = 0.5
lobby_yaw = 0.0
lobby_pitch = 0.0
```

Run `/pumpkin-duels reload` to apply configuration changes without restarting the server.

### Step 2: Create an Arena Template

1. Generate a new void template arena with a starter stone platform:
   ```
   /arena create classic minecraft:overworld
   ```
2. Teleport to the template world to inspect and build arena structures:
   ```
   /arena tp classic
   ```
3. Stand at Spawn Point 1 and run:
   ```
   /arena setspawn1 classic
   ```
4. Stand at Spawn Point 2 and run:
   ```
   /arena setspawn2 classic
   ```
5. Set the spectator vantage point:
   ```
   /arena setspectator classic
   ```
6. Define the boundary radius:
   ```
   /arena setbounds classic 50.0
   ```
7. Return to the lobby:
   ```
   /arena leave
   ```
8. Verify that the arena is registered:
   ```
   /arena list
   ```

### Step 3: Create a Duel Kit

1. Place the items and equipment you want in the kit directly into your inventory and armor slots.
2. Save the kit:
   ```
   /kit create nodebuff
   ```
3. Customize display name with MiniMessage formatting:
   ```
   /kit rename nodebuff <color:#FFA500>No Debuff</color>
   ```
4. Verify registered kits:
   ```
   /kit list
   ```

Players can now challenge each other or queue for matches with this kit.

## Building from Source

To compile the WebAssembly plugin binary locally:

1. Install the Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Add the `wasm32-wasip2` target:
   ```bash
   rustup target add wasm32-wasip2
   ```
3. Install WASI SDK (required for bundling SQLite C source):
   ```bash
   mkdir -p /tmp/wasi-sdk
   curl -sSL "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-24/wasi-sdk-24.0-x86_64-linux.tar.gz" | tar -xz -C /tmp/wasi-sdk --strip-components=1
   export WASI_SDK_PATH=/tmp/wasi-sdk
   export CC_wasm32_wasip2=/tmp/wasi-sdk/bin/clang
   ```
4. Compile the release binary:
   ```bash
   cargo build --release --target wasm32-wasip2
   ```
5. The compiled plugin is available at:
   ```
   target/wasm32-wasip2/release/pumpkin_duels.wasm
   ```
