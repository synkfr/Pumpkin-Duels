use duels_config::ConfigManager;
use pumpkin_plugin_api::text::TextComponent;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessagesConfig {
    pub prefix: String,
    pub duel_sent: String,
    pub duel_received: String,
    pub duel_accepted: String,
    pub duel_declined: String,
    pub duel_cancelled: String,
    pub duel_expired: String,
    pub duel_cannot_self: String,
    pub duel_already_in_match: String,
    pub target_already_in_match: String,
    pub queue_joined: String,
    pub queue_left: String,
    pub queue_already: String,
    pub queue_not_in: String,
    pub match_countdown: String,
    pub match_start: String,
    pub match_won: String,
    pub match_lost: String,
    pub match_broadcast_win: String,
    pub arena_created: String,
    pub arena_deleted: String,
    pub arena_already_exists: String,
    pub arena_spawn1_set: String,
    pub arena_spawn2_set: String,
    pub arena_spectator_set: String,
    pub arena_bounds_set: String,
    pub arena_empty_list: String,
    pub kit_created: String,
    pub kit_saved: String,
    pub kit_deleted: String,
    pub kit_already_exists: String,
    pub kit_empty_list: String,
    pub lobby_set: String,
    pub reload_success: String,
    pub player_not_found: String,
    pub arena_not_found: String,
    pub kit_not_found: String,
    pub no_permission: String,
    pub cannot_break_arena: String,
    pub only_players: String,
    pub command_help: String,
    pub admin_help: String,
    pub kit_renamed: String,
    pub kit_edited: String,
    pub not_in_match: String,
    pub match_surrendered: String,
    pub ping_self: String,
    pub ping_other: String,
    pub version_info: String,
    pub combat_nametag: String,
    pub combat_belowname: String,
}

impl Default for MessagesConfig {
    fn default() -> Self {
        Self {
            prefix: "<color:#E2B700>Duels</color> <color:#555555>»</color> ".to_string(),
            duel_sent: "<color:#A0A0A0>Duel request sent to</color> <color:#00E262>{target}</color> <color:#555555>•</color> <color:#A0A0A0>Kit:</color> <color:#E2B700>{kit}</color>".to_string(),
            duel_received: "<color:#00E262>{challenger}</color> <color:#A0A0A0>challenged you to a duel</color> <color:#555555>•</color> <color:#A0A0A0>Kit:</color> <color:#E2B700>{kit}</color> <color:#555555>|</color> <click:run_command:'/duel accept {challenger}'><hover:show_text:'Click to accept'><color:#00E262>[ACCEPT]</color></hover></click> <click:run_command:'/duel decline {challenger}'><hover:show_text:'Click to decline'><color:#E20000>[DECLINE]</color></hover></click>".to_string(),
            duel_accepted: "<color:#00E262>Duel request accepted!</color> <color:#A0A0A0>Teleporting to arena...</color>".to_string(),
            duel_declined: "<color:#A0A0A0>Declined duel request from</color> <color:#E2B700>{challenger}</color><color:#A0A0A0>.</color>".to_string(),
            duel_cancelled: "<color:#A0A0A0>Duel request to</color> <color:#E2B700>{target}</color> <color:#A0A0A0>was cancelled.</color>".to_string(),
            duel_expired: "<color:#E20000>The duel request has expired.</color>".to_string(),
            duel_cannot_self: "<color:#E20000>You cannot challenge yourself.</color>".to_string(),
            duel_already_in_match: "<color:#E20000>You are already in an active duel.</color>".to_string(),
            target_already_in_match: "<color:#E2B700>{target}</color> <color:#E20000>is currently in a duel.</color>".to_string(),
            queue_joined: "<color:#A0A0A0>Searching for opponent</color> <color:#555555>•</color> <color:#A0A0A0>Queue:</color> <color:#E2B700>{kit}</color>".to_string(),
            queue_left: "<color:#A0A0A0>Left the matchmaking queue.</color>".to_string(),
            queue_already: "<color:#E20000>You are already in a matchmaking queue.</color>".to_string(),
            queue_not_in: "<color:#E20000>You are not currently in a queue.</color>".to_string(),
            match_countdown: "<color:#A0A0A0>Starting in</color> <color:#00E262>{seconds}s</color><color:#A0A0A0>...</color>".to_string(),
            match_start: "<color:#00E262>FIGHT!</color>".to_string(),
            match_won: "<color:#00E262>VICTORY!</color> <color:#555555>•</color> <color:#A0A0A0>You defeated</color> <color:#E2B700>{loser}</color><color:#A0A0A0>.</color>".to_string(),
            match_lost: "<color:#E20000>DEFEAT!</color> <color:#555555>•</color> <color:#A0A0A0>You were defeated by</color> <color:#E2B700>{winner}</color><color:#A0A0A0>.</color>".to_string(),
            match_broadcast_win: "<color:#00E262>{winner}</color> <color:#A0A0A0>defeated</color> <color:#E20000>{loser}</color> <color:#555555>•</color> <color:#E2B700>{arena}</color> <color:#555555>({kit})</color>".to_string(),
            arena_created: "<color:#00E262>Created arena</color> <color:#E2B700>{name}</color><color:#00E262>.</color>".to_string(),
            arena_deleted: "<color:#A0A0A0>Deleted arena</color> <color:#E2B700>{name}</color><color:#A0A0A0>.</color>".to_string(),
            arena_already_exists: "<color:#E20000>Arena</color> <color:#E2B700>{name}</color> <color:#E20000>already exists.</color>".to_string(),
            arena_spawn1_set: "<color:#A0A0A0>Spawn 1 for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>".to_string(),
            arena_spawn2_set: "<color:#A0A0A0>Spawn 2 for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>".to_string(),
            arena_spectator_set: "<color:#A0A0A0>Spectator spawn for</color> <color:#E2B700>{name}</color> <color:#00E262>set to your position.</color>".to_string(),
            arena_bounds_set: "<color:#A0A0A0>Bounds for arena</color> <color:#E2B700>{name}</color> <color:#00E262>updated.</color>".to_string(),
            arena_empty_list: "<color:#A0A0A0>No arenas found in database.</color>".to_string(),
            kit_created: "<color:#00E262>Created kit</color> <color:#E2B700>{name}</color><color:#00E262>.</color>".to_string(),
            kit_saved: "<color:#00E262>Current inventory saved to kit</color> <color:#E2B700>{name}</color><color:#00E262>!</color>".to_string(),
            kit_deleted: "<color:#A0A0A0>Deleted kit</color> <color:#E2B700>{name}</color><color:#A0A0A0>.</color>".to_string(),
            kit_already_exists: "<color:#E20000>Kit</color> <color:#E2B700>{name}</color> <color:#E20000>already exists.</color>".to_string(),
            kit_empty_list: "<color:#A0A0A0>No kits found in database.</color>".to_string(),
            lobby_set: "<color:#00E262>Global duel lobby spawn set to your position.</color>".to_string(),
            reload_success: "<color:#00E262>Configuration and messages reloaded successfully.</color>".to_string(),
            player_not_found: "<color:#E20000>Player</color> <color:#E2B700>{player}</color> <color:#E20000>was not found online.</color>".to_string(),
            arena_not_found: "<color:#E20000>Arena</color> <color:#E2B700>{name}</color> <color:#E20000>was not found.</color>".to_string(),
            kit_not_found: "<color:#E20000>Kit</color> <color:#E2B700>{name}</color> <color:#E20000>was not found.</color>".to_string(),
            no_permission: "<color:#E20000>You do not have permission to execute this command.</color>".to_string(),
            cannot_break_arena: "<color:#E20000>You cannot modify blocks in the duel arena.</color>".to_string(),
            only_players: "<color:#E20000>This command can only be executed by a player in-game.</color>".to_string(),
            command_help: "<color:#E2B700>Commands</color> <color:#555555>»</color> <color:#A0A0A0>/duel <accept|deny|invite> <player>, /queue [kit], /leavequeue, /surrender, /ping, /duel stats, /duel top</color>".to_string(),
            admin_help: "<color:#E2B700>Admin Commands</color> <color:#555555>»</color> <color:#A0A0A0>/pd <version|reload|setlobby>, /arena <create|setspawn1|setspawn2|setspectator|setbounds|tp|leave|list|delete>, /kit <create|delete|edit|list|rename></color>".to_string(),
            kit_renamed: "<color:#00E262>Kit</color> <color:#E2B700>{old_name}</color> <color:#00E262>renamed to</color> {new_name}<color:#00E262>!</color>".to_string(),
            kit_edited: "<color:#00E262>Kit</color> <color:#E2B700>{name}</color> <color:#00E262>updated with current inventory!</color>".to_string(),
            not_in_match: "<color:#E20000>You are not currently in an active match.</color>".to_string(),
            match_surrendered: "<color:#E2B700>{player}</color> <color:#E20000>surrendered the match.</color>".to_string(),
            ping_self: "<color:#A0A0A0>Your ping:</color> <color:#00E262>{ping}ms</color>".to_string(),
            ping_other: "<color:#E2B700>{player}'s</color> <color:#A0A0A0>ping:</color> <color:#00E262>{ping}ms</color>".to_string(),
            version_info: "<color:#E2B700>Pumpkin-Duels</color> <color:#555555>•</color> <color:#00E262>v0.1.0</color> <color:#555555>•</color> <color:#A0A0A0>Pumpkin 0.2.0 (Protocol 777)</color>".to_string(),
            combat_nametag: "<color:#A0A0A0>{display_name}</color>".to_string(),
            combat_belowname: "<color:{color}>{heart} ❤</color>".to_string(),
        }
    }
}

pub struct MessageService {
    config_manager: Arc<ConfigManager<MessagesConfig>>,
}

impl MessageService {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, duels_config::ConfigError> {
        let manager = ConfigManager::load_or_create(path)?;
        Ok(Self {
            config_manager: Arc::new(manager),
        })
    }

    pub fn format(&self, template: &str, placeholders: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, val) in placeholders {
            result = result.replace(key, val);
        }
        format!("{}{}", self.config_manager.get().prefix, result)
    }

    pub fn format_raw(&self, template: &str, placeholders: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, val) in placeholders {
            result = result.replace(key, val);
        }
        result
    }

    pub fn component(&self, template: &str, placeholders: &[(&str, &str)]) -> TextComponent {
        crate::minimessage::parse_minimessage(&self.format(template, placeholders))
    }

    pub fn component_raw(&self, text: &str) -> TextComponent {
        crate::minimessage::parse_minimessage(text)
    }

    pub fn get_config(&self) -> MessagesConfig {
        self.config_manager.get()
    }

    pub fn reload(&self) -> Result<(), duels_config::ConfigError> {
        self.config_manager.reload()?;
        Ok(())
    }
}
