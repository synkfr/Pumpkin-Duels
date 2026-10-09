use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub storage_type: String,
    pub sqlite_file: String,
    pub mysql_url: String,
    pub countdown_seconds: u32,
    pub challenge_expire_seconds: u32,
    pub worlds_directory: String,
    pub lobby_world: String,
    pub lobby_x: f64,
    pub lobby_y: f64,
    pub lobby_z: f64,
    pub lobby_yaw: f32,
    pub lobby_pitch: f32,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            storage_type: "sqlite".to_string(),
            sqlite_file: "pumpkin-duels.db".to_string(),
            mysql_url: "mysql://root:password@localhost:3306/pumpkin_duels".to_string(),
            countdown_seconds: 3,
            challenge_expire_seconds: 30,
            worlds_directory: "worlds".to_string(),
            lobby_world: "minecraft:overworld".to_string(),
            lobby_x: 0.5,
            lobby_y: 100.0,
            lobby_z: 0.5,
            lobby_yaw: 0.0,
            lobby_pitch: 0.0,
        }
    }
}
