# Commands and Permissions

This page documents all commands and permissions provided by Pumpkin-Duels.

## Permissions

| Node | Default | Description |
| :--- | :--- | :--- |
| `pumpkin-duels:duels.use` | Everyone (`Allow`) | Allows players to challenge others, queue, view statistics, and accept/decline duels. |
| `pumpkin-duels:duels.admin` | Operators (Level 2+) | Grants full access to arena creation, kit management, lobby location setup, and configuration reloading. |

Console sender always has full administrative privileges.

---

## Player Commands

### /duel

Primary command for direct player challenges and matchmaking.

- **Aliases**: `/duels`, `/1v1`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/duel` | Displays the help menu with available duel commands. |
| `/duel <player>` | Challenges the specified player. Opens the interactive Kit Selector menu to choose the match kit. |
| `/duel accept [player]` | Accepts a pending duel challenge from the specified player or the most recent challenge. |
| `/duel decline [player]` | Declines a pending duel challenge. |
| `/duel queue [kit]` | Enters the matchmaking queue. If no kit is specified, opens the Kit Selector menu. |
| `/duel leave` | Leaves the matchmaking queue. |
| `/duel stats [player]` | Displays duel statistics (ELO, wins, losses, win rate, win streak) for yourself or the specified player. |
| `/duel top` | Displays the top players on the leaderboard sorted by ELO rating. |

---

### /queue

Quick command to enter the matchmaking queue.

- **Aliases**: `/joinqueue`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/queue` | Opens the Kit Selector GUI to select a queue. |
| `/queue <kit>` | Joins the queue for the specified kit directly. |

---

### /leavequeue

Quick command to exit any active matchmaking queue.

- **Aliases**: `/unqueue`
- **Permission**: `pumpkin-duels:duels.use`

| Syntax | Description |
| :--- | :--- |
| `/leavequeue` | Removes the player from the queue if currently queued. |

---

## Administrative Commands

### /arena

Arena management and world generation command.

- **Aliases**: `/arenas`, `/duel admin arena`, `/dueladmin arena`
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

---

### /dueladmin kit

Kit management commands.

- **Aliases**: `/da kit`, `/duelsadmin kit`, `/duel admin kit`
- **Permission**: `pumpkin-duels:duels.admin`

| Syntax | Description |
| :--- | :--- |
| `/dueladmin kit save <name>` | Saves the administrator's current inventory items, armor slots, and offhand item as a new duel kit under `<name>`. |
| `/dueladmin kit list` | Lists all registered kits in the database. |
| `/dueladmin kit delete <name>` | Removes the specified kit from the database. |

---

### /dueladmin setlobby

- **Aliases**: `/da setlobby`, `/duel admin setlobby`
- **Permission**: `pumpkin-duels:duels.admin`

Sets the server-wide duel lobby spawn to your current location (world, X, Y, Z, yaw, pitch). Players are teleported here upon leaving matches or queues.

---

### /dueladmin reload

- **Aliases**: `/da reload`, `/duel admin reload`
- **Permission**: `pumpkin-duels:duels.admin`

Hot-reloads `config.toml` and `messages.toml` from disk, applying new settings and message templates without restarting the server.
