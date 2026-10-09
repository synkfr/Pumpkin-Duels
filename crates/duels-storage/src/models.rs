use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundsDto {
    pub min_x: i32,
    pub max_x: i32,
    pub min_z: i32,
    pub max_z: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub ringout_y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaRecord {
    pub name: String,
    pub template_world: String,
    pub spawn1: PositionDto,
    pub spawn2: PositionDto,
    pub spectator_spawn: PositionDto,
    pub bounds: BoundsDto,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KitRulesDto {
    pub allow_build: bool,
    pub hunger: bool,
    pub natural_regen: bool,
    pub combo_mode: bool,
    pub boxing_mode: bool,
    pub sumo_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KitRecord {
    pub name: String,
    pub display_item: String,
    pub inventory_json: String,
    pub armor_json: String,
    pub offhand_json: String,
    pub rules: KitRulesDto,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatsRecord {
    pub uuid: String,
    pub name: String,
    pub wins: u32,
    pub losses: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub elo: u32,
    pub last_played: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchHistoryRecord {
    pub id: i64,
    pub winner_uuid: String,
    pub loser_uuid: String,
    pub kit_name: String,
    pub arena_name: String,
    pub duration_seconds: u32,
    pub played_at: u64,
}
