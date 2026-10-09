use crate::get_service;
use duels_core::arena::{ArenaBounds, ArenaLocation, ArenaTemplate};
use duels_core::kit::Kit;
use pumpkin_plugin_api::command::{
    Arg, ArgumentType, Command, CommandError, CommandNode, CommandSender, ConsumedArgs, StringType,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};

const PERM_USER: &str = "pumpkin-duels:duels.use";
const PERM_ADMIN: &str = "pumpkin-duels:duels.admin";

fn send_feedback(sender: &CommandSender, server: &Server, component: TextComponent) {
    if let Some(player) = sender.as_player().or_else(|| server.get_player_by_name(&sender.get_name())) {
        player.send_system_message(component, false);
        crate::service::play_ui_click(&player);
    } else {
        sender.send_message(component);
    }
}

fn send_feedback_success(sender: &CommandSender, server: &Server, component: TextComponent) {
    if let Some(player) = sender.as_player().or_else(|| server.get_player_by_name(&sender.get_name())) {
        player.send_system_message(component, false);
        crate::service::play_sound_success(&player);
    } else {
        sender.send_message(component);
    }
}

fn send_feedback_error(sender: &CommandSender, server: &Server, component: TextComponent) {
    if let Some(player) = sender.as_player().or_else(|| server.get_player_by_name(&sender.get_name())) {
        player.send_system_message(component, false);
        crate::service::play_sound_error(&player);
    } else {
        sender.send_message(component);
    }
}

fn check_admin_permission(sender: &CommandSender, server: &Server) -> bool {
    if sender.is_console() {
        return true;
    }
    sender.has_permission_level(pumpkin_plugin_api::command_wit::PermissionLevel::Two)
        || sender.has_permission_level(pumpkin_plugin_api::command_wit::PermissionLevel::Three)
        || sender.has_permission_level(pumpkin_plugin_api::command_wit::PermissionLevel::Four)
        || sender.has_permission(server, PERM_ADMIN)
}

struct DuelHelpHandler;
impl CommandHandler for DuelHelpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        let msg = service.messages.get_config().command_help;
        send_feedback(&sender, &server, service.messages.component(&msg, &[]));
        Ok(0)
    }
}

struct DuelRootHandler;
impl CommandHandler for DuelRootHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player_name = match args.get_value("player") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if player.get_name() == player_name {
            let msg = service.messages.get_config().duel_cannot_self;
            player.send_system_message(service.messages.component(&msg, &[]), false);
            return Ok(0);
        }

        if let Some(target) = server.get_player_by_name(&player_name) {
            let _ = service.open_kit_selector(&player, Some(target.get_name()));
        } else {
            let msg = service.messages.get_config().player_not_found;
            player.send_system_message(
                service.messages.component(&msg, &[("{player}", &player_name)]),
                false,
            );
        }

        Ok(0)
    }
}

struct DuelAcceptHandler;
impl CommandHandler for DuelAcceptHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player_name = match args.get_value("player") {
            Arg::Simple(s) | Arg::Msg(s) => Some(s),
            _ => None,
        };

        let service = get_service();
        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Err(err_msg) = service.accept_challenge(&player, player_name.as_deref(), &server) {
            player.send_system_message(service.messages.component_raw(&err_msg), false);
            crate::service::play_sound_error(&player);
        }

        Ok(0)
    }
}

struct DuelDeclineHandler;
impl CommandHandler for DuelDeclineHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player_name = match args.get_value("player") {
            Arg::Simple(s) | Arg::Msg(s) => Some(s),
            _ => None,
        };

        let service = get_service();
        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Err(err_msg) = service.decline_challenge(&player, player_name.as_deref(), &server) {
            player.send_system_message(service.messages.component_raw(&err_msg), false);
            crate::service::play_sound_error(&player);
        }

        Ok(0)
    }
}

struct DuelQueueHandler;
impl CommandHandler for DuelQueueHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if let Some(player) = server.get_player_by_name(&sender.get_name()) {
            let _ = service.open_kit_selector(&player, None);
        } else {
            let msg = service.messages.get_config().only_players;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
        }
        Ok(0)
    }
}

struct DuelQueueKitHandler;
impl CommandHandler for DuelQueueKitHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("kit") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        match service.storage.get_kit(&kit_name) {
            Ok(Some(_)) => {
                let _ = service.enqueue(&player, &kit_name, false, &server);
            }
            _ => {
                let msg = service.messages.get_config().kit_not_found;
                player.send_system_message(
                    service.messages.component(&msg, &[("{name}", &kit_name)]),
                    false,
                );
            }
        }

        Ok(0)
    }
}

struct DuelLeaveHandler;
impl CommandHandler for DuelLeaveHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if let Some(player) = server.get_player_by_name(&sender.get_name()) {
            let p_uuid = player.get_id().to_string();
            if let Some(session) = service.get_match_by_player(&p_uuid) {
                let winner_uuid = if session.player1_uuid == p_uuid {
                    session.player2_uuid
                } else {
                    session.player1_uuid
                };
                service.end_match(&session.id, &winner_uuid, &server);
                return Ok(0);
            }
            service.leave_queue(&player);
        } else {
            let msg = service.messages.get_config().only_players;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
        }
        Ok(0)
    }
}

struct DuelStatsHandler;
impl CommandHandler for DuelStatsHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let target_name = match args.get_value("player") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => sender.get_name(),
        };

        let service = get_service();
        let target_player = server.get_player_by_name(&target_name);

        let (uuid, name) = if let Some(p) = target_player {
            (p.get_id().to_string(), p.get_name())
        } else {
            (target_name.clone(), target_name.clone())
        };

        if let Ok(stats) = service.storage.get_or_create_stats(&uuid, &name) {
            let msg = format!(
                "<color:#E2B700>Duel Stats: <color:#FFFFFF>{}</color></color>\n<color:#A0A0A0>Wins:</color> <color:#00E262>{}</color>\n<color:#A0A0A0>Losses:</color> <color:#E20000>{}</color>\n<color:#A0A0A0>Current Streak:</color> <color:#E2B700>{}</color>\n<color:#A0A0A0>Best Streak:</color> <color:#E2B700>{}</color>\n<color:#A0A0A0>Elo:</color> <color:#00E262>{}</color>",
                stats.name, stats.wins, stats.losses, stats.current_streak, stats.best_streak, stats.elo
            );
            send_feedback(&sender, &server, service.messages.component_raw(&msg));
        }

        Ok(0)
    }
}

struct DuelTopHandler;
impl CommandHandler for DuelTopHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if let Ok(top_players) = service.storage.get_top_players(10) {
            let mut msg = "<color:#E2B700>Top Players (Elo):</color>\n".to_string();
            for (idx, p) in top_players.iter().enumerate() {
                msg.push_str(&format!(
                    "<color:#A0A0A0>#{}. <color:#FFFFFF>{}</color> - <color:#00E262>{}</color> Elo (<color:#00E262>{}W</color>/<color:#E20000>{}L</color>)</color>\n",
                    idx + 1,
                    p.name,
                    p.elo,
                    p.wins,
                    p.losses
                ));
            }
            send_feedback(&sender, &server, service.messages.component_raw(&msg));
        }
        Ok(0)
    }
}

struct AdminHelpHandler;
impl CommandHandler for AdminHelpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }
        let msg = service.messages.get_config().admin_help;
        send_feedback(&sender, &server, service.messages.component(&msg, &[]));
        Ok(0)
    }
}

struct AdminArenaCreateHandler;
impl CommandHandler for AdminArenaCreateHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let dimension = match args.get_value("dimension") {
            Arg::Simple(s) | Arg::Msg(s) => match s.to_lowercase().as_str() {
                "nether" | "the_nether" => pumpkin_plugin_api::server::Dimension::Nether,
                "end" | "the_end" => pumpkin_plugin_api::server::Dimension::End,
                _ => pumpkin_plugin_api::server::Dimension::Overworld,
            },
            _ => pumpkin_plugin_api::server::Dimension::Overworld,
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(_)) = service.storage.get_arena(&arena_name) {
            let msg = service.messages.get_config().arena_already_exists;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
            return Ok(0);
        }

        let (world_name, world) =
            crate::arena_world::create_arena_void_world(&server, &arena_name, dimension);

        player.teleport((0.0, 65.0, 0.0), Some(0.0), Some(0.0), world);

        let template = ArenaTemplate {
            name: arena_name.clone(),
            template_world: world_name.clone(),
            spawn1: ArenaLocation {
                x: -5.0,
                y: 65.0,
                z: 0.0,
                yaw: -90.0,
                pitch: 0.0,
            },
            spawn2: ArenaLocation {
                x: 5.0,
                y: 65.0,
                z: 0.0,
                yaw: 90.0,
                pitch: 0.0,
            },
            spectator_spawn: ArenaLocation {
                x: 0.0,
                y: 70.0,
                z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
            },
            bounds: ArenaBounds {
                min_x: -30,
                max_x: 30,
                min_z: -30,
                max_z: 30,
                min_y: 45,
                max_y: 120,
                ringout_y: Some(55.0),
            },
            enabled: true,
        };

        let _ = service.storage.save_arena(&template.into());
        let raw_msg = format!(
            "<color:#00E262>Void arena <color:#E2B700>{arena_name}</color> created! World: <color:#E2B700>{world_name}</color>. Build your arena, then run <color:#00E262>/arena setspawn1 {arena_name}</color> and <color:#00E262>/arena setspawn2 {arena_name}</color>.</color>"
        );
        player.send_system_message(service.messages.component_raw(&raw_msg), false);

        Ok(0)
    }
}

struct AdminArenaSetSpawn1Handler;
impl CommandHandler for AdminArenaSetSpawn1Handler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(mut arena)) = service.storage.get_arena(&arena_name) {
            let pos = player.get_position();
            arena.spawn1.x = pos.0;
            arena.spawn1.y = pos.1;
            arena.spawn1.z = pos.2;
            arena.spawn1.yaw = player.get_yaw();
            arena.spawn1.pitch = player.get_pitch();
            let _ = service.storage.save_arena(&arena);

            let msg = service.messages.get_config().arena_spawn1_set;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetSpawn2Handler;
impl CommandHandler for AdminArenaSetSpawn2Handler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(mut arena)) = service.storage.get_arena(&arena_name) {
            let pos = player.get_position();
            arena.spawn2.x = pos.0;
            arena.spawn2.y = pos.1;
            arena.spawn2.z = pos.2;
            arena.spawn2.yaw = player.get_yaw();
            arena.spawn2.pitch = player.get_pitch();
            let _ = service.storage.save_arena(&arena);

            let msg = service.messages.get_config().arena_spawn2_set;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetSpectatorHandler;
impl CommandHandler for AdminArenaSetSpectatorHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(mut arena)) = service.storage.get_arena(&arena_name) {
            let pos = player.get_position();
            arena.spectator_spawn.x = pos.0;
            arena.spectator_spawn.y = pos.1;
            arena.spectator_spawn.z = pos.2;
            arena.spectator_spawn.yaw = player.get_yaw();
            arena.spectator_spawn.pitch = player.get_pitch();
            let _ = service.storage.save_arena(&arena);

            let msg = service.messages.get_config().arena_spectator_set;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetBoundsHandler;
impl CommandHandler for AdminArenaSetBoundsHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let radius = match args.get_value("radius") {
            Arg::Simple(s) | Arg::Msg(s) => s.parse::<i32>().unwrap_or(30),
            _ => 30,
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(mut arena)) = service.storage.get_arena(&arena_name) {
            let (px, py, pz) = player.get_position();
            arena.bounds.min_x = (px - radius as f64) as i32;
            arena.bounds.max_x = (px + radius as f64) as i32;
            arena.bounds.min_z = (pz - radius as f64) as i32;
            arena.bounds.max_z = (pz + radius as f64) as i32;
            arena.bounds.min_y = (py - 20.0) as i32;
            arena.bounds.max_y = (py + 60.0) as i32;
            arena.bounds.ringout_y = Some(py - 5.0);
            let _ = service.storage.save_arena(&arena);

            let msg = service.messages.get_config().arena_bounds_set;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaTpHandler;
impl CommandHandler for AdminArenaTpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(arena)) = service.storage.get_arena(&arena_name) {
            let arena_world = server
                .get_world_by_name(&arena.template_world)
                .unwrap_or_else(|| {
                    server.create_world(
                        &arena.template_world,
                        pumpkin_plugin_api::server::Dimension::Overworld,
                    )
                });

            player.teleport(
                (arena.spectator_spawn.x, arena.spectator_spawn.y, arena.spectator_spawn.z),
                Some(arena.spectator_spawn.yaw),
                Some(arena.spectator_spawn.pitch),
                arena_world,
            );

            let raw_msg = format!("<color:#00E262>Teleported to arena <color:#E2B700>{arena_name}</color>.</color>");
            player.send_system_message(service.messages.component_raw(&raw_msg), false);
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaLeaveHandler;
impl CommandHandler for AdminArenaLeaveHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        service.teleport_lobby(&player, &server);
        let raw_msg = "<color:#00E262>Teleported to duel lobby.</color>";
        player.send_system_message(service.messages.component_raw(raw_msg), false);
        Ok(0)
    }
}

struct AdminArenaDeleteHandler;
impl CommandHandler for AdminArenaDeleteHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let arena_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        if let Ok(Some(_)) = service.storage.get_arena(&arena_name) {
            let _ = service.storage.delete_arena(&arena_name);
            let msg = service.messages.get_config().arena_deleted;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaListHandler;
impl CommandHandler for AdminArenaListHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        match service.storage.list_arenas() {
            Ok(arenas) if !arenas.is_empty() => {
                let mut msg = "<color:#E2B700>Configured Arenas:</color>\n".to_string();
                for a in arenas {
                    msg.push_str(&format!(
                        "<color:#00E262>•</color> <color:#A0A0A0>{}</color> <color:#707070>(world: {}, enabled: {})</color>\n",
                        a.name, a.template_world, a.enabled
                    ));
                }
                send_feedback(&sender, &server, service.messages.component_raw(&msg));
            }
            _ => {
                let msg = service.messages.get_config().arena_empty_list;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            }
        }
        Ok(0)
    }
}

struct AdminKitSaveHandler;
impl CommandHandler for AdminKitSaveHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        let mut kit = Kit::classic();
        kit.name = kit_name.clone();

        let player_inv = player.get_inventory();
        let inv = player_inv.as_inventory();

        kit.items.clear();
        for slot in 0..36 {
            if let Some(item) = inv.get_item(slot) {
                kit.items.push(duels_core::kit::KitItem {
                    slot,
                    id: item.get_registry_key(),
                    count: item.get_count(),
                });
            }
        }

        kit.helmet = player_inv.get_helmet().map(|i| i.get_registry_key());
        kit.chestplate = player_inv.get_chestplate().map(|i| i.get_registry_key());
        kit.leggings = player_inv.get_leggings().map(|i| i.get_registry_key());
        kit.boots = player_inv.get_boots().map(|i| i.get_registry_key());
        kit.offhand = player_inv.get_off_hand().map(|i| i.get_registry_key());

        if let Ok(rec) = kit.to_record() {
            let _ = service.storage.save_kit(&rec);
            let msg = service.messages.get_config().kit_saved;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &kit_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminKitDeleteHandler;
impl CommandHandler for AdminKitDeleteHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        if let Ok(Some(_)) = service.storage.get_kit(&kit_name) {
            let _ = service.storage.delete_kit(&kit_name);
            let msg = service.messages.get_config().kit_deleted;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &kit_name)]),
            );
        } else {
            let msg = service.messages.get_config().kit_not_found;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &kit_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminKitListHandler;
impl CommandHandler for AdminKitListHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        match service.storage.list_kits() {
            Ok(kits) if !kits.is_empty() => {
                let mut msg = "<color:#E2B700>Configured Kits:</color>\n".to_string();
                for k in kits {
                    msg.push_str(&format!(
                        "<color:#00E262>•</color> <color:#A0A0A0>{}</color> <color:#707070>(icon: {}, enabled: {})</color>\n",
                        k.name, k.display_item, k.enabled
                    ));
                }
                send_feedback(&sender, &server, service.messages.component_raw(&msg));
            }
            _ => {
                let msg = service.messages.get_config().kit_empty_list;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            }
        }
        Ok(0)
    }
}

struct AdminSetLobbyHandler;
impl CommandHandler for AdminSetLobbyHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        if let Some(player) = server.get_player_by_name(&sender.get_name()) {
            let pos = player.get_position();
            let world_name = player.get_world().get_name();
            let yaw = player.get_yaw();
            let pitch = player.get_pitch();
            let _ = service.config.update(|cfg| {
                cfg.lobby_world = world_name;
                cfg.lobby_x = pos.0;
                cfg.lobby_y = pos.1;
                cfg.lobby_z = pos.2;
                cfg.lobby_yaw = yaw;
                cfg.lobby_pitch = pitch;
            });
            let msg = service.messages.get_config().lobby_set;
            send_feedback_success(&sender, &server, service.messages.component(&msg, &[]));
        } else {
            let msg = service.messages.get_config().only_players;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
        }
        Ok(0)
    }
}

struct AdminReloadHandler;
impl CommandHandler for AdminReloadHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let _ = service.config.reload();
        let _ = service.messages.reload();
        let msg = service.messages.get_config().reload_success;
        send_feedback_success(&sender, &server, service.messages.component(&msg, &[]));
        Ok(0)
    }
}

fn build_arena_node() -> CommandNode {
    CommandNode::literal("arena")
        .execute(AdminHelpHandler)
        .then(
            CommandNode::literal("create").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaCreateHandler)
                    .then(
                        CommandNode::argument("dimension", &ArgumentType::String(StringType::SingleWord))
                            .execute(AdminArenaCreateHandler),
                    ),
            ),
        )
        .then(
            CommandNode::literal("setspawn1").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpawn1Handler),
            ),
        )
        .then(
            CommandNode::literal("setspawn2").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpawn2Handler),
            ),
        )
        .then(
            CommandNode::literal("setspectator").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpectatorHandler),
            ),
        )
        .then(
            CommandNode::literal("setbounds").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetBoundsHandler)
                    .then(
                        CommandNode::argument("radius", &ArgumentType::String(StringType::SingleWord))
                            .execute(AdminArenaSetBoundsHandler),
                    ),
            ),
        )
        .then(
            CommandNode::literal("tp").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaTpHandler),
            ),
        )
        .then(CommandNode::literal("leave").execute(AdminArenaLeaveHandler))
        .then(
            CommandNode::literal("delete").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaDeleteHandler),
            ),
        )
        .then(CommandNode::literal("list").execute(AdminArenaListHandler))
}

fn build_arena_tree() -> Command {
    let aliases = ["arena".to_string(), "arenas".to_string()];
    Command::new(&aliases, "Arena management command")
        .execute(AdminHelpHandler)
        .then(
            CommandNode::literal("create").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaCreateHandler)
                    .then(
                        CommandNode::argument("dimension", &ArgumentType::String(StringType::SingleWord))
                            .execute(AdminArenaCreateHandler),
                    ),
            ),
        )
        .then(
            CommandNode::literal("setspawn1").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpawn1Handler),
            ),
        )
        .then(
            CommandNode::literal("setspawn2").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpawn2Handler),
            ),
        )
        .then(
            CommandNode::literal("setspectator").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetSpectatorHandler),
            ),
        )
        .then(
            CommandNode::literal("setbounds").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaSetBoundsHandler)
                    .then(
                        CommandNode::argument("radius", &ArgumentType::String(StringType::SingleWord))
                            .execute(AdminArenaSetBoundsHandler),
                    ),
            ),
        )
        .then(
            CommandNode::literal("tp").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaTpHandler),
            ),
        )
        .then(CommandNode::literal("leave").execute(AdminArenaLeaveHandler))
        .then(
            CommandNode::literal("delete").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminArenaDeleteHandler),
            ),
        )
        .then(CommandNode::literal("list").execute(AdminArenaListHandler))
}

fn build_kit_node() -> CommandNode {
    CommandNode::literal("kit")
        .execute(AdminHelpHandler)
        .then(
            CommandNode::literal("save").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminKitSaveHandler),
            ),
        )
        .then(
            CommandNode::literal("delete").then(
                CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                    .execute(AdminKitDeleteHandler),
            ),
        )
        .then(CommandNode::literal("list").execute(AdminKitListHandler))
}

fn build_admin_node() -> CommandNode {
    CommandNode::literal("admin")
        .execute(AdminHelpHandler)
        .then(build_arena_node())
        .then(build_kit_node())
        .then(CommandNode::literal("setlobby").execute(AdminSetLobbyHandler))
        .then(CommandNode::literal("reload").execute(AdminReloadHandler))
}

fn build_duel_tree() -> Command {
    let duel_aliases = ["duel".to_string(), "duels".to_string(), "1v1".to_string()];
    Command::new(&duel_aliases, "Practice duel command")
        .execute(DuelHelpHandler)
        .then(
            CommandNode::literal("accept")
                .execute(DuelAcceptHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelAcceptHandler),
                ),
        )
        .then(
            CommandNode::literal("decline")
                .execute(DuelDeclineHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelDeclineHandler),
                ),
        )
        .then(
            CommandNode::literal("queue")
                .execute(DuelQueueHandler)
                .then(
                    CommandNode::argument("kit", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelQueueKitHandler),
                ),
        )
        .then(CommandNode::literal("leave").execute(DuelLeaveHandler))
        .then(
            CommandNode::literal("stats")
                .execute(DuelStatsHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelStatsHandler),
                ),
        )
        .then(CommandNode::literal("top").execute(DuelTopHandler))
        .then(build_admin_node())
        .then(
            CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                .execute(DuelRootHandler),
        )
}

pub fn register_commands(context: &Context) {
    let user_perm = Permission {
        node: PERM_USER.to_string(),
        description: "Allows players to use duel commands".to_string(),
        default: PermissionDefault::Allow,
        children: vec![],
    };
    if let Err(e) = context.register_permission(&user_perm) {
        tracing::error!("Failed to register permission {}: {e}", PERM_USER);
    } else {
        tracing::info!("Registered permission {}", PERM_USER);
    }

    let admin_perm = Permission {
        node: PERM_ADMIN.to_string(),
        description: "Allows administrators to configure duels".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Two),
        children: vec![],
    };
    if let Err(e) = context.register_permission(&admin_perm) {
        tracing::error!("Failed to register permission {}: {e}", PERM_ADMIN);
    } else {
        tracing::info!("Registered permission {}", PERM_ADMIN);
    }

    context.register_command(build_duel_tree(), PERM_USER);
    context.register_command(build_arena_tree(), PERM_ADMIN);

    let admin_aliases = [
        "dueladmin".to_string(),
        "duelsadmin".to_string(),
        "da".to_string(),
    ];
    let admin_cmd = Command::new(&admin_aliases, "Duels administration command")
        .execute(AdminHelpHandler)
        .then(build_arena_node())
        .then(build_kit_node())
        .then(CommandNode::literal("setlobby").execute(AdminSetLobbyHandler))
        .then(CommandNode::literal("reload").execute(AdminReloadHandler));
    context.register_command(admin_cmd, PERM_ADMIN);

    let queue_aliases = ["queue".to_string(), "joinqueue".to_string()];
    let queue_cmd = Command::new(&queue_aliases, "Queue for a duel")
        .execute(DuelQueueHandler)
        .then(
            CommandNode::argument("kit", &ArgumentType::String(StringType::SingleWord))
                .execute(DuelQueueKitHandler),
        );
    context.register_command(queue_cmd, PERM_USER);

    let leave_aliases = ["leavequeue".to_string(), "unqueue".to_string()];
    let leave_cmd = Command::new(&leave_aliases, "Leave duel queue").execute(DuelLeaveHandler);
    context.register_command(leave_cmd, PERM_USER);
}
