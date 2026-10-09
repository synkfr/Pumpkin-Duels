use crate::get_service;
use duels_core::match_state::MatchPhase;
use duels_gui::ClickContext;
use pumpkin_plugin_api::common::Position;
use pumpkin_plugin_api::events::block::{BlockBreakEvent, BlockPlaceEvent};
use pumpkin_plugin_api::events::entity::PlayerDeathEvent;
use pumpkin_plugin_api::events::player::{
    InventoryClickEvent, InventoryCloseEvent, PlayerDropItemEvent, PlayerLeaveEvent,
    PlayerMoveEvent, PlayerRespawnEvent,
};
use pumpkin_plugin_api::events::{EventHandler, EventPriority};
use pumpkin_plugin_api::events_wit::{
    BlockBreakEventData, BlockPlaceEventData, InventoryClickEventData, InventoryCloseEventData,
    PlayerDeathEventData, PlayerDropItemEventData, PlayerLeaveEventData, PlayerMoveEventData,
    PlayerRespawnEventData,
};
use pumpkin_plugin_api::{Context, Server};

struct InventoryClickListener;

impl EventHandler<InventoryClickEvent> for InventoryClickListener {
    fn handle(&self, server: Server, mut event: InventoryClickEventData) -> InventoryClickEventData {
        let player_uuid = event.player.get_id().to_string();
        let player_name = event.player.get_name();
        let service = get_service();

        if service.gui_registry.has_session(&player_uuid) {
            event.cancelled = true;

            let slot = if event.slot >= 0 {
                event.slot as u32
            } else {
                return event;
            };

            if let Some(player) = server.get_player_by_name(&player_name) {
                let ctx = ClickContext {
                    player,
                    server,
                    slot,
                    click_type: event.click_type,
                };
                service.gui_registry.handle_click(&player_uuid, ctx);
            }
        }

        event
    }
}

struct InventoryCloseListener;

impl EventHandler<InventoryCloseEvent> for InventoryCloseListener {
    fn handle(&self, _server: Server, event: InventoryCloseEventData) -> InventoryCloseEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();
        service.gui_registry.remove(&player_uuid);
        event
    }
}

struct PlayerMoveListener;

impl EventHandler<PlayerMoveEvent> for PlayerMoveListener {
    fn handle(&self, server: Server, mut event: PlayerMoveEventData) -> PlayerMoveEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        if let Some(session) = service.get_match_by_player(&player_uuid) {
            match session.phase {
                MatchPhase::Countdown { .. } => {
                    let is_p1 = session.player1_uuid == player_uuid;
                    let spawn = if is_p1 {
                        &session.arena.spawn1
                    } else {
                        &session.arena.spawn2
                    };

                    let spawn_pos: Position = (spawn.x, spawn.y, spawn.z);
                    event.to_position = spawn_pos;
                }
                MatchPhase::Fighting { .. } => {
                    let to_y = event.to_position.1;
                    if session.arena.bounds.is_ringout(to_y) {
                        let opponent_uuid = if session.player1_uuid == player_uuid {
                            session.player2_uuid.clone()
                        } else {
                            session.player1_uuid.clone()
                        };
                        service.end_match(&session.id, &opponent_uuid, &server);
                    }
                }
                MatchPhase::Ended { .. } => {}
            }
        }

        event
    }
}

struct BlockBreakListener;

impl EventHandler<BlockBreakEvent> for BlockBreakListener {
    fn handle(&self, _server: Server, mut event: BlockBreakEventData) -> BlockBreakEventData {
        if let Some(ref player) = event.player {
            let player_uuid = player.get_id().to_string();
            let service = get_service();

            if let Some(session) = service.get_match_by_player(&player_uuid)
                && !session.kit.rules.allow_build
            {
                event.cancelled = true;
                let msg = service.messages.get_config().cannot_break_arena;
                player.send_system_message(service.messages.component(&msg, &[]), true);
            }
        }
        event
    }
}

struct BlockPlaceListener;

impl EventHandler<BlockPlaceEvent> for BlockPlaceListener {
    fn handle(&self, _server: Server, mut event: BlockPlaceEventData) -> BlockPlaceEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        if let Some(session) = service.get_match_by_player(&player_uuid)
            && !session.kit.rules.allow_build
        {
            event.cancelled = true;
            let msg = service.messages.get_config().cannot_break_arena;
            event.player.send_system_message(service.messages.component(&msg, &[]), true);
        }

        event
    }
}

struct PlayerDropItemListener;

impl EventHandler<PlayerDropItemEvent> for PlayerDropItemListener {
    fn handle(&self, _server: Server, mut event: PlayerDropItemEventData) -> PlayerDropItemEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        if service.is_in_match(&player_uuid) {
            event.cancelled = true;
        }

        event
    }
}

struct PlayerDeathListener;

impl EventHandler<PlayerDeathEvent> for PlayerDeathListener {
    fn handle(&self, server: Server, mut event: PlayerDeathEventData) -> PlayerDeathEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        if let Some(session) = service.get_match_by_player(&player_uuid) {
            event.cancelled = true;
            event.keep_inventory = true;
            let winner_uuid = if session.player1_uuid == player_uuid {
                session.player2_uuid
            } else {
                session.player1_uuid
            };
            service.end_match(&session.id, &winner_uuid, &server);
        }

        event
    }
}

struct PlayerLeaveListener;

impl EventHandler<PlayerLeaveEvent> for PlayerLeaveListener {
    fn handle(&self, server: Server, event: PlayerLeaveEventData) -> PlayerLeaveEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        service.leave_queue(&event.player);

        if let Some(session) = service.get_match_by_player(&player_uuid) {
            let winner_uuid = if session.player1_uuid == player_uuid {
                session.player2_uuid
            } else {
                session.player1_uuid
            };
            service.end_match(&session.id, &winner_uuid, &server);
        }

        event
    }
}

struct PlayerRespawnListener;

impl EventHandler<PlayerRespawnEvent> for PlayerRespawnListener {
    fn handle(&self, server: Server, mut event: PlayerRespawnEventData) -> PlayerRespawnEventData {
        let service = get_service();
        let cfg = service.config.get();
        if let Some(lobby_world) = server
            .get_world_by_name(&cfg.lobby_world)
            .or_else(|| server.get_world_by_name("minecraft:overworld"))
            .or_else(|| server.get_all_worlds().into_iter().next())
        {
            event.respawned_world = lobby_world;
            event.position = (cfg.lobby_x, cfg.lobby_y, cfg.lobby_z);
            event.yaw = cfg.lobby_yaw;
            event.pitch = cfg.lobby_pitch;
        }
        event
    }
}

pub fn register_listeners(context: &Context) -> Result<(), String> {
    context.register_event_handler(
        InventoryClickListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        InventoryCloseListener,
        EventPriority::Normal,
        false,
    )?;

    context.register_event_handler(
        PlayerMoveListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        BlockBreakListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        BlockPlaceListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        PlayerDropItemListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        PlayerDeathListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        PlayerRespawnListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        PlayerLeaveListener,
        EventPriority::Normal,
        false,
    )?;

    Ok(())
}
