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
- **Event Listeners**:
  - `PlayerDeathEvent`: Detects fatal damage during duels, identifies winner and loser, and triggers match conclusion without allowing normal death drops.
  - `PlayerMoveEvent`: Restricts movement during the countdown phase and detects arena boundary violations.
  - `BlockBreakEvent` / `BlockPlaceEvent`: Enforces kit rules such as `allow_building = false`.
- **Dynamic Instancing**: Interacts with the filesystem to duplicate template world directories into `duel_<match_id>` instances and removes them completely upon match conclusion.
- **Sound and Title Feedback**: Sends audio cues for countdown ticks, hits, and victories, alongside synchronized title displays.

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
