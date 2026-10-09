use crate::arena::ActiveArenaInstance;
use crate::kit::Kit;
use crate::stats::CombatTracker;

#[derive(Clone, Debug, PartialEq)]
pub enum MatchPhase {
    Countdown { remaining_ticks: u32 },
    Fighting { elapsed_seconds: u32 },
    Ended { winner: String, loser: String },
}

#[derive(Clone, Debug)]
pub struct MatchSession {
    pub id: String,
    pub player1_uuid: String,
    pub player1_name: String,
    pub player2_uuid: String,
    pub player2_name: String,
    pub kit: Kit,
    pub arena: ActiveArenaInstance,
    pub phase: MatchPhase,
    pub combat: CombatTracker,
    pub started_at: u64,
}

impl MatchSession {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        player1_uuid: String,
        player1_name: String,
        player2_uuid: String,
        player2_name: String,
        kit: Kit,
        arena: ActiveArenaInstance,
        countdown_ticks: u32,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            player1_uuid,
            player1_name,
            player2_uuid,
            player2_name,
            kit,
            arena,
            phase: MatchPhase::Countdown {
                remaining_ticks: countdown_ticks,
            },
            combat: CombatTracker::new(),
            started_at: now,
        }
    }

    pub fn is_participant(&self, uuid: &str) -> bool {
        self.player1_uuid == uuid || self.player2_uuid == uuid
    }

    pub fn get_opponent(&self, uuid: &str) -> Option<(String, String)> {
        if self.player1_uuid == uuid {
            Some((self.player2_uuid.clone(), self.player2_name.clone()))
        } else if self.player2_uuid == uuid {
            Some((self.player1_uuid.clone(), self.player1_name.clone()))
        } else {
            None
        }
    }

    pub fn set_winner(&mut self, winner_uuid: &str) -> Option<(String, String)> {
        let loser_uuid = if self.player1_uuid == winner_uuid {
            self.player2_uuid.clone()
        } else {
            self.player1_uuid.clone()
        };

        self.phase = MatchPhase::Ended {
            winner: winner_uuid.to_string(),
            loser: loser_uuid.clone(),
        };

        Some((winner_uuid.to_string(), loser_uuid))
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.phase,
            MatchPhase::Countdown { .. } | MatchPhase::Fighting { .. }
        )
    }

    pub fn is_fighting(&self) -> bool {
        matches!(self.phase, MatchPhase::Fighting { .. })
    }
}
