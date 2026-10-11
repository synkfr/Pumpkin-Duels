# Configuration

Pumpkin-Duels provides two primary configuration files located under `plugins/pumpkin-duels/`:

1. `config.toml`: Engine settings, database configuration, countdown intervals, and lobby spawn coordinates.
2. `messages.toml`: Text message templates formatted with MiniMessage styling, supporting custom colors, hover actions, and clickable commands.

The configuration engine includes automatic reconciliation. When upgrading plugin versions or adding new options, existing comments and formatting are preserved while missing keys are automatically populated with defaults.

---

## config.toml

Below is the complete reference for `config.toml`:

```toml
# Storage backend: "sqlite" (default) or "mysql"
storage_type = "sqlite"

# SQLite database file name (located inside the plugins/pumpkin-duels directory)
sqlite_file = "pumpkin-duels.db"

# MySQL database connection URL (used when storage_type is set to "mysql")
mysql_url = "mysql://root:password@localhost:3306/pumpkin_duels"

# Match countdown duration in seconds before combat begins
countdown_seconds = 3

# Duration in seconds before a pending direct duel challenge expires
challenge_expire_seconds = 30

# Root directory containing world templates and instances
worlds_directory = "worlds"

# World identifier for the global duel lobby
lobby_world = "minecraft:overworld"

# Exact lobby coordinates where players return after matches
lobby_x = 0.5
lobby_y = 100.0
lobby_z = 0.5

# Lobby orientation angles
lobby_yaw = 0.0
lobby_pitch = 0.0
```

### Configuration Keys

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `storage_type` | String | `"sqlite"` | Database driver to use. |
| `sqlite_file` | String | `"pumpkin-duels.db"` | Path to the SQLite database file. |
| `mysql_url` | String | `mysql://...` | Connection URI when using MySQL. |
| `countdown_seconds` | Integer | `3` | Seconds players remain frozen at spawns before match start. |
| `challenge_expire_seconds` | Integer | `30` | Timeout window for direct duel requests. |
| `worlds_directory` | String | `"worlds"` | Server worlds root directory. |
| `lobby_world` | String | `"minecraft:overworld"` | Target dimension identifier for lobby teleports. |
| `lobby_x`, `lobby_y`, `lobby_z` | Float | `0.5`, `100.0`, `0.5` | Coordinates for the post-match spawn location. |
| `lobby_yaw`, `lobby_pitch` | Float | `0.0`, `0.0` | Player viewing angles when teleported to lobby. |

---

## messages.toml

Pumpkin-Duels uses standard MiniMessage formatting syntax.

### Color Palette

The default message theme follows a clean, minimalist palette:

| Role | Color Hex | Sample Syntax |
| :--- | :--- | :--- |
| Normal Text | `#A0A0A0` | `<color:#A0A0A0>Text content</color>` |
| Error / Danger | `#E20000` | `<color:#E20000>Error occurred</color>` |
| Success | `#00E262` | `<color:#00E262>Operation successful</color>` |
| Warning / Highlight | `#E2B700` | `<color:#E2B700>Notice</color>` |
| Muted / Accent | `#555555` | `<color:#555555>»</color>` |

### Interactive Actions

Messages support native Minecraft clickable and hover components:

- **Run Command**: `<click:run_command:'/command arg'>Clickable Text</click>`
- **Hover Text**: `<hover:show_text:'Tooltip description'>Hoverable Text</hover>`
- **Formatting**: `<bold>Bold</bold>`, `<italic>Italic</italic>`, `<underlined>Underline</underlined>`

### Default Message Templates

```toml
prefix = "<color:#E2B700>Duels</color> <color:#555555>»</color> "
duel_sent = "<color:#A0A0A0>Duel request sent to</color> <color:#00E262>{target}</color> <color:#555555>•</color> <color:#A0A0A0>Kit:</color> <color:#E2B700>{kit}</color>"
duel_received = "<color:#00E262>{challenger}</color> <color:#A0A0A0>challenged you to a duel</color> <color:#555555>•</color> <color:#A0A0A0>Kit:</color> <color:#E2B700>{kit}</color> <color:#555555>|</color> <click:run_command:'/duel accept {challenger}'><hover:show_text:'Click to accept'><color:#00E262>[ACCEPT]</color></hover></click> <click:run_command:'/duel decline {challenger}'><hover:show_text:'Click to decline'><color:#E20000>[DECLINE]</color></hover></click>"
duel_accepted = "<color:#00E262>Duel request accepted!</color> <color:#A0A0A0>Teleporting to arena...</color>"
duel_declined = "<color:#A0A0A0>Declined duel request from</color> <color:#E2B700>{challenger}</color><color:#A0A0A0>.</color>"
duel_cancelled = "<color:#A0A0A0>Duel request to</color> <color:#E2B700>{target}</color> <color:#A0A0A0>was cancelled.</color>"
duel_expired = "<color:#E20000>The duel request has expired.</color>"
duel_cannot_self = "<color:#E20000>You cannot challenge yourself.</color>"
duel_already_in_match = "<color:#E20000>You are already in an active duel.</color>"
target_already_in_match = "<color:#E2B700>{target}</color> <color:#E20000>is currently in a duel.</color>"
queue_joined = "<color:#A0A0A0>Searching for opponent</color> <color:#555555>•</color> <color:#A0A0A0>Queue:</color> <color:#E2B700>{kit}</color>"
queue_left = "<color:#A0A0A0>Left the matchmaking queue.</color>"
queue_already = "<color:#E20000>You are already in a matchmaking queue.</color>"
queue_already_kit = "<color:#E20000>You're already in queue on that kit.</color>"
queue_action_bar = "<color:#A0A0A0>InQueue: </color><color:#00E262>{kit}</color> <color:#555555>({time})</color>"
queue_action_bar_multi = "<color:#A0A0A0>InQueue: </color><color:#00E262>{kit}</color><color:#A0A0A0>, </color><color:#E2B700>+{more} more</color> <color:#555555>({time})</color>"
queue_not_in = "<color:#E20000>You are not currently in a queue.</color>"
match_countdown = "<color:#A0A0A0>Starting in</color> <color:#00E262>{seconds}s</color><color:#A0A0A0>...</color>"
match_start = "<color:#00E262>FIGHT!</color>"
match_won = "<color:#00E262>VICTORY!</color> <color:#555555>•</color> <color:#A0A0A0>You defeated</color> <color:#E2B700>{loser}</color><color:#A0A0A0>.</color>"
match_lost = "<color:#E20000>DEFEAT!</color> <color:#555555>•</color> <color:#A0A0A0>You were defeated by</color> <color:#E2B700>{winner}</color><color:#A0A0A0>.</color>"
match_broadcast_win = "<color:#00E262>{winner}</color> <color:#A0A0A0>defeated</color> <color:#E20000>{loser}</color> <color:#555555>•</color> <color:#E2B700>{arena}</color> <color:#555555>({kit})</color>"
arena_created = "<color:#00E262>Created arena</color> <color:#E2B700>{name}</color><color:#00E262>.</color>"
arena_deleted = "<color:#A0A0A0>Deleted arena</color> <color:#E2B700>{name}</color><color:#A0A0A0>.</color>"
arena_already_exists = "<color:#E20000>Arena</color> <color:#E2B700>{name}</color> <color:#E20000>already exists.</color>"
arena_spawn1_set = "<color:#A0A0A0>Spawn 1 for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>"
arena_spawn2_set = "<color:#A0A0A0>Spawn 2 for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>"
arena_spectator_set = "<color:#A0A0A0>Spectator spawn for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>"
arena_bounds_set = "<color:#A0A0A0>Bounds for arena</color> <color:#E2B700>{name}</color> <color:#00E262>updated.</color>"
arena_empty_list = "<color:#A0A0A0>No arenas found in database.</color>"
kit_created = "<color:#00E262>Created kit</color> <color:#E2B700>{name}</color><color:#00E262>.</color>"
kit_saved = "<color:#00E262>Current inventory saved to kit</color> <color:#E2B700>{name}</color><color:#00E262>!</color>"
kit_deleted = "<color:#A0A0A0>Deleted kit</color> <color:#E2B700>{name}</color><color:#A0A0A0>.</color>"
kit_already_exists = "<color:#E20000>Kit</color> <color:#E2B700>{name}</color> <color:#E20000>already exists.</color>"
kit_empty_list = "<color:#A0A0A0>No kits found in database.</color>"
lobby_set = "<color:#00E262>Global duel lobby spawn set to your position.</color>"
reload_success = "<color:#00E262>Configuration and messages reloaded successfully.</color>"
player_not_found = "<color:#E20000>Player</color> <color:#E2B700>{player}</color> <color:#E20000>was not found online.</color>"
arena_not_found = "<color:#E20000>Arena</color> <color:#E2B700>{name}</color> <color:#E20000>was not found.</color>"
kit_not_found = "<color:#E20000>Kit</color> <color:#E2B700>{name}</color> <color:#E20000>was not found.</color>"
no_permission = "<color:#E20000>You do not have permission to execute this command.</color>"
cannot_break_arena = "<color:#E20000>You cannot modify blocks in the duel arena.</color>"
cannot_break_lobby = "<color:#E20000>You cannot break blocks in the lobby.</color>"
cannot_place_lobby = "<color:#E20000>You cannot place blocks in the lobby.</color>"
only_players = "<color:#E20000>This command can only be executed by a player in-game.</color>"
command_help = "<color:#E2B700>Commands</color> <color:#555555>»</color> <color:#A0A0A0>/duel <accept|deny|invite> <player>, /queue [kit], /leavequeue, /surrender, /ping, /duel stats, /duel top</color>"
admin_help = "<color:#E2B700>Admin Commands</color> <color:#555555>»</color> <color:#A0A0A0>/pd <version|reload|setlobby>, /arena <create|setspawn1|setspawn2|setspectator|setbounds|tp|leave|list|delete>, /kit <create|delete|edit|list|rename></color>"
kit_renamed = "<color:#00E262>Kit</color> <color:#E2B700>{old_name}</color> <color:#00E262>renamed to</color> {new_name}<color:#00E262>!</color>"
kit_edited = "<color:#00E262>Kit</color> <color:#E2B700>{name}</color> <color:#00E262>updated with current inventory!</color>"
not_in_match = "<color:#E20000>You are not currently in an active match.</color>"
match_surrendered = "<color:#E2B700>{player}</color> <color:#E20000>surrendered the match.</color>"
ping_self = "<color:#A0A0A0>Your ping:</color> <color:#00E262>{ping}ms</color>"
ping_other = "<color:#E2B700>{player}'s</color> <color:#A0A0A0>ping:</color> <color:#00E262>{ping}ms</color>"
version_info = "<color:#E2B700>Pumpkin-Duels</color> <color:#555555>•</color> <color:#00E262>v0.1.0</color> <color:#555555>•</color> <color:#A0A0A0>Pumpkin 0.2.0 (Protocol 777)</color>"
combat_nametag = "<color:#A0A0A0>{display_name}</color>"
combat_belowname = "<color:{color}>{heart} ❤</color>"
```

### Combat Display Placeholders

During active matches, player nametags and below-name scoreboards support dynamic formatting:

- `{display_name}`: Target player username or custom display name.
- `{heart}`: Exact heart count, displayed as integers for full hearts (e.g. `10`, `9`) or decimals for half-hearts (e.g. `9.5`, `8.5`).
- `{color}`: Dynamic hex color code automatically reflecting active player status effects:
  - Absorption (Golden Apple or extra health above 20 HP): `#FFEE2E`
  - Wither Effect: `#303030`
  - Poison Effect: `#2DAD48`
  - Freezing (Powder Snow): `#3BE3FF`
  - Normal / Healthy: `#FF5555`

### Template Placeholders

Messages dynamically substitute variables at runtime:

- `{target}`: Name of the recipient player.
- `{challenger}`: Name of the challenging player.
- `{winner}`: Name of the match winner.
- `{loser}`: Name of the defeated participant.
- `{kit}`: Name of the match kit.
- `{arena}`: Identifier of the duel arena.
- `{seconds}`: Remaining countdown ticks or seconds.
- `{player}`: Targeted player name in queries.
- `{time}`: Elapsed queue time formatted as `HH:MM:SS` (e.g. `00:00:15`).
- `{more}`: Number of additional queued kits when multi-queuing (e.g. `1`, `2`).
