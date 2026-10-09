use crate::button::{ActionFn, ClickContext};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default, Clone)]
pub struct GuiSessionRegistry {
    sessions: Arc<RwLock<HashMap<String, HashMap<u32, ActionFn>>>>,
}

impl GuiSessionRegistry {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, player_uuid: String, actions: HashMap<u32, ActionFn>) {
        let mut guard = self.sessions.write().unwrap();
        guard.insert(player_uuid, actions);
    }

    pub fn remove(&self, player_uuid: &str) {
        let mut guard = self.sessions.write().unwrap();
        guard.remove(player_uuid);
    }

    pub fn has_session(&self, player_uuid: &str) -> bool {
        let guard = self.sessions.read().unwrap();
        guard.contains_key(player_uuid)
    }

    pub fn handle_click(&self, player_uuid: &str, ctx: ClickContext) -> bool {
        let action = {
            let guard = self.sessions.read().unwrap();
            guard
                .get(player_uuid)
                .and_then(|slots| slots.get(&ctx.slot).cloned())
        };

        if let Some(handler) = action {
            handler(ctx);
            true
        } else {
            false
        }
    }
}
