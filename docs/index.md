---
layout: home

hero:
  name: "Pumpkin-Duels"
  text: "High-Performance 1v1 Duels Plugin"
  tagline: "Native Rust WebAssembly practice duel engine built specifically for the Pumpkin Minecraft server."
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: View Commands
      link: /commands

features:
  - title: Native WebAssembly Performance
    details: Compiled directly to wasm32-wasip2 for sandboxed, safe, and near-native execution speed inside the Pumpkin server runtime.
  - title: Cross-Platform UI Engine
    details: Modern Java 1.21+ Dialog screens with interactive ActionButtons, alongside native Bedrock ModalForms and SimpleForms with custom item icons.
  - title: Dynamic World Instancing
    details: Automatically clones arena world templates into isolated duel instances for concurrent matches and deletes temporary folders immediately upon match termination.
  - title: Matchmaking Queue and Direct Challenges
    details: Supports direct player challenges with interactive dialogs, clickable chat prompts, and automated queue matchmaking per kit.
  - title: Combat Immersion and True Freeze
    details: Zero-jitter countdown freeze via native movement locks, real-time Action Bar combat HUD meters, and victory particle fanfare.
  - title: Embedded SQLite Storage
    details: High-speed relational persistence for player statistics, ELO ratings, win streaks, kit contents, and arena metadata.
---

## Overview

Pumpkin-Duels is a competitive 1v1 practice duels plugin developed in Rust for the [Pumpkin](https://pumpkinmc.org) server architecture. Unlike legacy Bukkit or Spigot duel plugins that rely on world coordinate offsetting or world-edit schematics, Pumpkin-Duels utilizes native directory cloning and world instancing to guarantee clean, zero-interference matches.

## Server Version Requirements

| Component | Minimum Required Version | Notes |
| :--- | :--- | :--- |
| **Pumpkin Server** | `0.2.0` (or `0.2.0+26.3-26.51`+) | Native WebAssembly (`wasm32-wasip2`) component model runtime. |
| **Minecraft: Java Edition** | `1.21.4`+ (Protocol `777`) | Required for modern native client dialog screens and ActionButtons. |
| **Minecraft: Bedrock Edition** | `1.26.51`+ (Protocol `2193`) | Native Bedrock cross-play, ModalForms, and SimpleForms. |
| **Plugin Target Architecture** | `wasm32-wasip2` | WASI Preview 2 component specification. |

---

## Core Capabilities

- **Cross-Platform User Interface**: Tailored UI experiences for Java (dialogs and chest GUIs) and Bedrock (native form screens) without third-party proxy bridges.
- **Match Flow Management**: Full state machine governing countdowns, match progression, damage calculation, victory determination, and player inventory restoration.
- **Combat Feedback**: Real-time Action Bar opponent health tracking, combo counters, and victory particle celebrations.
- **Movement Physics**: Camera-smooth movement locks during countdowns without camera snapping or teleport spam.
- **Arena Management**: In-game commands to create void template worlds, set exact player spawn points, spectator coordinates, and boundary limits.
- **Kit Customization**: Snapshot player inventories into custom duel kits with configurable rules such as building permissions and natural regeneration.
- **Rating System**: Standardized ELO rating algorithm tracking wins, losses, win streaks, and competitive leaderboards.
