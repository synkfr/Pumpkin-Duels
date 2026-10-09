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
  - title: Dynamic World Instancing
    details: Automatically clones arena world templates into isolated duel instances for concurrent matches and deletes temporary folders immediately upon match termination.
  - title: Matchmaking Queue and Direct Challenges
    details: Supports direct player challenges with interactive clickable chat buttons and automated queue matchmaking per kit.
  - title: Embedded SQLite Storage
    details: High-speed relational persistence for player statistics, ELO ratings, win streaks, kit contents, and arena metadata.
  - title: Hot-Reconciliation Configuration
    details: TOML-based configuration and customizable MiniMessage chat format that preserves comments and automatically migrates schema keys.
  - title: Audio and Visual Feedback
    details: Synchronized title announcements, countdown animations, and audio effects for match states and victories.
---

## Overview

Pumpkin-Duels is a competitive 1v1 practice duels plugin developed in Rust for the [Pumpkin](https://pumpkinmc.org) server architecture. Unlike legacy Bukkit or Spigot duel plugins that rely on world coordinate offsetting or world-edit schematics, Pumpkin-Duels utilizes native directory cloning and world instancing to guarantee clean, zero-interference matches.

## Core Capabilities

- **Match Flow Management**: Full state machine governing countdowns, match progression, damage calculation, victory determination, and player inventory restoration.
- **Interactive Chat Communication**: Rich MiniMessage styling with clickable hover and command actions for immediate duel acceptance or rejection.
- **Arena Management**: In-game commands to create void template worlds, set exact player spawn points, spectator coordinates, and boundary limits.
- **Kit Customization**: Snapshot player inventories into custom duel kits with configurable rules such as building permissions and natural regeneration.
- **Rating System**: Standardized ELO rating algorithm tracking wins, losses, win streaks, and competitive leaderboards.
