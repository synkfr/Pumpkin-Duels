# Arena Management

Pumpkin-Duels features an instanced world arena system. Instead of sharing a single world or requiring complex world region schematics, each duel match is allocated its own dynamic cloned instance of an arena template.

## How Arena Instancing Works

```
worlds/
|-- arena_template_sumo/         [Read-Only Arena Template]
|   |-- region/
|   `-- level.dat
|-- duel_1a2b3c4d/               [Active Match 1 Instance]
|   `-- ... (cloned from sumo)
`-- duel_9f8e7d6c/               [Active Match 2 Instance]
    `-- ... (cloned from sumo)
```

1. **Template Definition**: Administrators design the arena in a template world folder (`worlds/<arena_id>`).
2. **Match Initialization**: When two players start a duel, Pumpkin-Duels duplicates the template files into an isolated directory named `worlds/duel_<match_id>`.
3. **Execution**: Players battle inside this private instance. Damage to blocks, explosions, and placed blocks remain strictly contained within this temporary world.
4. **Cleanup**: As soon as the match concludes and players are teleported to the lobby, the `duel_<match_id>` directory is completely purged from disk, leaving zero leftover files.

---

## Arena Setup Workflow

Follow these steps to create and configure a competitive arena:

### 1. Create the Void Template

Execute the creation command:

```
/arena create classic minecraft:overworld
```

The plugin creates a dedicated void template folder under `worlds/classic` and generates an initial stone platform at `y = 64` to prevent players from falling into the void or triggering chunk loading issues.

### 2. Teleport to the Template

Teleport to the newly created template world:

```
/arena tp classic
```

Build your arena map, borders, structures, and visual decorations.

### 3. Set Player Spawns

Stand at the designated position for Player 1, facing the center of the arena, and run:

```
/arena setspawn1 classic
```

Next, stand at the designated position for Player 2, facing Player 1, and run:

```
/arena setspawn2 classic
```

Both commands record your exact X, Y, Z coordinates, as well as Yaw and Pitch orientation angles.

### 4. Set Spectator Location

Stand at an elevated vantage point overlooking the arena and run:

```
/arena setspectator classic
```

Eliminated participants and spectators will be placed here in spectator mode.

### 5. Define Boundaries

Set the arena vertical and horizontal boundaries:

```
/arena setbounds classic 50.0
```

Players who exit this boundary or fall below minimum Y will be handled by the match state machine.

### 6. Return to Lobby

Once configuration is complete, return to the server lobby:

```
/arena leave
```

Verify that the arena is listed and ready:

```
/arena list
```

---

## Arena Database Model

Arena templates are persisted in the SQLite database with the following attributes:

- `id`: Unique string identifier (e.g. `sumo`, `classic`).
- `name`: Display name.
- `world_template_folder`: Directory path relative to `worlds/`.
- `dimension_name`: Dimension type (e.g. `minecraft:overworld`, `minecraft:the_nether`).
- `spawn_pos_1`: Location vector (X, Y, Z, yaw, pitch).
- `spawn_pos_2`: Location vector (X, Y, Z, yaw, pitch).
- `spectator_pos`: Location vector (X, Y, Z, yaw, pitch).
- `bounds`: Minimum and maximum Y levels, bounding radius.
- `enabled`: Boolean toggle indicating whether the arena is open for matchmaking.
