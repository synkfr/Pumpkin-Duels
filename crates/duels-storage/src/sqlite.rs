use crate::error::StorageError;
use crate::models::{
    ArenaRecord, BoundsDto, KitRecord, KitRulesDto, PlayerStatsRecord, PositionDto,
};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, StorageError> {
        let conn = Connection::open(path)?;
        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.migrate()?;
        Ok(storage)
    }

    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.migrate()?;
        Ok(storage)
    }

    fn migrate(&self) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS duels_arenas (
                name TEXT PRIMARY KEY,
                template_world TEXT NOT NULL,
                spawn1_x REAL NOT NULL,
                spawn1_y REAL NOT NULL,
                spawn1_z REAL NOT NULL,
                spawn1_yaw REAL NOT NULL,
                spawn1_pitch REAL NOT NULL,
                spawn2_x REAL NOT NULL,
                spawn2_y REAL NOT NULL,
                spawn2_z REAL NOT NULL,
                spawn2_yaw REAL NOT NULL,
                spawn2_pitch REAL NOT NULL,
                spec_x REAL NOT NULL,
                spec_y REAL NOT NULL,
                spec_z REAL NOT NULL,
                spec_yaw REAL NOT NULL,
                spec_pitch REAL NOT NULL,
                min_x INTEGER NOT NULL,
                max_x INTEGER NOT NULL,
                min_z INTEGER NOT NULL,
                max_z INTEGER NOT NULL,
                min_y INTEGER NOT NULL,
                max_y INTEGER NOT NULL,
                ringout_y REAL,
                enabled INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS duels_kits (
                name TEXT PRIMARY KEY,
                display_item TEXT NOT NULL,
                inventory_json TEXT NOT NULL,
                armor_json TEXT NOT NULL,
                offhand_json TEXT NOT NULL,
                rules_json TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS duels_player_stats (
                uuid TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                wins INTEGER NOT NULL DEFAULT 0,
                losses INTEGER NOT NULL DEFAULT 0,
                current_streak INTEGER NOT NULL DEFAULT 0,
                best_streak INTEGER NOT NULL DEFAULT 0,
                elo INTEGER NOT NULL DEFAULT 1000,
                last_played INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS duels_match_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                winner_uuid TEXT NOT NULL,
                loser_uuid TEXT NOT NULL,
                kit_name TEXT NOT NULL,
                arena_name TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL,
                played_at INTEGER NOT NULL
            );
            "#,
        )?;

        Ok(())
    }

    pub fn save_arena(&self, arena: &ArenaRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT INTO duels_arenas (
                name, template_world,
                spawn1_x, spawn1_y, spawn1_z, spawn1_yaw, spawn1_pitch,
                spawn2_x, spawn2_y, spawn2_z, spawn2_yaw, spawn2_pitch,
                spec_x, spec_y, spec_z, spec_yaw, spec_pitch,
                min_x, max_x, min_z, max_z, min_y, max_y, ringout_y, enabled
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)
            ON CONFLICT(name) DO UPDATE SET
                template_world=excluded.template_world,
                spawn1_x=excluded.spawn1_x, spawn1_y=excluded.spawn1_y, spawn1_z=excluded.spawn1_z, spawn1_yaw=excluded.spawn1_yaw, spawn1_pitch=excluded.spawn1_pitch,
                spawn2_x=excluded.spawn2_x, spawn2_y=excluded.spawn2_y, spawn2_z=excluded.spawn2_z, spawn2_yaw=excluded.spawn2_yaw, spawn2_pitch=excluded.spawn2_pitch,
                spec_x=excluded.spec_x, spec_y=excluded.spec_y, spec_z=excluded.spec_z, spec_yaw=excluded.spec_yaw, spec_pitch=excluded.spec_pitch,
                min_x=excluded.min_x, max_x=excluded.max_x, min_z=excluded.min_z, max_z=excluded.max_z, min_y=excluded.min_y, max_y=excluded.max_y,
                ringout_y=excluded.ringout_y, enabled=excluded.enabled
            "#,
            params![
                arena.name, arena.template_world,
                arena.spawn1.x, arena.spawn1.y, arena.spawn1.z, arena.spawn1.yaw, arena.spawn1.pitch,
                arena.spawn2.x, arena.spawn2.y, arena.spawn2.z, arena.spawn2.yaw, arena.spawn2.pitch,
                arena.spectator_spawn.x, arena.spectator_spawn.y, arena.spectator_spawn.z, arena.spectator_spawn.yaw, arena.spectator_spawn.pitch,
                arena.bounds.min_x, arena.bounds.max_x, arena.bounds.min_z, arena.bounds.max_z, arena.bounds.min_y, arena.bounds.max_y,
                arena.bounds.ringout_y, arena.enabled as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_arena(&self, name: &str) -> Result<Option<ArenaRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT name, template_world,
                spawn1_x, spawn1_y, spawn1_z, spawn1_yaw, spawn1_pitch,
                spawn2_x, spawn2_y, spawn2_z, spawn2_yaw, spawn2_pitch,
                spec_x, spec_y, spec_z, spec_yaw, spec_pitch,
                min_x, max_x, min_z, max_z, min_y, max_y, ringout_y, enabled
            FROM duels_arenas WHERE name = ?1
            "#,
        )?;

        let mut rows = stmt.query(params![name])?;
        if let Some(row) = rows.next()? {
            let enabled_int: i32 = row.get(24)?;
            Ok(Some(ArenaRecord {
                name: row.get(0)?,
                template_world: row.get(1)?,
                spawn1: PositionDto {
                    x: row.get(2)?,
                    y: row.get(3)?,
                    z: row.get(4)?,
                    yaw: row.get(5)?,
                    pitch: row.get(6)?,
                },
                spawn2: PositionDto {
                    x: row.get(7)?,
                    y: row.get(8)?,
                    z: row.get(9)?,
                    yaw: row.get(10)?,
                    pitch: row.get(11)?,
                },
                spectator_spawn: PositionDto {
                    x: row.get(12)?,
                    y: row.get(13)?,
                    z: row.get(14)?,
                    yaw: row.get(15)?,
                    pitch: row.get(16)?,
                },
                bounds: BoundsDto {
                    min_x: row.get(17)?,
                    max_x: row.get(18)?,
                    min_z: row.get(19)?,
                    max_z: row.get(20)?,
                    min_y: row.get(21)?,
                    max_y: row.get(22)?,
                    ringout_y: row.get(23)?,
                },
                enabled: enabled_int != 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_arenas(&self) -> Result<Vec<ArenaRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT name, template_world,
                spawn1_x, spawn1_y, spawn1_z, spawn1_yaw, spawn1_pitch,
                spawn2_x, spawn2_y, spawn2_z, spawn2_yaw, spawn2_pitch,
                spec_x, spec_y, spec_z, spec_yaw, spec_pitch,
                min_x, max_x, min_z, max_z, min_y, max_y, ringout_y, enabled
            FROM duels_arenas
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            let enabled_int: i32 = row.get(24)?;
            Ok(ArenaRecord {
                name: row.get(0)?,
                template_world: row.get(1)?,
                spawn1: PositionDto {
                    x: row.get(2)?,
                    y: row.get(3)?,
                    z: row.get(4)?,
                    yaw: row.get(5)?,
                    pitch: row.get(6)?,
                },
                spawn2: PositionDto {
                    x: row.get(7)?,
                    y: row.get(8)?,
                    z: row.get(9)?,
                    yaw: row.get(10)?,
                    pitch: row.get(11)?,
                },
                spectator_spawn: PositionDto {
                    x: row.get(12)?,
                    y: row.get(13)?,
                    z: row.get(14)?,
                    yaw: row.get(15)?,
                    pitch: row.get(16)?,
                },
                bounds: BoundsDto {
                    min_x: row.get(17)?,
                    max_x: row.get(18)?,
                    min_z: row.get(19)?,
                    max_z: row.get(20)?,
                    min_y: row.get(21)?,
                    max_y: row.get(22)?,
                    ringout_y: row.get(23)?,
                },
                enabled: enabled_int != 0,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn delete_arena(&self, name: &str) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM duels_arenas WHERE name = ?1", params![name])?;
        Ok(affected > 0)
    }

    pub fn save_kit(&self, kit: &KitRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        let rules_json = serde_json::to_string(&kit.rules)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        conn.execute(
            r#"
            INSERT INTO duels_kits (name, display_item, inventory_json, armor_json, offhand_json, rules_json, enabled)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(name) DO UPDATE SET
                display_item=excluded.display_item,
                inventory_json=excluded.inventory_json,
                armor_json=excluded.armor_json,
                offhand_json=excluded.offhand_json,
                rules_json=excluded.rules_json,
                enabled=excluded.enabled
            "#,
            params![
                kit.name,
                kit.display_item,
                kit.inventory_json,
                kit.armor_json,
                kit.offhand_json,
                rules_json,
                kit.enabled as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_kit(&self, name: &str) -> Result<Option<KitRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT name, display_item, inventory_json, armor_json, offhand_json, rules_json, enabled FROM duels_kits WHERE name = ?1"
        )?;

        let mut rows = stmt.query(params![name])?;
        if let Some(row) = rows.next()? {
            let rules_json: String = row.get(5)?;
            let rules: KitRulesDto = serde_json::from_str(&rules_json)
                .map_err(|e| StorageError::Serialization(e.to_string()))?;
            let enabled_int: i32 = row.get(6)?;

            Ok(Some(KitRecord {
                name: row.get(0)?,
                display_item: row.get(1)?,
                inventory_json: row.get(2)?,
                armor_json: row.get(3)?,
                offhand_json: row.get(4)?,
                rules,
                enabled: enabled_int != 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_kits(&self) -> Result<Vec<KitRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT name, display_item, inventory_json, armor_json, offhand_json, rules_json, enabled FROM duels_kits"
        )?;

        let rows = stmt.query_map([], |row| {
            let rules_json: String = row.get(5)?;
            let rules: KitRulesDto = serde_json::from_str(&rules_json).unwrap_or_default();
            let enabled_int: i32 = row.get(6)?;
            Ok(KitRecord {
                name: row.get(0)?,
                display_item: row.get(1)?,
                inventory_json: row.get(2)?,
                armor_json: row.get(3)?,
                offhand_json: row.get(4)?,
                rules,
                enabled: enabled_int != 0,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn delete_kit(&self, name: &str) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM duels_kits WHERE name = ?1", params![name])?;
        Ok(affected > 0)
    }

    pub fn get_or_create_stats(
        &self,
        uuid: &str,
        player_name: &str,
    ) -> Result<PlayerStatsRecord, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT uuid, name, wins, losses, current_streak, best_streak, elo, last_played FROM duels_player_stats WHERE uuid = ?1"
        )?;

        let mut rows = stmt.query(params![uuid])?;
        if let Some(row) = rows.next()? {
            Ok(PlayerStatsRecord {
                uuid: row.get(0)?,
                name: row.get(1)?,
                wins: row.get(2)?,
                losses: row.get(3)?,
                current_streak: row.get(4)?,
                best_streak: row.get(5)?,
                elo: row.get(6)?,
                last_played: row.get(7)?,
            })
        } else {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            conn.execute(
                "INSERT INTO duels_player_stats (uuid, name, wins, losses, current_streak, best_streak, elo, last_played) VALUES (?1, ?2, 0, 0, 0, 0, 1000, ?3)",
                params![uuid, player_name, now],
            )?;

            Ok(PlayerStatsRecord {
                uuid: uuid.to_string(),
                name: player_name.to_string(),
                wins: 0,
                losses: 0,
                current_streak: 0,
                best_streak: 0,
                elo: 1000,
                last_played: now,
            })
        }
    }

    pub fn update_stats(&self, stats: &PlayerStatsRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            UPDATE duels_player_stats
            SET name = ?2, wins = ?3, losses = ?4, current_streak = ?5, best_streak = ?6, elo = ?7, last_played = ?8
            WHERE uuid = ?1
            "#,
            params![
                stats.uuid, stats.name, stats.wins, stats.losses,
                stats.current_streak, stats.best_streak, stats.elo, stats.last_played
            ],
        )?;
        Ok(())
    }

    pub fn get_top_players(&self, limit: usize) -> Result<Vec<PlayerStatsRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT uuid, name, wins, losses, current_streak, best_streak, elo, last_played FROM duels_player_stats ORDER BY elo DESC, wins DESC LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(PlayerStatsRecord {
                uuid: row.get(0)?,
                name: row.get(1)?,
                wins: row.get(2)?,
                losses: row.get(3)?,
                current_streak: row.get(4)?,
                best_streak: row.get(5)?,
                elo: row.get(6)?,
                last_played: row.get(7)?,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn record_match(
        &self,
        winner_uuid: &str,
        loser_uuid: &str,
        kit_name: &str,
        arena_name: &str,
        duration_seconds: u32,
    ) -> Result<i64, StorageError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        conn.execute(
            r#"
            INSERT INTO duels_match_history (winner_uuid, loser_uuid, kit_name, arena_name, duration_seconds, played_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![winner_uuid, loser_uuid, kit_name, arena_name, duration_seconds, now],
        )?;

        Ok(conn.last_insert_rowid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_in_memory_arena_crud() {
        let storage = SqliteStorage::open_in_memory().unwrap();

        let arena = ArenaRecord {
            name: "test_arena".to_string(),
            template_world: "world".to_string(),
            spawn1: PositionDto {
                x: 0.0,
                y: 100.0,
                z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
            },
            spawn2: PositionDto {
                x: 10.0,
                y: 100.0,
                z: 0.0,
                yaw: 180.0,
                pitch: 0.0,
            },
            spectator_spawn: PositionDto {
                x: 5.0,
                y: 110.0,
                z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
            },
            bounds: BoundsDto {
                min_x: -20,
                max_x: 20,
                min_z: -20,
                max_z: 20,
                min_y: 80,
                max_y: 120,
                ringout_y: Some(90.0),
            },
            enabled: true,
        };

        storage.save_arena(&arena).unwrap();
        let loaded = storage.get_arena("test_arena").unwrap().unwrap();
        assert_eq!(loaded.name, "test_arena");
        assert_eq!(loaded.bounds.min_x, -20);

        let list = storage.list_arenas().unwrap();
        assert_eq!(list.len(), 1);

        let deleted = storage.delete_arena("test_arena").unwrap();
        assert!(deleted);
        assert!(storage.get_arena("test_arena").unwrap().is_none());
    }

    #[test]
    fn test_sqlite_in_memory_kit_crud() {
        let storage = SqliteStorage::open_in_memory().unwrap();

        let kit = KitRecord {
            name: "sumo_test".to_string(),
            display_item: "minecraft:lead".to_string(),
            inventory_json: "[]".to_string(),
            armor_json: "{}".to_string(),
            offhand_json: "null".to_string(),
            rules: KitRulesDto {
                sumo_mode: true,
                ..Default::default()
            },
            enabled: true,
        };

        storage.save_kit(&kit).unwrap();
        let loaded = storage.get_kit("sumo_test").unwrap().unwrap();
        assert_eq!(loaded.name, "sumo_test");
        assert!(loaded.rules.sumo_mode);

        let deleted = storage.delete_kit("sumo_test").unwrap();
        assert!(deleted);
        assert!(storage.get_kit("sumo_test").unwrap().is_none());
    }
}
