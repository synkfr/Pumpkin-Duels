# Getting Started

This guide explains how to install, configure, and run Pumpkin-Duels on your Pumpkin server.

## System Requirements

- **Server Software**: Pumpkin Minecraft server (version 0.2.0 or newer with WebAssembly plugin support enabled).
- **Architecture**: WebAssembly component model (`wasm32-wasip2`).
- **Operating System**: Linux (x86_64, aarch64), macOS, or Windows.

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
`-- duels.db          # SQLite database storing kits, arenas, and player stats
```

## Initial Setup Walkthrough

### Step 1: Configure Lobby Coordinates

Open `plugins/pumpkin-duels/config.toml` and set the coordinates where players will be teleported after a match finishes:

```toml
[lobby]
world = "minecraft:overworld"
x = 0.5
y = 65.0
z = 0.5
yaw = 0.0
pitch = 0.0
```

Run `/duels admin reload` to reload configuration files without restarting the server.

### Step 2: Create an Arena Template

1. Generate a new void template arena:
   ```
   /duels admin arena create classic_arena minecraft:overworld
   ```
2. Build the arena terrain or boundaries inside the template world.
3. Stand at the first player spawn location and run:
   ```
   /duels admin arena setspawn1 classic_arena
   ```
4. Stand at the second player spawn location and run:
   ```
   /duels admin arena setspawn2 classic_arena
   ```
5. Set the spectator vantage point:
   ```
   /duels admin arena setspectator classic_arena
   ```
6. Define the vertical boundaries (for instance, min Y of 0 and max Y of 120):
   ```
   /duels admin arena setbounds classic_arena 0.0 120.0
   ```
7. Save and enable the arena:
   ```
   /duels admin arena save classic_arena
   ```

### Step 3: Create a Duel Kit

1. Place the items and equipment you want in the kit directly into your player inventory and armor slots.
2. Run the kit creation command:
   ```
   /duels admin kit create nodebuff "NoDebuff"
   ```
3. Set the icon item used in GUI menus:
   ```
   /duels admin kit setdisplay nodebuff minecraft:diamond_sword
   ```
4. Save the kit:
   ```
   /duels admin kit save nodebuff
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
