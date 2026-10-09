# Commands and Permissions

This page documents all commands and permissions provided by Pumpkin-Duels.

## Permissions

| Node | Default | Description |
| :--- | :--- | :--- |
| `pumpkin-duels:duels.use` | Everyone (`Allow`) | Allows players to challenge others, queue, view statistics, surrender matches, and accept/deny duels. |
| `pumpkin-duels:duels.admin` | Operators (Level 2+) | Grants full access to arena creation, kit management, lobby location setup, and configuration reloading. |

Console sender always has full administrative privileges.

---

## Player Commands

### /duel

Primary command for direct player challenges, duel responses, and matchmaking.

- **Aliases**: `/duels`, `/1v1`, `/fight`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/duel` | Displays the help menu with available duel commands. |
| `/duel <player>` | Challenges the specified player. Opens the interactive Kit Selector menu to choose the match kit. |
| `/duel invite <player> [kit]` | Directly challenges a player with an optional kit name. |
| `/duel accept [player]` | Accepts a pending duel challenge from the specified player or the most recent challenge. |
| `/duel deny [player]` | Declines or denies a pending duel challenge. |
| `/duel decline [player]` | Alias for denying a pending challenge. |
| `/duel queue [kit]` | Enters the matchmaking queue. If no kit is specified, opens the Kit Selector menu. |
| `/duel leave` | Leaves the matchmaking queue. |
| `/duel stats [player]` | Displays duel statistics (ELO, wins, losses, win rate, win streak) for yourself or the specified player. |
| `/duel top` | Displays the top players on the leaderboard sorted by ELO rating. |

---

### /queue

Quick command to enter the matchmaking queue.

- **Aliases**: `/q`, `/joinqueue`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/queue` | Opens the Kit Selector GUI to select a kit queue. |
| `/queue <kit>` | Joins the queue for the specified kit directly. |

---

### /leavequeue

Quick command to exit any active matchmaking queue.

- **Aliases**: `/lq`, `/leave`, `/unqueue`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/leavequeue` | Removes the player from the queue if currently queued. |

---

### /surrender

Forfeits your active duel match. Running the base command displays an interactive confirmation dialog (or prompts for confirmation in chat). Running `/surrender confirm` executes the forfeiture immediately, awarding the victory to your opponent, updating ELO ratings, and returning both participants to the lobby.

- **Aliases**: `/ff`, `/forfeit`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/surrender` | Displays an interactive confirmation dialog to forfeit the active match. |
| `/surrender confirm` | Confirms and executes match forfeiture immediately. |

---

### /ping

Displays your latency in milliseconds, or checks another player's latency.

- **Aliases**: `/ms`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/ping` | Displays your current ping to the server. |
| `/ping <player>` | Displays the ping of the specified player. |

---

## Administrative Commands

### /pumpkin-duels

Primary administrative command for plugin management and lobby setup.

- **Aliases**: `/pd`, `/pumpkinduels`, `/dueladmin`, `/duelsadmin`, `/da`
- **Permission**: `pumpkin-duels:duels.admin`

| Syntax | Description |
| :--- | :--- |
| `/pumpkin-duels version` | Displays the running plugin version and runtime engine info. |
| `/pumpkin-duels reload` | Hot-reloads `config.toml` and `messages.toml` from disk without server restart. |
| `/pumpkin-duels setlobby` | Sets the server-wide duel lobby spawn to your current location (world, coordinates, pitch, yaw). |

---

### /kit

Kit management command to create, delete, edit, list, and rename duel kits.

- **Aliases**: `/kits`
- **Permission**: `pumpkin-duels:duels.admin`

| Syntax | Description |
| :--- | :--- |
| `/kit create <name>` | Captures your current inventory, armor, and offhand items into a new duel kit under `<name>`. |
| `/kit delete <name>` | Permanently removes the kit from the database. |
| `/kit edit <name>` | Overwrites the existing kit's inventory loadout with your current player inventory. |
| `/kit list` | Lists all registered kits in the database with their display names. |
| `/kit rename <name> <display_name>` | Updates the kit's display name. Supports full MiniMessage formatting (for example, `/kit rename nodebuff <color:#FFA500><bold>NoDebuff</bold></color>`). |

---

### /arena

Arena management and world generation command.

- **Aliases**: `/arenas`
- **Permission**: `pumpkin-duels:duels.admin`

| Syntax | Description |
| :--- | :--- |
| `/arena create <name> [dimension]` | Generates a new void template world for the arena with a starter stone platform. Default dimension is `minecraft:overworld`. |
| `/arena setspawn1 <name>` | Sets Spawn Point 1 at the administrator's current coordinates, pitch, and yaw. |
| `/arena setspawn2 <name>` | Sets Spawn Point 2 at the administrator's current coordinates, pitch, and yaw. |
| `/arena setspectator <name>` | Sets the spectator spawn location at the current player position. |
| `/arena setbounds <name> [radius]` | Configures the arena boundary box or boundary radius. |
| `/arena tp <name>` | Teleports the administrator to the arena template world for editing and inspection. |
| `/arena leave` | Teleports the administrator back to the configured lobby spawn. |
| `/arena list` | Lists all arenas and their readiness status. |
| `/arena delete <name>` | Deletes the arena configuration from the database and removes template files. |

