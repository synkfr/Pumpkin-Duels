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

fn capture_player_kit(player: &pumpkin_plugin_api::player::Player, name: String) -> Kit {
    let mut kit = Kit::classic();
    kit.name = name;

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
    kit
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

struct DuelInviteUsageHandler;
impl CommandHandler for DuelInviteUsageHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let raw = "<color:#E20000>Usage: /duel invite <player> [kit]</color>";
        send_feedback_error(&sender, &server, get_service().messages.component_raw(raw));
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
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if player.get_name() == player_name {
            let msg = service.messages.get_config().duel_cannot_self;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        if let Some(target) = server.get_player_by_name(&player_name) {
            let _ = service.open_kit_selector(&player, Some(target.get_name()));
        } else {
            let msg = service.messages.get_config().player_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{player}", &player_name)]),
            );
        }

        Ok(0)
    }
}

struct DuelInviteKitHandler;
impl CommandHandler for DuelInviteKitHandler {
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
        let kit_query = match args.get_value("kit") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => return Ok(0),
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

        if player.get_name() == player_name {
            let msg = service.messages.get_config().duel_cannot_self;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let target = match server.get_player_by_name(&player_name) {
            Some(t) => t,
            None => {
                let msg = service.messages.get_config().player_not_found;
                send_feedback_error(
                    &sender,
                    &server,
                    service.messages.component(&msg, &[("{player}", &player_name)]),
                );
                return Ok(0);
            }
        };

        let kit = match service.find_kit(&kit_query) {
            Ok(Some(k)) => k,
            _ => {
                let msg = service.messages.get_config().kit_not_found;
                send_feedback_error(
                    &sender,
                    &server,
                    service.messages.component(&msg, &[("{name}", &kit_query)]),
                );
                return Ok(0);
            }
        };

        if let Err(err_msg) = service.send_challenge(&player, &target, &kit.name) {
            player.send_system_message(service.messages.component_raw(&err_msg), false);
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
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        match service.find_kit(&kit_name) {
            Ok(Some(k)) => {
                let _ = service.enqueue(&player, &k.name, false, &server);
            }
            _ => {
                let msg = service.messages.get_config().kit_not_found;
                send_feedback_error(
                    &sender,
                    &server,
                    service.messages.component(&msg, &[("{name}", &kit_name)]),
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
        let target_uuid = if let Some(target) = server.get_player_by_name(&target_name) {
            target.get_id().to_string()
        } else {
            match service.storage.get_stats_by_name(&target_name) {
                Ok(Some(st)) => st.uuid,
                _ => {
                    let msg = service.messages.get_config().player_not_found;
                    send_feedback_error(
                        &sender,
                        &server,
                        service.messages.component(&msg, &[("{player}", &target_name)]),
                    );
                    return Ok(0);
                }
            }
        };

        if let Ok(stats) = service.storage.get_or_create_stats(&target_uuid, &target_name) {
            let winrate = if stats.wins + stats.losses > 0 {
                (stats.wins as f64 / (stats.wins + stats.losses) as f64) * 100.0
            } else {
                0.0
            };

            let raw_msg = format!(
                "<color:#E2B700>Stats</color> <color:#555555>»</color> <color:#00E262>{}</color>\n<color:#A0A0A0>ELO:</color> <color:#00E262>{}</color>\n<color:#A0A0A0>Wins:</color> <color:#00E262>{}</color> <color:#555555>•</color> <color:#A0A0A0>Losses:</color> <color:#E20000>{}</color>\n<color:#A0A0A0>Win Rate:</color> <color:#E2B700>{:.1}%</color>\n<color:#A0A0A0>Streak:</color> <color:#00E262>{}</color> <color:#555555>(Best: {})</color>",
                stats.name, stats.elo, stats.wins, stats.losses, winrate, stats.current_streak, stats.best_streak
            );
            send_feedback(&sender, &server, service.messages.component_raw(&raw_msg));
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
        match service.storage.get_top_players(10) {
            Ok(top) if !top.is_empty() => {
                let mut msg = "<color:#E2B700>Leaderboard</color> <color:#555555>»</color> <color:#A0A0A0>Top 10</color>\n".to_string();
                for (rank, p) in top.iter().enumerate() {
                    msg.push_str(&format!(
                        "<color:#555555>{}.</color> <color:#00E262>{}</color> <color:#555555>•</color> <color:#E2B700>{} ELO</color> <color:#555555>({}W / {}L)</color>\n",
                        rank + 1, p.name, p.elo, p.wins, p.losses
                    ));
                }
                send_feedback(&sender, &server, service.messages.component_raw(&msg));
            }
            _ => {
                send_feedback(
                    &sender,
                    &server,
                    service.messages.component_raw("<color:#E2B700>No ranked stats recorded yet.</color>"),
                );
            }
        }
        Ok(0)
    }
}

struct SurrenderHandler;
impl CommandHandler for SurrenderHandler {
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
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if !service.is_in_match(&player.get_id().to_string()) {
            send_feedback_error(
                &sender,
                &server,
                service.messages.component_raw("<color:#E20000>You are not currently in an active match.</color>"),
            );
            return Ok(0);
        }

        crate::ui::show_surrender_confirmation(&player);
        let prompt_msg = "<color:#A0A0A0>Please confirm forfeit in the dialog or run</color> <color:#00E262>/surrender confirm</color>";
        player.send_system_message(service.messages.component_raw(prompt_msg), false);
        Ok(0)
    }
}

struct SurrenderConfirmHandler;
impl CommandHandler for SurrenderConfirmHandler {
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
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Err(err_msg) = service.surrender_match(&player, &server) {
            send_feedback_error(&sender, &server, service.messages.component_raw(&err_msg));
        }
        Ok(0)
    }
}

struct PingHandler;
impl CommandHandler for PingHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        let target_name = match args.get_value("player") {
            Arg::Simple(s) | Arg::Msg(s) => Some(s),
            _ => None,
        };

        if let Some(target_name) = target_name {
            if let Some(target) = server.get_player_by_name(&target_name) {
                let ping = target.get_ping().to_string();
                let msg = service.messages.get_config().ping_other;
                send_feedback(
                    &sender,
                    &server,
                    service.messages.component(
                        &msg,
                        &[("{player}", &target.get_name()), ("{ping}", &ping)],
                    ),
                );
            } else {
                let msg = service.messages.get_config().player_not_found;
                send_feedback_error(
                    &sender,
                    &server,
                    service.messages.component(&msg, &[("{player}", &target_name)]),
                );
            }
        } else if let Some(player) = server.get_player_by_name(&sender.get_name()) {
            let ping = player.get_ping().to_string();
            let msg = service.messages.get_config().ping_self;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{ping}", &ping)]),
            );
        } else {
            send_feedback(
                &sender,
                &server,
                service.messages.component_raw("<color:#A0A0A0>Console ping: <color:#00E262>0ms</color></color>"),
            );
        }
        Ok(0)
    }
}

struct PdHelpHandler;
impl CommandHandler for PdHelpHandler {
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
        let msg = service.messages.get_config().admin_help;
        send_feedback(&sender, &server, service.messages.component(&msg, &[]));
        Ok(0)
    }
}

struct PdVersionHandler;
impl CommandHandler for PdVersionHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let service = get_service();
        let msg = service.messages.get_config().version_info;
        send_feedback(&sender, &server, service.messages.component(&msg, &[]));
        Ok(0)
    }
}

struct PdReloadHandler;
impl CommandHandler for PdReloadHandler {
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

struct PdSetLobbyHandler;
impl CommandHandler for PdSetLobbyHandler {
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

struct AdminArenaHelpHandler;
impl CommandHandler for AdminArenaHelpHandler {
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
        let raw = "<color:#E2B700>Arena Commands</color> <color:#555555>»</color>\n<color:#555555>•</color> <color:#00E262>/arena create <name> [dimension]</color> <color:#555555>-</color> <color:#A0A0A0>Create a void arena template</color>\n<color:#555555>•</color> <color:#00E262>/arena setspawn1 <name></color> <color:#555555>-</color> <color:#A0A0A0>Set player 1 spawn at your location</color>\n<color:#555555>•</color> <color:#00E262>/arena setspawn2 <name></color> <color:#555555>-</color> <color:#A0A0A0>Set player 2 spawn at your location</color>\n<color:#555555>•</color> <color:#00E262>/arena setspectator <name></color> <color:#555555>-</color> <color:#A0A0A0>Set spectator spawn at your location</color>\n<color:#555555>•</color> <color:#00E262>/arena setbounds <name> [radius]</color> <color:#555555>-</color> <color:#A0A0A0>Set bounding radius</color>\n<color:#555555>•</color> <color:#00E262>/arena tp <name></color> <color:#555555>-</color> <color:#A0A0A0>Teleport to arena template world</color>\n<color:#555555>•</color> <color:#00E262>/arena leave</color> <color:#555555>-</color> <color:#A0A0A0>Return to lobby</color>\n<color:#555555>•</color> <color:#00E262>/arena list</color> <color:#555555>-</color> <color:#A0A0A0>List all arenas</color>\n<color:#555555>•</color> <color:#00E262>/arena delete <name></color> <color:#555555>-</color> <color:#A0A0A0>Delete arena</color>";
        send_feedback(&sender, &server, service.messages.component_raw(raw));
        Ok(0)
    }
}

struct AdminArenaCreateUsageHandler;
impl CommandHandler for AdminArenaCreateUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena create <name> [dimension]</color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena create <name> [dimension]</color>"));
                return Ok(0);
            }
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
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(_)) = service.storage.get_arena(&arena_name) {
            let msg = service.messages.get_config().arena_already_exists;
            send_feedback_error(
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
            "<color:#00E262>Void arena</color> <color:#E2B700>{arena_name}</color> <color:#00E262>created!</color> <color:#555555>•</color> <color:#A0A0A0>World: {world_name}. Build your arena, then run</color> <color:#00E262>/arena setspawn1 {arena_name}</color> <color:#A0A0A0>and</color> <color:#00E262>/arena setspawn2 {arena_name}</color><color:#A0A0A0>.</color>"
        );
        send_feedback_success(&sender, &server, service.messages.component_raw(&raw_msg));

        Ok(0)
    }
}

struct AdminArenaSetSpawn1UsageHandler;
impl CommandHandler for AdminArenaSetSpawn1UsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspawn1 <name></color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspawn1 <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
            send_feedback_success(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetSpawn2UsageHandler;
impl CommandHandler for AdminArenaSetSpawn2UsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspawn2 <name></color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspawn2 <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
            send_feedback_success(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetSpectatorUsageHandler;
impl CommandHandler for AdminArenaSetSpectatorUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspectator <name></color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setspectator <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
            send_feedback_success(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaSetBoundsUsageHandler;
impl CommandHandler for AdminArenaSetBoundsUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setbounds <name> [radius]</color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena setbounds <name> [radius]</color>"));
                return Ok(0);
            }
        };

        let radius = match args.get_value("radius") {
            Arg::Simple(s) | Arg::Msg(s) => s.parse::<i32>().unwrap_or(30),
            _ => 30,
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
            send_feedback_success(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &arena_name)]),
            );
        }

        Ok(0)
    }
}

struct AdminArenaTpUsageHandler;
impl CommandHandler for AdminArenaTpUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena tp <name></color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena tp <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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

            let raw_msg = format!("<color:#00E262>Teleported to arena</color> <color:#E2B700>{arena_name}</color><color:#00E262>.</color>");
            send_feedback_success(&sender, &server, service.messages.component_raw(&raw_msg));
        } else {
            let msg = service.messages.get_config().arena_not_found;
            send_feedback_error(
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
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        service.teleport_lobby(&player, &server);
        send_feedback_success(
            &sender,
            &server,
            service.messages.component_raw("<color:#00E262>Returned to lobby.</color>"),
        );
        Ok(0)
    }
}

struct AdminArenaDeleteUsageHandler;
impl CommandHandler for AdminArenaDeleteUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena delete <name></color>"));
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
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /arena delete <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
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
            send_feedback_error(
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
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        match service.storage.list_arenas() {
            Ok(arenas) if !arenas.is_empty() => {
                let mut msg = format!("<color:#E2B700>Configured Arenas ({})</color> <color:#555555>»</color>\n", arenas.len());
                for a in arenas {
                    msg.push_str(&format!(
                        "<color:#555555>•</color> <color:#00E262>{}</color> <color:#555555>(world: {}, enabled: {})</color>\n",
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

struct KitHelpHandler;
impl CommandHandler for KitHelpHandler {
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
        let raw = "<color:#E2B700>Kit Commands</color> <color:#555555>»</color>\n<color:#555555>•</color> <color:#00E262>/kit create <name></color> <color:#555555>-</color> <color:#A0A0A0>Save inventory as new kit</color>\n<color:#555555>•</color> <color:#00E262>/kit delete <name></color> <color:#555555>-</color> <color:#A0A0A0>Delete kit</color>\n<color:#555555>•</color> <color:#00E262>/kit edit <name></color> <color:#555555>-</color> <color:#A0A0A0>Overwrite kit items with current inventory</color>\n<color:#555555>•</color> <color:#00E262>/kit list</color> <color:#555555>-</color> <color:#A0A0A0>List all kits</color>\n<color:#555555>•</color> <color:#00E262>/kit rename <name> <new_name></color> <color:#555555>-</color> <color:#A0A0A0>Rename kit (MiniMessage supported)</color>";
        send_feedback(&sender, &server, service.messages.component_raw(raw));
        Ok(0)
    }
}

struct KitCreateUsageHandler;
impl CommandHandler for KitCreateUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit create <name></color>"));
        Ok(0)
    }
}

struct KitCreateHandler;
impl CommandHandler for KitCreateHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit create <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        if let Ok(Some(_)) = service.find_kit(&kit_name) {
            let raw = format!("<color:#E20000>Kit</color> <color:#E2B700>{kit_name}</color> <color:#E20000>already exists.</color> <color:#555555>•</color> <color:#A0A0A0>Use</color> <color:#00E262>/kit edit {kit_name}</color> <color:#A0A0A0>to update it.</color>");
            send_feedback_error(&sender, &server, service.messages.component_raw(&raw));
            return Ok(0);
        }

        let kit = capture_player_kit(&player, kit_name.clone());
        if let Ok(rec) = kit.to_record() {
            let _ = service.storage.save_kit(&rec);
        }
        let msg = service.messages.get_config().kit_saved;
        send_feedback_success(&sender, &server, service.messages.component(&msg, &[("{name}", &kit_name)]));
        Ok(0)
    }
}

struct KitDeleteUsageHandler;
impl CommandHandler for KitDeleteUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit delete <name></color>"));
        Ok(0)
    }
}

struct KitDeleteHandler;
impl CommandHandler for KitDeleteHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit delete <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        if let Ok(Some(k)) = service.find_kit(&kit_name) {
            let _ = service.storage.delete_kit(&k.name);
            let msg = service.messages.get_config().kit_deleted;
            send_feedback(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &k.name)]),
            );
        } else {
            let msg = service.messages.get_config().kit_not_found;
            send_feedback_error(
                &sender,
                &server,
                service.messages.component(&msg, &[("{name}", &kit_name)]),
            );
        }

        Ok(0)
    }
}

struct KitEditUsageHandler;
impl CommandHandler for KitEditUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit edit <name></color>"));
        Ok(0)
    }
}

struct KitEditHandler;
impl CommandHandler for KitEditHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let kit_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit edit <name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let player = match server.get_player_by_name(&sender.get_name()) {
            Some(p) => p,
            None => {
                let msg = service.messages.get_config().only_players;
                send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
                return Ok(0);
            }
        };

        let existing = match service.find_kit(&kit_name) {
            Ok(Some(k)) => k,
            _ => {
                let raw = format!("<color:#E20000>Kit</color> <color:#E2B700>{kit_name}</color> <color:#E20000>was not found.</color> <color:#555555>•</color> <color:#A0A0A0>Use</color> <color:#00E262>/kit create {kit_name}</color> <color:#A0A0A0>to create it.</color>");
                send_feedback_error(&sender, &server, service.messages.component_raw(&raw));
                return Ok(0);
            }
        };

        let mut updated = capture_player_kit(&player, existing.name.clone());
        updated.rules = existing.rules.into();
        updated.display_item = existing.display_item.clone();
        if let Ok(rec) = updated.to_record() {
            let _ = service.storage.save_kit(&rec);
        }

        let msg = service.messages.get_config().kit_edited;
        send_feedback_success(&sender, &server, service.messages.component(&msg, &[("{name}", &existing.name)]));
        Ok(0)
    }
}

struct KitListHandler;
impl CommandHandler for KitListHandler {
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

        match service.storage.list_kits() {
            Ok(kits) if !kits.is_empty() => {
                let header = format!(
                    "<color:#E2B700>Available Kits ({})</color> <color:#555555>»</color>",
                    kits.len()
                );
                send_feedback(&sender, &server, service.messages.component_raw(&header));
                for kit in kits {
                    let line = format!(
                        "<color:#555555>•</color> {} <color:#555555>(icon: {})</color>",
                        kit.name, kit.display_item
                    );
                    send_feedback(&sender, &server, service.messages.component_raw(&line));
                }
            }
            _ => {
                let msg = service.messages.get_config().kit_empty_list;
                send_feedback(&sender, &server, service.messages.component(&msg, &[]));
            }
        }
        Ok(0)
    }
}

struct KitRenameUsageHandler;
impl CommandHandler for KitRenameUsageHandler {
    fn handle(&self, sender: CommandSender, server: Server, _args: ConsumedArgs) -> Result<i32, CommandError> {
        send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit rename <name> <new_name></color>"));
        Ok(0)
    }
}

struct KitRenameHandler;
impl CommandHandler for KitRenameHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let old_name = match args.get_value("name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit rename <name> <new_name></color>"));
                return Ok(0);
            }
        };
        let new_name = match args.get_value("new_name") {
            Arg::Simple(s) | Arg::Msg(s) => s,
            _ => {
                send_feedback_error(&sender, &server, get_service().messages.component_raw("<color:#E20000>Usage: /kit rename <name> <new_name></color>"));
                return Ok(0);
            }
        };

        let service = get_service();
        if !check_admin_permission(&sender, &server) {
            let msg = service.messages.get_config().no_permission;
            send_feedback_error(&sender, &server, service.messages.component(&msg, &[]));
            return Ok(0);
        }

        let existing = match service.find_kit(&old_name) {
            Ok(Some(k)) => k,
            _ => {
                let msg = service.messages.get_config().kit_not_found;
                send_feedback_error(
                    &sender,
                    &server,
                    service.messages.component(&msg, &[("{name}", &old_name)]),
                );
                return Ok(0);
            }
        };

        if let Err(e) = service.storage.rename_kit(&existing.name, &new_name) {
            let raw = format!("<color:#E20000>Failed to rename kit: {e}</color>");
            send_feedback_error(&sender, &server, service.messages.component_raw(&raw));
            return Ok(0);
        }

        let msg = service.messages.get_config().kit_renamed;
        send_feedback_success(
            &sender,
            &server,
            service.messages.component(
                &msg,
                &[("{old_name}", &existing.name), ("{new_name}", &new_name)],
            ),
        );
        Ok(0)
    }
}

fn build_arena_tree() -> Command {
    let aliases = ["arena".to_string(), "arenas".to_string()];
    Command::new(&aliases, "Arena management command")
        .execute(AdminArenaHelpHandler)
        .then(
            CommandNode::literal("create")
                .execute(AdminArenaCreateUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaCreateHandler)
                        .then(
                            CommandNode::argument("dimension", &ArgumentType::String(StringType::SingleWord))
                                .execute(AdminArenaCreateHandler),
                        ),
                ),
        )
        .then(
            CommandNode::literal("setspawn1")
                .execute(AdminArenaSetSpawn1UsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaSetSpawn1Handler),
                ),
        )
        .then(
            CommandNode::literal("setspawn2")
                .execute(AdminArenaSetSpawn2UsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaSetSpawn2Handler),
                ),
        )
        .then(
            CommandNode::literal("setspectator")
                .execute(AdminArenaSetSpectatorUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaSetSpectatorHandler),
                ),
        )
        .then(
            CommandNode::literal("setbounds")
                .execute(AdminArenaSetBoundsUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaSetBoundsHandler)
                        .then(
                            CommandNode::argument("radius", &ArgumentType::String(StringType::SingleWord))
                                .execute(AdminArenaSetBoundsHandler),
                        ),
                ),
        )
        .then(
            CommandNode::literal("tp")
                .execute(AdminArenaTpUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaTpHandler),
                ),
        )
        .then(CommandNode::literal("leave").execute(AdminArenaLeaveHandler))
        .then(
            CommandNode::literal("delete")
                .execute(AdminArenaDeleteUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(AdminArenaDeleteHandler),
                ),
        )
        .then(CommandNode::literal("list").execute(AdminArenaListHandler))
}

fn build_kit_tree() -> Command {
    let aliases = ["kit".to_string(), "kits".to_string()];
    Command::new(&aliases, "Kit management command")
        .execute(KitHelpHandler)
        .then(
            CommandNode::literal("create")
                .execute(KitCreateUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(KitCreateHandler),
                ),
        )
        .then(
            CommandNode::literal("delete")
                .execute(KitDeleteUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(KitDeleteHandler),
                ),
        )
        .then(
            CommandNode::literal("edit")
                .execute(KitEditUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(KitEditHandler),
                ),
        )
        .then(CommandNode::literal("list").execute(KitListHandler))
        .then(
            CommandNode::literal("rename")
                .execute(KitRenameUsageHandler)
                .then(
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(KitRenameUsageHandler)
                        .then(
                            CommandNode::argument("new_name", &ArgumentType::String(StringType::Greedy))
                                .execute(KitRenameHandler),
                        ),
                ),
        )
}

fn build_duel_tree() -> Command {
    let duel_aliases = ["duel".to_string(), "duels".to_string(), "1v1".to_string(), "fight".to_string()];
    Command::new(&duel_aliases, "Practice duel command")
        .execute(DuelHelpHandler)
        .then(
            CommandNode::literal("invite")
                .execute(DuelInviteUsageHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelRootHandler)
                        .then(
                            CommandNode::argument("kit", &ArgumentType::String(StringType::SingleWord))
                                .execute(DuelInviteKitHandler),
                        ),
                ),
        )
        .then(
            CommandNode::literal("accept")
                .execute(DuelAcceptHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelAcceptHandler),
                ),
        )
        .then(
            CommandNode::literal("deny")
                .execute(DuelDeclineHandler)
                .then(
                    CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelDeclineHandler),
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
        .then(
            CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                .execute(DuelRootHandler)
                .then(
                    CommandNode::argument("kit", &ArgumentType::String(StringType::SingleWord))
                        .execute(DuelInviteKitHandler),
                ),
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

    // 1. /pumpkin-duels (aliases: pd, pumpkinduels, dueladmin, duelsadmin, da)
    let pd_aliases = [
        "pumpkin-duels".to_string(),
        "pd".to_string(),
        "pumpkinduels".to_string(),
        "dueladmin".to_string(),
        "duelsadmin".to_string(),
        "da".to_string(),
    ];
    let pd_cmd = Command::new(&pd_aliases, "Pumpkin-Duels administration command")
        .execute(PdHelpHandler)
        .then(CommandNode::literal("version").execute(PdVersionHandler))
        .then(CommandNode::literal("reload").execute(PdReloadHandler))
        .then(CommandNode::literal("setlobby").execute(PdSetLobbyHandler));
    context.register_command(pd_cmd, PERM_ADMIN);

    // 2. /duel (aliases: duels, 1v1, fight)
    context.register_command(build_duel_tree(), PERM_USER);

    // 3. /queue (aliases: q, joinqueue)
    let queue_aliases = ["queue".to_string(), "q".to_string(), "joinqueue".to_string()];
    let queue_cmd = Command::new(&queue_aliases, "Queue for a duel")
        .execute(DuelQueueHandler)
        .then(
            CommandNode::argument("kit", &ArgumentType::String(StringType::SingleWord))
                .execute(DuelQueueKitHandler),
        );
    context.register_command(queue_cmd, PERM_USER);

    // /leavequeue (aliases: lq, leave, unqueue)
    let leave_aliases = ["leavequeue".to_string(), "lq".to_string(), "leave".to_string(), "unqueue".to_string()];
    let leave_cmd = Command::new(&leave_aliases, "Leave duel queue").execute(DuelLeaveHandler);
    context.register_command(leave_cmd, PERM_USER);

    // 4. In-match commands: /surrender (ff, forfeit) and /ping (ms)
    let surrender_aliases = ["surrender".to_string(), "ff".to_string(), "forfeit".to_string()];
    let surrender_cmd = Command::new(&surrender_aliases, "Surrender current match")
        .execute(SurrenderHandler)
        .then(CommandNode::literal("confirm").execute(SurrenderConfirmHandler));
    context.register_command(surrender_cmd, PERM_USER);

    let ping_aliases = ["ping".to_string(), "ms".to_string()];
    let ping_cmd = Command::new(&ping_aliases, "View latency ping")
        .execute(PingHandler)
        .then(
            CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord))
                .execute(PingHandler),
        );
    context.register_command(ping_cmd, PERM_USER);

    // 5. /arena (aliases: arenas)
    context.register_command(build_arena_tree(), PERM_ADMIN);

    // 6. /kit (aliases: kits)
    context.register_command(build_kit_tree(), PERM_ADMIN);
}
