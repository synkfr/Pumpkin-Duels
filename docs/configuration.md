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
duel_sent = "<color:#A0A0A0>Duel request sent to <color:#00E262>{target}</color> with kit <color:#E2B700>{kit}</color>.</color>"
duel_received = "<color:#00E262>{challenger}</color> <color:#A0A0A0>challenged you to a duel with kit <color:#E2B700>{kit}</color>!</color> <click:run_command:'/duel accept {challenger}'><hover:show_text:'Click to accept!'><color:#00E262><bold>[ACCEPT]</bold></color></hover></click>  <click:run_command:'/duel decline {challenger}'><hover:show_text:'Click to decline.'><color:#E20000><bold>[DECLINE]</bold></color></hover></click>"
duel_accepted = "<color:#00E262>Duel request accepted! Teleporting to arena...</color>"
duel_declined = "<color:#E20000>You declined the duel request from <color:#E2B700>{challenger}</color>.</color>"
duel_cancelled = "<color:#E20000>Duel request to <color:#E2B700>{target}</color> cancelled.</color>"
duel_expired = "<color:#E20000>The duel request has expired.</color>"
duel_cannot_self = "<color:#E20000>You cannot challenge yourself.</color>"
duel_already_in_match = "<color:#E20000>You are already in an active duel.</color>"
target_already_in_match = "<color:#E20000><color:#E2B700>{target}</color> is currently in a duel.</color>"
queue_joined = "<color:#A0A0A0>Searching for opponent in <color:#E2B700>{kit}</color> queue...</color>"
queue_left = "<color:#E20000>Left the matchmaking queue.</color>"
queue_already = "<color:#E20000>You are already queued for a duel.</color>"
queue_not_in = "<color:#E20000>You are not currently in a queue.</color>"
match_countdown = "<color:#E2B700>Starting in <color:#00E262>{seconds}</color>s...</color>"
match_start = "<color:#00E262><bold>FIGHT!</bold></color>"
match_won = "<color:#00E262><bold>VICTORY!</bold></color> <color:#A0A0A0>You defeated <color:#E2B700>{loser}</color>!</color>"
match_lost = "<color:#E20000><bold>DEFEAT!</bold></color> <color:#A0A0A0>You were defeated by <color:#E2B700>{winner}</color>!</color>"
match_broadcast_win = "<color:#00E262>{winner}</color> <color:#A0A0A0>defeated</color> <color:#E20000>{loser}</color> <color:#A0A0A0>in</color> <color:#E2B700>{arena}</color> <color:#555555>({kit})</color>"
```

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
