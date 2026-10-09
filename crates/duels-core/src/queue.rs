use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueEntry {
    pub player_uuid: String,
    pub player_name: String,
    pub kit_name: String,
    pub ranked: bool,
    pub enqueued_at: u64,
}

#[derive(Clone, Debug)]
pub struct DuelChallenge {
    pub challenger_uuid: String,
    pub challenger_name: String,
    pub target_uuid: String,
    pub target_name: String,
    pub kit_name: String,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnqueueResult {
    AlreadyInQueue,
    Enqueued,
    Matched(QueueEntry, QueueEntry),
}

#[derive(Default)]
pub struct Matchmaker {
    queues: Vec<QueueEntry>,
    challenges: HashMap<(String, String), DuelChallenge>,
}

impl Matchmaker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&mut self, entry: QueueEntry) -> EnqueueResult {
        if self.is_queued_for_kit(&entry.player_uuid, &entry.kit_name, entry.ranked) {
            return EnqueueResult::AlreadyInQueue;
        }

        if let Some(pos) = self
            .queues
            .iter()
            .position(|q| q.kit_name == entry.kit_name && q.ranked == entry.ranked && q.player_uuid != entry.player_uuid)
        {
            let matched_opponent = self.queues.remove(pos);
            self.dequeue(&matched_opponent.player_uuid);
            self.dequeue(&entry.player_uuid);
            EnqueueResult::Matched(matched_opponent, entry)
        } else {
            self.queues.push(entry);
            EnqueueResult::Enqueued
        }
    }

    pub fn is_queued_for_kit(&self, player_uuid: &str, kit_name: &str, ranked: bool) -> bool {
        self.queues
            .iter()
            .any(|q| q.player_uuid == player_uuid && q.kit_name == kit_name && q.ranked == ranked)
    }

    pub fn get_player_queues(&self, player_uuid: &str) -> Vec<QueueEntry> {
        self.queues
            .iter()
            .filter(|q| q.player_uuid == player_uuid)
            .cloned()
            .collect()
    }

    pub fn get_all_player_queues(&self) -> HashMap<String, Vec<QueueEntry>> {
        let mut map: HashMap<String, Vec<QueueEntry>> = HashMap::new();
        for q in &self.queues {
            map.entry(q.player_uuid.clone())
                .or_default()
                .push(q.clone());
        }
        map
    }

    pub fn dequeue(&mut self, player_uuid: &str) -> bool {
        let initial_len = self.queues.len();
        self.queues.retain(|q| q.player_uuid != player_uuid);
        self.queues.len() < initial_len
    }

    pub fn is_queued(&self, player_uuid: &str) -> bool {
        self.queues.iter().any(|q| q.player_uuid == player_uuid)
    }

    pub fn send_challenge(&mut self, challenge: DuelChallenge) {
        let key = (
            challenge.challenger_uuid.clone(),
            challenge.target_uuid.clone(),
        );
        self.challenges.insert(key, challenge);
    }

    pub fn get_challenge(&self, challenger_uuid: &str, target_uuid: &str) -> Option<&DuelChallenge> {
        let key = (challenger_uuid.to_string(), target_uuid.to_string());
        self.challenges.get(&key)
    }

    pub fn accept_challenge(
        &mut self,
        challenger_uuid: &str,
        target_uuid: &str,
    ) -> Option<DuelChallenge> {
        let key = (challenger_uuid.to_string(), target_uuid.to_string());
        if let Some(challenge) = self.challenges.remove(&key) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if challenge.expires_at >= now {
                Some(challenge)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn accept_latest_challenge_for_target(&mut self, target_uuid: &str) -> Option<DuelChallenge> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let latest_key = self
            .challenges
            .iter()
            .filter(|((_, t_uuid), c)| t_uuid == target_uuid && c.expires_at >= now)
            .max_by_key(|(_, c)| c.created_at)
            .map(|(key, _)| key.clone());

        if let Some(key) = latest_key {
            self.challenges.remove(&key)
        } else {
            None
        }
    }

    pub fn decline_challenge(&mut self, challenger_uuid: &str, target_uuid: &str) -> bool {
        let key = (challenger_uuid.to_string(), target_uuid.to_string());
        self.challenges.remove(&key).is_some()
    }

    pub fn decline_latest_challenge_for_target(&mut self, target_uuid: &str) -> bool {
        let latest_key = self
            .challenges
            .iter()
            .filter(|((_, t_uuid), _)| t_uuid == target_uuid)
            .max_by_key(|(_, c)| c.created_at)
            .map(|(key, _)| key.clone());

        if let Some(key) = latest_key {
            self.challenges.remove(&key).is_some()
        } else {
            false
        }
    }

    pub fn cancel_challenge(&mut self, challenger_uuid: &str, target_uuid: &str) -> bool {
        let key = (challenger_uuid.to_string(), target_uuid.to_string());
        self.challenges.remove(&key).is_some()
    }

    pub fn cleanup_expired(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.challenges.retain(|_, c| c.expires_at >= now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matchmaker_queue() {
        let mut mm = Matchmaker::new();
        let entry1 = QueueEntry {
            player_uuid: "u1".into(),
            player_name: "p1".into(),
            kit_name: "sumo".into(),
            ranked: false,
            enqueued_at: 100,
        };
        let entry2 = QueueEntry {
            player_uuid: "u2".into(),
            player_name: "p2".into(),
            kit_name: "sumo".into(),
            ranked: false,
            enqueued_at: 105,
        };

        assert_eq!(mm.enqueue(entry1), EnqueueResult::Enqueued);
        assert!(mm.is_queued("u1"));

        let match_result = mm.enqueue(entry2);
        match match_result {
            EnqueueResult::Matched(p1, p2) => {
                assert_eq!(p1.player_uuid, "u1");
                assert_eq!(p2.player_uuid, "u2");
            }
            _ => panic!("Expected match"),
        }
        assert!(!mm.is_queued("u1"));
        assert!(!mm.is_queued("u2"));
    }

    #[test]
    fn test_multi_queue_and_duplicate() {
        let mut mm = Matchmaker::new();
        let entry1 = QueueEntry {
            player_uuid: "u1".into(),
            player_name: "p1".into(),
            kit_name: "classic".into(),
            ranked: false,
            enqueued_at: 100,
        };
        let entry1_dup = entry1.clone();
        let entry2 = QueueEntry {
            player_uuid: "u1".into(),
            player_name: "p1".into(),
            kit_name: "nodebuff".into(),
            ranked: false,
            enqueued_at: 110,
        };

        assert_eq!(mm.enqueue(entry1), EnqueueResult::Enqueued);
        assert_eq!(mm.enqueue(entry1_dup), EnqueueResult::AlreadyInQueue);
        assert_eq!(mm.enqueue(entry2), EnqueueResult::Enqueued);

        let queues = mm.get_player_queues("u1");
        assert_eq!(queues.len(), 2);
        assert_eq!(queues[0].kit_name, "classic");
        assert_eq!(queues[1].kit_name, "nodebuff");
    }

    #[test]
    fn test_challenge_lifecycle() {
        let mut mm = Matchmaker::new();
        let challenge = DuelChallenge {
            challenger_uuid: "c1".into(),
            challenger_name: "Challenger".into(),
            target_uuid: "t1".into(),
            target_name: "Target".into(),
            kit_name: "classic".into(),
            created_at: 100,
            expires_at: 2000000000,
        };

        mm.send_challenge(challenge);
        let accepted = mm.accept_challenge("c1", "t1");
        assert!(accepted.is_some());
        assert_eq!(accepted.unwrap().kit_name, "classic");
        assert!(mm.accept_challenge("c1", "t1").is_none());
    }

    #[test]
    fn test_accept_latest_challenge() {
        let mut mm = Matchmaker::new();
        let c1 = DuelChallenge {
            challenger_uuid: "c1".into(),
            challenger_name: "Challenger1".into(),
            target_uuid: "t1".into(),
            target_name: "Target".into(),
            kit_name: "classic".into(),
            created_at: 100,
            expires_at: 2000000000,
        };
        let c2 = DuelChallenge {
            challenger_uuid: "c2".into(),
            challenger_name: "Challenger2".into(),
            target_uuid: "t1".into(),
            target_name: "Target".into(),
            kit_name: "sumo".into(),
            created_at: 200,
            expires_at: 2000000000,
        };

        mm.send_challenge(c1);
        mm.send_challenge(c2);

        let latest = mm.accept_latest_challenge_for_target("t1");
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().challenger_uuid, "c2");
    }
}
