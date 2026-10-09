# Kit Setup and Customization

Kits define the equipment, armor, inventory contents, and gameplay rules applied to participants during a duel.

## Kit Lifecycle

1. **Snapshot**: When a kit is saved, the administrator's entire inventory (main inventory slots 0-35, armor slots, and offhand slot) is captured into a serialized JSON descriptor.
2. **Equip**: When a match begins, both players' current inventories and active potion effects are backed up into memory, and the kit items are equipped.
3. **Restoration**: When the match concludes and participants are teleported back to the lobby, their original pre-match inventories, health, food level, and game mode are fully restored.

---

## Creating a Kit

Follow these steps to create and configure a new duel kit:

### 1. Equip Items In-Game

Join the server as an administrator and arrange your inventory:

- Place swords, bows, arrows, food, and potions into hotbar and storage slots.
- Equip armor pieces in their corresponding equipment slots.
- Place a shield or totem of undying in your offhand slot.

### 2. Save the Kit

Run the kit create command:

```
/kit create nodebuff
```

The plugin captures your current loadout and persists it to the database under the identifier `nodebuff`.

### 3. Customize Display Name with MiniMessage

You can rename kits with rich colors, gradients, and formatting using MiniMessage:

```
/kit rename nodebuff <color:#FFA500><bold>No Debuff</bold></color>
```

The formatted name is automatically translated into Minecraft color codes for GUI title and lore displays.

### 4. Updating or Editing an Existing Kit

To update the items in an existing kit, adjust your inventory and run:

```
/kit edit nodebuff
```

### 5. Verify Registered Kits

List all registered kits and their active display names:

```
/kit list
```

---

## Kit Selector Interface

When players initiate a challenge with `/duel <player>` or enter matchmaking with `/queue`, Pumpkin-Duels delivers a platform-tailored selection interface:

- **Java Edition (1.21.4+)**: Presents native client dialog screens with interactive ActionButtons for each available kit. Clicking an action button immediately dispatches the challenge or enqueues the player.
- **Bedrock Edition (1.26.51+)**: Dispatches native Bedrock SimpleForms populated with kit buttons and item icons, designed specifically for mobile and console touch/controller inputs.
- **Virtual Chest GUI & Chat Menus**: Provides an interactive chest inventory interface alongside hoverable, clickable MiniMessage chat menus as reliable fallbacks.

---

## Kit Configuration Schema

In storage, kits are represented by the following structure:

```json
{
  "id": "nodebuff",
  "name": "NoDebuff",
  "display_item": "minecraft:diamond_sword",
  "inventory_items": [
    { "slot": 0, "item_id": "minecraft:diamond_sword", "count": 1 },
    { "slot": 1, "item_id": "minecraft:ender_pearl", "count": 16 },
    { "slot": 2, "item_id": "minecraft:splash_potion", "count": 1 }
  ],
  "armor_items": {
    "helmet": { "item_id": "minecraft:diamond_helmet", "count": 1 },
    "chestplate": { "item_id": "minecraft:diamond_chestplate", "count": 1 },
    "leggings": { "item_id": "minecraft:diamond_leggings", "count": 1 },
    "boots": { "item_id": "minecraft:diamond_boots", "count": 1 }
  },
  "offhand_item": { "item_id": "minecraft:totem_of_undying", "count": 1 },
  "allow_building": false,
  "natural_regeneration": true,
  "hunger_depletion": true
}
```

### Kit Rule Flags

- `allow_building`: When set to `false`, blocks inside the arena cannot be broken or placed by participants.
- `natural_regeneration`: Controls whether health naturally regenerates from food saturation.
- `hunger_depletion`: Controls whether player food level depletes during combat.
