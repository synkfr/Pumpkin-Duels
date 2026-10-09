use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct CombatStats {
    pub hits_landed: u32,
    pub current_combo: u32,
    pub best_combo: u32,
    pub damage_dealt: f32,
}

#[derive(Clone, Debug, Default)]
pub struct CombatTracker {
    players: HashMap<String, CombatStats>,
    last_hit_player: Option<String>,
}

impl CombatTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_hit(&mut self, attacker_uuid: &str, victim_uuid: &str, damage: f32) {
        let stats = self.players.entry(attacker_uuid.to_string()).or_default();
        stats.hits_landed += 1;
        stats.damage_dealt += damage;

        if let Some(ref last) = self.last_hit_player {
            if last == attacker_uuid {
                stats.current_combo += 1;
            } else {
                stats.current_combo = 1;
            }
        } else {
            stats.current_combo = 1;
        }

        if stats.current_combo > stats.best_combo {
            stats.best_combo = stats.current_combo;
        }

        if let Some(victim_stats) = self.players.get_mut(victim_uuid) {
            victim_stats.current_combo = 0;
        }

        self.last_hit_player = Some(attacker_uuid.to_string());
    }

    pub fn get_stats(&self, player_uuid: &str) -> CombatStats {
        self.players.get(player_uuid).cloned().unwrap_or_default()
    }
}
