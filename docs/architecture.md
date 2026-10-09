# Architecture and Internals

Pumpkin-Duels is engineered as a modular Rust Cargo workspace compiled to WebAssembly (`wasm32-wasip2`) targeting the Pumpkin server plugin runtime.

```
Pumpkin-Duels/
|-- crates/
|   |-- duels-core/        # Pure domain logic, state machines, matchmaking, and ELO calculations
|   |-- duels-config/      # TOML hot-reconciliation configuration engine
|   |-- duels-storage/     # SQLite persistence layer and repository interfaces
|   |-- duels-gui/         # Virtual chest GUI builder abstractions
|   `-- pumpkin-duels/     # Pumpkin server plugin entrypoint, commands, and listeners
`-- docs/                  # VitePress documentation site
```

## Target Platform and Requirements

| Requirement | Minimum Version | Notes |
| :--- | :--- | :--- |
| **Server Software** | Pumpkin `0.2.0` (`0.2.0+26.3-26.51`+) | Native WebAssembly (`wasm32-wasip2`) component model host. |
| **Java Edition** | Minecraft `1.21.4`+ (Protocol `777`) | Supports native client dialog screens with ActionButtons. |
| **Bedrock Edition** | Minecraft `1.26.51`+ (Protocol `2193`) | Native cross-play ModalForms and SimpleForms without Geyser. |
| **Compilation Target** | `wasm32-wasip2` | WASI Preview 2 component specification. |

---

## Workspace Crates

### 1. duels-core

Contains all core match domain models and business logic without any dependency on Minecraft server APIs:

- **MatchSession**: Deterministic state machine governing match phases (`Waiting`, `Countdown`, `Active`, `Ending`, `Finished`).
- **Matchmaker**: Kit-segmented matchmaking queue pairing players with matching criteria.
- **PlayerStats**: Mathematical ELO rating calculations with configurable K-factors, streak tracking, and win ratios.
- **ArenaTemplate and ActiveArenaInstance**: Arena configuration data models and runtime tracking structures.

Because `duels-core` contains no server-specific code, all state machine transitions and matchmaking logic are covered by fast unit tests.

### 2. duels-config

A robust configuration management engine built on `toml_edit` and `serde`:

- **Hot-Reconciliation**: Compares existing configuration files against newly generated template structures. Any newly introduced keys are inserted into the user's config file without wiping existing comments, spacing, or custom values.
- **Thread-Safe Access**: Encapsulates configuration objects within `Arc<RwLock<T>>` for read and reload operations across threads.

### 3. duels-storage

High-performance persistence engine:

- **SQLite Backend**: Employs bundled SQLite via `rusqlite` compiled directly to WebAssembly.
- **Relational Tables**:
  - `players`: UUID, username, ELO rating, wins, losses, current streak, best streak, and timestamp.
  - `kits`: Unique ID, display item, and JSON serialized item loadout.
  - `arenas`: Unique ID, name, dimension, spawn positions, bounding coordinates, and enabled flag.
  - `match_history`: Log of completed matches, participants, winners, ELO deltas, and durations.

### 4. duels-gui

Chest inventory GUI library:

- **Declarative Layouts**: Constructs inventory grids with rows of 9 slots.
- **Action Buttons**: Maps slot coordinates to custom click handlers, lore components, and display materials.
- **State Handling**: Prevents item theft or ghost clicks by intercepting inventory click events.

### 5. pumpkin-duels

The primary plugin crate compiling to a CDYLIB WebAssembly module:

- **Plugin Entry**: Implements `pumpkin_plugin_api::PumpkinPlugin`.
- **Cross-Platform UI (`ui.rs`)**:
  - Dispatches native Java 1.21+ Dialog screens with interactive ActionButtons.
  - Dispatches native Bedrock ModalForms and SimpleForms with custom item icons.
  - Provides fallback chest GUI for legacy Java clients.
- **Event Listeners (`listeners.rs`)**:
  - `DialogClickActionEvent`: Intercepts client dialog button actions (`duel:accept:<player>`, `duel:surrender:confirm`, `duel:queue:<kit>`).
  - `BedrockFormResponseEvent`: Intercepts Bedrock form submissions and kit selections.
  - `EntityDamageEvent`: Cancels pre-match damage, updates live nametag and below-name scoreboard health values, and spawns hit particles.
  - `PlayerDeathEvent`: Concludes matches, updates ELO, and restores inventories without item loss.
  - `PlayerMoveEvent`: Detects boundary ringouts while camera stays smooth under native `set_movement_locked(true)`.
  - `PlayerInteractEvent`: Intercepts lobby sword right-clicks to open kit selectors.
  - `BlockBreakEvent` / `BlockPlaceEvent`: Enforces kit building restrictions (`allow_build = false`).
- **Dynamic Instancing**: Duplicates template world directories into `duel_<match_id>` instances and completely removes temporary directories upon match conclusion.
- **Immersion**: Controls movement locking during countdowns, real-time nametag and below-name scoreboard health feedback, and celebratory victory particle emitters.

---

## State Machine Diagram

```
[Challenge / Queue]
        |
        v
    [Waiting]  <-- Players teleported to cloned arena instance
        |
        v
   [Countdown] <-- Players frozen, audio countdown ticks (3, 2, 1)
        |
        v
    [Active]   <-- Combat unlocked, damage and boundaries monitored
        |
        v
    [Ending]   <-- Victor determined, victory sound, title banner
        |
        v
   [Finished]  <-- Inventories restored, players teleported to lobby,
                   temporary world folder deleted from disk
```
