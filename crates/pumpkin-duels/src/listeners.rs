use crate::get_service;
use crate::ui::PendingBedrockForm;
use duels_core::match_state::MatchPhase;
use duels_gui::ClickContext;
use pumpkin_plugin_api::events::block::{BlockBreakEvent, BlockPlaceEvent};
use pumpkin_plugin_api::events::dialog::DialogClickActionEvent;
use pumpkin_plugin_api::events::entity::creature_spawn::CreatureSpawnEvent;
use pumpkin_plugin_api::events::entity::{EntityDamageEvent, PlayerDeathEvent};
use pumpkin_plugin_api::events::player::{
    BedrockFormResponseEvent, InventoryClickEvent, InventoryCloseEvent, PlayerDropItemEvent,
    PlayerInteractEvent, PlayerItemConsumeEvent, PlayerLeaveEvent, PlayerMoveEvent,
    PlayerRespawnEvent,
};
use pumpkin_plugin_api::events::{EventHandler, EventPriority};
use pumpkin_plugin_api::events_wit::{
    BedrockFormResponseEventData, BlockBreakEventData, BlockPlaceEventData, CreatureSpawnEventData,
    DialogClickActionEventData, EntityDamageEventData, InventoryClickEventData,
    InventoryCloseEventData, PlayerDeathEventData, PlayerDropItemEventData,
    PlayerInteractEventData, PlayerItemConsumeEventData, PlayerLeaveEventData, PlayerMoveEventData,
    PlayerRespawnEventData,
};
use pumpkin_plugin_api::forms::FormResponse;
use pumpkin_plugin_api::{Context, ItemStackExt, Server};

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
    fn handle(&self, server: Server, event: PlayerMoveEventData) -> PlayerMoveEventData {
        let player_uuid = event.player.get_id().to_string();
        let service = get_service();

        if let Some(session) = service.get_match_by_player(&player_uuid) {
            match session.phase {
                MatchPhase::Countdown { .. } => {
                    // Handled natively by player.set_movement_locked(true)
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

struct DialogClickActionListener;

impl EventHandler<DialogClickActionEvent> for DialogClickActionListener {
    fn handle(&self, server: Server, event: DialogClickActionEventData) -> DialogClickActionEventData {
        let player = &event.player;
        let id = &event.id;
        let service = get_service();

        if let Some(challenger) = id.strip_prefix("duel:accept:") {
            let _ = service.accept_challenge(player, Some(challenger), &server);
        } else if let Some(challenger) = id.strip_prefix("duel:decline:") {
            let _ = service.decline_challenge(player, Some(challenger), &server);
        } else if id == "duel:surrender:confirm" {
            let _ = service.surrender_match(player, &server);
        } else if id == "duel:surrender:cancel" {
            let msg = service.messages.component_raw("<color:#A0A0A0>Surrender cancelled.</color>");
            player.send_system_message(msg, true);
        } else if let Some(kit_name) = id.strip_prefix("duel:queue:") {
            let _ = service.enqueue(player, kit_name, false, &server);
        }

        event
    }
}

struct BedrockFormResponseListener;

impl EventHandler<BedrockFormResponseEvent> for BedrockFormResponseListener {
    fn handle(&self, server: Server, event: BedrockFormResponseEventData) -> BedrockFormResponseEventData {
        let player = &event.player;
        let p_uuid = player.get_id().to_string();
        let service = get_service();

        if let Some(pending) = crate::ui::take_bedrock_form(&p_uuid, event.form_id) {
            let response = FormResponse::parse(event.response_data.clone());
            match pending {
                PendingBedrockForm::Challenge { challenger_name } => {
                    match response {
                        FormResponse::Modal(true) => {
                            let _ = service.accept_challenge(player, Some(&challenger_name), &server);
                        }
                        FormResponse::Modal(false) => {
                            let _ = service.decline_challenge(player, Some(&challenger_name), &server);
                        }
                        _ => {}
                    }
                }
                PendingBedrockForm::Surrender => {
                    if let FormResponse::Modal(true) = response {
                        let _ = service.surrender_match(player, &server);
                    }
                }
                PendingBedrockForm::KitSelect { kits, target_player } => {
                    if let FormResponse::Simple(idx) = response {
                        if (idx as usize) < kits.len() {
                            let kit_name = &kits[idx as usize];
                            if let Some(target) = target_player {
                                if let Some(target_p) = server.get_player_by_name(&target) {
                                    let _ = service.send_challenge(player, &target_p, kit_name);
                                }
                            } else {
                                let _ = service.enqueue(player, kit_name, false, &server);
                            }
                        }
                    }
                }
                PendingBedrockForm::MatchResult { kit_name } => {
                    if let FormResponse::Simple(0) = response {
                        let _ = service.enqueue(player, &kit_name, false, &server);
                    }
                }
            }
        }

        event
    }
}

struct EntityDamageListener;

impl EventHandler<EntityDamageEvent> for EntityDamageListener {
    fn handle(&self, server: Server, mut event: EntityDamageEventData) -> EntityDamageEventData {
        let service = get_service();
        let entity_id = event.entity_id as u32;

        let active_match = {
            let matches = service.active_matches.lock().unwrap();
            matches.values().find_map(|session| {
                let p1 = server.get_player_by_name(&session.player1_name)?;
                let p2 = server.get_player_by_name(&session.player2_name)?;
                if p1.as_entity().get_id() == entity_id {
                    Some((session.clone(), p1, p2, false))
                } else if p2.as_entity().get_id() == entity_id {
                    Some((session.clone(), p2, p1, true))
                } else {
                    None
                }
            })
        };

        if let Some((session, victim, attacker, _)) = active_match {
            match session.phase {
                MatchPhase::Countdown { .. } | MatchPhase::Ended { .. } => {
                    event.cancelled = true;
                }
                MatchPhase::Fighting { .. } => {
                    let new_hp = (victim.get_health() - event.damage).max(0.0);
                    let world = victim.get_world();
                    crate::ui::update_combat_health(&victim, new_hp, Some(&attacker), &world);
                    crate::ui::update_combat_health(&attacker, attacker.get_health(), Some(&victim), &world);

                    let (vx, vy, vz) = victim.get_position();
                    victim.spawn_particles(
                        pumpkin_plugin_api::particles::Particle::DamageIndicator,
                        (vx, vy + 1.0, vz),
                        3,
                        (0.3, 0.3, 0.3),
                        0.1,
                    );
                    victim.spawn_particles(
                        pumpkin_plugin_api::particles::Particle::Crit,
                        (vx, vy + 1.0, vz),
                        4,
                        (0.3, 0.3, 0.3),
                        0.1,
                    );
                }
            }
        }

        event
    }
}

struct PlayerInteractListener;

impl EventHandler<PlayerInteractEvent> for PlayerInteractListener {
    fn handle(&self, _server: Server, event: PlayerInteractEventData) -> PlayerInteractEventData {
        let player = &event.player;
        let player_uuid = player.get_id().to_string();
        let service = get_service();

        if !service.is_in_match(&player_uuid) && event.block == "minecraft:air" {
            let inv = player.get_inventory().as_inventory();
            if let Some(item) = inv.get_item(0) {
                if let Some(it) = item.get_item() {
                    if it.as_str().ends_with("_sword") {
                        let _ = service.open_kit_selector(player, None);
                    }
                }
            }
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

    context.register_event_handler(
        DialogClickActionListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        BedrockFormResponseListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        EntityDamageListener,
        EventPriority::High,
        true,
    )?;

    context.register_event_handler(
        PlayerInteractListener,
        EventPriority::Normal,
        false,
    )?;

    context.register_event_handler(
        PlayerItemConsumeListener,
        EventPriority::Normal,
        false,
    )?;

    context.register_event_handler(
        CreatureSpawnListener,
        EventPriority::High,
        true,
    )?;

    Ok(())
}

struct CreatureSpawnListener;

impl EventHandler<CreatureSpawnEvent> for CreatureSpawnListener {
    fn handle(&self, _server: Server, mut event: CreatureSpawnEventData) -> CreatureSpawnEventData {
        let world_name = event.target_world.get_name();
        if world_name.starts_with("duel_") || world_name.starts_with("arena_") {
            event.cancelled = true;
        }
        event
    }
}

struct PlayerItemConsumeListener;

impl EventHandler<PlayerItemConsumeEvent> for PlayerItemConsumeListener {
    fn handle(&self, server: Server, event: PlayerItemConsumeEventData) -> PlayerItemConsumeEventData {
        let p_uuid = event.player.get_id().to_string();
        let service = get_service();
        if let Some(session) = service.get_match_by_player(&p_uuid) {
            let opp_name = if session.player1_uuid == p_uuid {
                &session.player2_name
            } else {
                &session.player1_name
            };
            let opp = server.get_player_by_name(opp_name);
            crate::ui::update_combat_health(
                &event.player,
                event.player.get_health(),
                opp.as_ref(),
                &event.player.get_world(),
            );
        }
        event
    }
}
