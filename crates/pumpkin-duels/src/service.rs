use crate::config::PluginConfig;
use crate::messages::MessageService;
use duels_config::ConfigManager;
use duels_core::arena::{ArenaInstanceManager, ArenaLocation, ArenaTemplate};
use duels_core::kit::Kit;
use duels_core::match_state::{MatchPhase, MatchSession};
use duels_core::queue::{DuelChallenge, Matchmaker, QueueEntry};
use duels_gui::{Button, GuiBuilder, GuiSessionRegistry, ItemDescriptor};
use duels_storage::SqliteStorage;
use pumpkin_plugin_api::common::Position;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::{Screen, Server};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub fn play_ui_click(_player: &Player) {}

pub fn play_sound_error(_player: &Player) {}

pub fn play_sound_success(_player: &Player) {}

pub fn play_duel_sent(_player: &Player) {}

pub fn play_duel_received(_player: &Player) {}

pub fn play_countdown_tick(_player: &Player, _remaining_sec: u32) {}

pub fn play_match_start(_player: &Player) {}

pub fn play_victory(_player: &Player) {}

pub fn play_defeat(_player: &Player) {}

pub struct DuelsService {
    pub storage: Arc<SqliteStorage>,
    pub config: Arc<ConfigManager<PluginConfig>>,
    pub messages: Arc<MessageService>,
    pub gui_registry: GuiSessionRegistry,
    pub matchmaker: Arc<Mutex<Matchmaker>>,
    pub arena_manager: Arc<ArenaInstanceManager>,
    pub active_matches: Arc<Mutex<HashMap<String, MatchSession>>>,
    pub player_match_map: Arc<Mutex<HashMap<String, String>>>,
}

impl DuelsService {
    pub fn new<P: AsRef<Path>>(
        data_dir: P,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let dir = data_dir.as_ref();
        std::fs::create_dir_all(dir)?;

        let config_path = dir.join("config.toml");
        let messages_path = dir.join("messages.toml");
        let db_path = dir.join("pumpkin-duels.db");

        let config_manager: ConfigManager<PluginConfig> = ConfigManager::load_or_create(config_path)?;
        let message_service = MessageService::new(messages_path)?;
        let storage = SqliteStorage::open(db_path)?;

        let worlds_dir = dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join(&config_manager.get().worlds_directory))
            .unwrap_or_else(|| PathBuf::from(&config_manager.get().worlds_directory));

        let service = Self {
            storage: Arc::new(storage),
            config: Arc::new(config_manager),
            messages: Arc::new(message_service),
            gui_registry: GuiSessionRegistry::new(),
            matchmaker: Arc::new(Mutex::new(Matchmaker::new())),
            arena_manager: Arc::new(ArenaInstanceManager::new(worlds_dir)),
            active_matches: Arc::new(Mutex::new(HashMap::new())),
            player_match_map: Arc::new(Mutex::new(HashMap::new())),
        };

        service.init_defaults()?;
        Ok(service)
    }

    fn init_defaults(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let existing_kits = self.storage.list_kits()?;
        if existing_kits.is_empty() {
            let sumo = Kit::sumo().to_record().map_err(|e| e.to_string())?;
            let classic = Kit::classic().to_record().map_err(|e| e.to_string())?;
            self.storage.save_kit(&sumo)?;
            self.storage.save_kit(&classic)?;
        }
        Ok(())
    }

    pub fn is_in_match(&self, player_uuid: &str) -> bool {
        self.player_match_map
            .lock()
            .unwrap()
            .contains_key(player_uuid)
    }

    pub fn get_match_by_player(&self, player_uuid: &str) -> Option<MatchSession> {
        let match_id = {
            let guard = self.player_match_map.lock().unwrap();
            guard.get(player_uuid).cloned()
        }?;

        let matches = self.active_matches.lock().unwrap();
        matches.get(&match_id).cloned()
    }

    pub fn send_challenge(
        &self,
        challenger: &Player,
        target: &Player,
        kit_name: &str,
    ) -> Result<(), String> {
        let c_uuid = challenger.get_id().to_string();
        let t_uuid = target.get_id().to_string();

        if c_uuid == t_uuid {
            let msg = self.messages.get_config().duel_cannot_self;
            challenger.send_system_message(self.messages.component(&msg, &[]), false);
            play_sound_error(challenger);
            return Ok(());
        }

        if self.is_in_match(&c_uuid) {
            let msg = self.messages.get_config().duel_already_in_match;
            challenger.send_system_message(self.messages.component(&msg, &[]), false);
            play_sound_error(challenger);
            return Ok(());
        }

        if self.is_in_match(&t_uuid) {
            let msg = self.messages.get_config().target_already_in_match;
            challenger.send_system_message(
                self.messages
                    .component(&msg, &[("{target}", &target.get_name())]),
                false,
            );
            play_sound_error(challenger);
            return Ok(());
        }

        let expire_secs = self.config.get().challenge_expire_seconds as u64;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        {
            let mm = self.matchmaker.lock().unwrap();
            if let Some(existing) = mm.get_challenge(&c_uuid, &t_uuid)
                && now.saturating_sub(existing.created_at) < 3
            {
                challenger.send_system_message(
                    self.messages.component_raw(
                        "<color:#E2B700>Please wait a moment before sending another duel request.</color>",
                    ),
                    false,
                );
                play_sound_error(challenger);
                return Ok(());
            }
        }

        let challenge = DuelChallenge {
            challenger_uuid: c_uuid,
            challenger_name: challenger.get_name(),
            target_uuid: t_uuid,
            target_name: target.get_name(),
            kit_name: kit_name.to_string(),
            created_at: now,
            expires_at: now + expire_secs,
        };

        self.matchmaker.lock().unwrap().send_challenge(challenge);

        let sent_msg = self.messages.get_config().duel_sent;
        challenger.send_system_message(
            self.messages
                .component(&sent_msg, &[("{target}", &target.get_name()), ("{kit}", kit_name)]),
            false,
        );
        play_duel_sent(challenger);

        let rec_msg = self.messages.get_config().duel_received;
        target.send_system_message(
            self.messages
                .component(&rec_msg, &[("{challenger}", &challenger.get_name()), ("{kit}", kit_name)]),
            false,
        );
        play_duel_received(target);

        Ok(())
    }

    pub fn accept_challenge(
        &self,
        target: &Player,
        challenger_name: Option<&str>,
        server: &Server,
    ) -> Result<(), String> {
        let target_uuid = target.get_id().to_string();

        let challenge = {
            let mut mm = self.matchmaker.lock().unwrap();
            if let Some(c_name) = challenger_name {
                let challenger = server.get_player_by_name(c_name).ok_or_else(|| {
                    let msg = self.messages.get_config().player_not_found;
                    self.messages.format(&msg, &[("{player}", c_name)])
                })?;
                mm.accept_challenge(&challenger.get_id().to_string(), &target_uuid)
            } else {
                mm.accept_latest_challenge_for_target(&target_uuid)
            }
        }
        .ok_or_else(|| {
            let msg = self.messages.get_config().duel_expired;
            self.messages.format(&msg, &[])
        })?;

        let challenger = server
            .get_player_by_name(&challenge.challenger_name)
            .ok_or_else(|| {
                let msg = self.messages.get_config().player_not_found;
                self.messages.format(&msg, &[("{player}", &challenge.challenger_name)])
            })?;

        let kit = self
            .find_kit(&challenge.kit_name)?
            .ok_or_else(|| {
                let msg = self.messages.get_config().kit_not_found;
                self.messages.format(&msg, &[("{name}", &challenge.kit_name)])
            })?;

        let core_kit = Kit::from_record(kit)?;
        if let Err(e) = self.start_match(&challenger, target, core_kit, server) {
            let err_text = format!("<color:#E20000>Failed to start duel: {e}</color>");
            target.send_system_message(self.messages.component_raw(&err_text), false);
            challenger.send_system_message(self.messages.component_raw(&err_text), false);
            play_sound_error(target);
            play_sound_error(&challenger);
            return Err(e);
        }

        let acc_msg = self.messages.get_config().duel_accepted;
        challenger.send_system_message(self.messages.component(&acc_msg, &[]), false);
        target.send_system_message(self.messages.component(&acc_msg, &[]), false);
        play_ui_click(target);
        play_ui_click(&challenger);

        Ok(())
    }

    pub fn decline_challenge(
        &self,
        target: &Player,
        challenger_name: Option<&str>,
        server: &Server,
    ) -> Result<(), String> {
        let target_uuid = target.get_id().to_string();

        let (challenger_opt, success) = {
            let mut mm = self.matchmaker.lock().unwrap();
            if let Some(c_name) = challenger_name {
                let challenger = server.get_player_by_name(c_name).ok_or_else(|| {
                    let msg = self.messages.get_config().player_not_found;
                    self.messages.format(&msg, &[("{player}", c_name)])
                })?;
                let ok = mm.decline_challenge(&challenger.get_id().to_string(), &target_uuid);
                (Some(challenger), ok)
            } else {
                let ok = mm.decline_latest_challenge_for_target(&target_uuid);
                (None, ok)
            }
        };

        if !success {
            let msg = self.messages.get_config().duel_expired;
            return Err(self.messages.format(&msg, &[]));
        }

        let dec_msg = self.messages.get_config().duel_declined;
        let c_display = challenger_opt
            .as_ref()
            .map(|c| c.get_name())
            .unwrap_or_else(|| "challenger".to_string());

        target.send_system_message(
            self.messages
                .component(&dec_msg, &[("{challenger}", &c_display)]),
            false,
        );
        play_sound_error(target);

        if let Some(c) = challenger_opt {
            let can_msg = self.messages.get_config().duel_cancelled;
            c.send_system_message(
                self.messages
                    .component(&can_msg, &[("{target}", &target.get_name())]),
                false,
            );
            play_sound_error(&c);
        }

        Ok(())
    }

    pub fn enqueue(
        &self,
        player: &Player,
        kit_name: &str,
        ranked: bool,
        server: &Server,
    ) -> Result<(), String> {
        let p_uuid = player.get_id().to_string();
        if self.is_in_match(&p_uuid) {
            let msg = self.messages.get_config().duel_already_in_match;
            player.send_system_message(self.messages.component(&msg, &[]), false);
            play_sound_error(player);
            return Ok(());
        }

        let entry = QueueEntry {
            player_uuid: p_uuid,
            player_name: player.get_name(),
            kit_name: kit_name.to_string(),
            ranked,
            enqueued_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        let matched = self.matchmaker.lock().unwrap().enqueue(entry);

        if let Some((p1_entry, p2_entry)) = matched {
            let p1 = server.get_player_by_name(&p1_entry.player_name);
            let p2 = server.get_player_by_name(&p2_entry.player_name);

            if let (Some(p1), Some(p2)) = (p1, p2) {
                let kit_rec = self
                    .find_kit(&p1_entry.kit_name)?
                    .ok_or_else(|| self.messages.get_config().kit_not_found)?;
                let kit = Kit::from_record(kit_rec)?;
                self.start_match(&p1, &p2, kit, server)?;
            }
        } else {
            let msg = self.messages.get_config().queue_joined;
            player.send_system_message(
                self.messages.component(&msg, &[("{kit}", kit_name)]),
                false,
            );
            play_ui_click(player);
        }

        Ok(())
    }

    pub fn leave_queue(&self, player: &Player) {
        let p_uuid = player.get_id().to_string();
        let was_queued = self.matchmaker.lock().unwrap().dequeue(&p_uuid);
        if was_queued {
            let msg = self.messages.get_config().queue_left;
            player.send_system_message(self.messages.component(&msg, &[]), false);
            play_ui_click(player);
        } else {
            let msg = self.messages.get_config().queue_not_in;
            player.send_system_message(self.messages.component(&msg, &[]), false);
            play_sound_error(player);
        }
    }

    pub fn start_match(
        &self,
        p1: &Player,
        p2: &Player,
        kit: Kit,
        server: &Server,
    ) -> Result<(), String> {
        let active_arenas: std::collections::HashSet<String> = {
            let matches = self.active_matches.lock().unwrap();
            matches.values().map(|s| s.arena.template_name.clone()).collect()
        };

        let arenas = self.storage.list_arenas().map_err(|e| e.to_string())?;
        let active_template: ArenaTemplate = arenas
            .into_iter()
            .filter(|a| a.enabled)
            .min_by_key(|a| {
                active_arenas.iter().filter(|&name| name == &a.name).count()
            })
            .map(Into::into)
            .ok_or_else(|| {
                let msg = self.messages.get_config().arena_not_found;
                self.messages.format(&msg, &[("{name}", "available")])
            })?;

        let match_id = uuid::Uuid::new_v4().to_string();
        let arena_instance = self
            .arena_manager
            .create_instance(server, &active_template, &match_id)?;

        let countdown_ticks = self.config.get().countdown_seconds * 20;

        let session = MatchSession::new(
            match_id.clone(),
            p1.get_id().to_string(),
            p1.get_name(),
            p2.get_id().to_string(),
            p2.get_name(),
            kit.clone(),
            arena_instance.clone(),
            countdown_ticks,
        );

        kit.apply_to_player(p1);
        kit.apply_to_player(p2);

        let duel_world = server
            .get_world_by_name(&arena_instance.world_name)
            .unwrap_or_else(|| {
                server.create_world(
                    &arena_instance.world_name,
                    pumpkin_plugin_api::server::Dimension::Overworld,
                )
            });

        teleport_player(p1, &arena_instance.spawn1, duel_world);
        let duel_world2 = server
            .get_world_by_name(&arena_instance.world_name)
            .unwrap_or_else(|| {
                server.create_world(
                    &arena_instance.world_name,
                    pumpkin_plugin_api::server::Dimension::Overworld,
                )
            });
        teleport_player(p2, &arena_instance.spawn2, duel_world2);

        {
            let mut matches = self.active_matches.lock().unwrap();
            matches.insert(match_id.clone(), session);

            let mut map = self.player_match_map.lock().unwrap();
            map.insert(p1.get_id().to_string(), match_id.clone());
            map.insert(p2.get_id().to_string(), match_id);
        }

        Ok(())
    }

    pub fn tick_matches(&self, server: &Server) {
        let mut matches_to_end = Vec::new();

        {
            let mut matches = self.active_matches.lock().unwrap();
            for (id, session) in matches.iter_mut() {
                match &mut session.phase {
                    MatchPhase::Countdown { remaining_ticks } => {
                        if *remaining_ticks > 0 {
                            if *remaining_ticks % 20 == 0 {
                                let sec_val = *remaining_ticks / 20;
                                let sec = sec_val.to_string();
                                let msg = self.messages.get_config().match_countdown;
                                let comp = self.messages.component(&msg, &[("{seconds}", &sec)]);
                                if let Some(p1) = server.get_player_by_name(&session.player1_name) {
                                    p1.show_title(comp);
                                    play_countdown_tick(&p1, sec_val);
                                }
                                if let Some(p2) = server.get_player_by_name(&session.player2_name) {
                                    p2.show_title(self.messages.component(&msg, &[("{seconds}", &sec)]));
                                    play_countdown_tick(&p2, sec_val);
                                }
                            }
                            *remaining_ticks -= 1;
                        } else {
                            session.phase = MatchPhase::Fighting { elapsed_seconds: 0 };
                            let start_comp = self.messages.component(&self.messages.get_config().match_start, &[]);
                            if let Some(p1) = server.get_player_by_name(&session.player1_name) {
                                p1.show_title(start_comp);
                                play_match_start(&p1);
                            }
                            if let Some(p2) = server.get_player_by_name(&session.player2_name) {
                                p2.show_title(self.messages.component(&self.messages.get_config().match_start, &[]));
                                play_match_start(&p2);
                            }
                        }
                    }
                    MatchPhase::Fighting { elapsed_seconds } => {
                        *elapsed_seconds += 1;

                        if let Some(p1) = server.get_player_by_name(&session.player1_name) {
                            let pos = p1.get_position();
                            if session.arena.bounds.is_ringout(pos.1) {
                                matches_to_end.push((id.clone(), session.player2_uuid.clone()));
                                continue;
                            }
                        }
                        if let Some(p2) = server.get_player_by_name(&session.player2_name) {
                            let pos = p2.get_position();
                            if session.arena.bounds.is_ringout(pos.1) {
                                matches_to_end.push((id.clone(), session.player1_uuid.clone()));
                                continue;
                            }
                        }
                    }
                    MatchPhase::Ended { .. } => {}
                }
            }
        }

        for (match_id, winner_uuid) in matches_to_end {
            self.end_match(&match_id, &winner_uuid, server);
        }
    }

    pub fn end_match(&self, match_id: &str, winner_uuid: &str, server: &Server) {
        let session = {
            let mut matches = self.active_matches.lock().unwrap();
            matches.remove(match_id)
        };

        if let Some(session) = session {
            {
                let mut map = self.player_match_map.lock().unwrap();
                map.remove(&session.player1_uuid);
                map.remove(&session.player2_uuid);
            }

            let loser_uuid = if session.player1_uuid == winner_uuid {
                session.player2_uuid.clone()
            } else {
                session.player1_uuid.clone()
            };

            let winner_name = if session.player1_uuid == winner_uuid {
                session.player1_name.clone()
            } else {
                session.player2_name.clone()
            };

            let loser_name = if session.player1_uuid == winner_uuid {
                session.player2_name.clone()
            } else {
                session.player1_name.clone()
            };

            let _ = self.storage.record_match(
                winner_uuid,
                &loser_uuid,
                &session.kit.name,
                &session.arena.template_name,
                match &session.phase {
                    MatchPhase::Fighting { elapsed_seconds } => *elapsed_seconds,
                    _ => 0,
                },
            );

            if let Ok(mut w_stats) = self.storage.get_or_create_stats(winner_uuid, &winner_name) {
                w_stats.wins += 1;
                w_stats.current_streak += 1;
                if w_stats.current_streak > w_stats.best_streak {
                    w_stats.best_streak = w_stats.current_streak;
                }
                w_stats.elo += 25;
                let _ = self.storage.update_stats(&w_stats);
            }

            if let Ok(mut l_stats) = self.storage.get_or_create_stats(&loser_uuid, &loser_name) {
                l_stats.losses += 1;
                l_stats.current_streak = 0;
                if l_stats.elo >= 1020 {
                    l_stats.elo -= 20;
                }
                let _ = self.storage.update_stats(&l_stats);
            }

            let win_msg = self.messages.get_config().match_won;
            let lose_msg = self.messages.get_config().match_lost;
            let bcast_msg = self.messages.get_config().match_broadcast_win;

            if let Some(w_player) = server.get_player_by_name(&winner_name) {
                w_player.show_title(self.messages.component(&win_msg, &[("{loser}", &loser_name)]));
                self.teleport_lobby(&w_player, server);
                play_victory(&w_player);
            }

            if let Some(l_player) = server.get_player_by_name(&loser_name) {
                l_player.show_title(self.messages.component(&lose_msg, &[("{winner}", &winner_name)]));
                self.teleport_lobby(&l_player, server);
                play_defeat(&l_player);
            }

            let bcast_formatted = self.messages.format(
                &bcast_msg,
                &[
                    ("{winner}", &winner_name),
                    ("{loser}", &loser_name),
                    ("{arena}", &session.arena.template_name),
                    ("{kit}", &session.kit.name),
                ],
            );
            for p in server.get_all_players() {
                p.send_system_message(self.messages.component_raw(&bcast_formatted), false);
            }

            let _ = self.arena_manager.destroy_instance(server, &session.arena);
        }
    }

    pub fn teleport_lobby(&self, player: &Player, server: &Server) {
        let cfg = self.config.get();
        let lobby_world = server
            .get_world_by_name(&cfg.lobby_world)
            .or_else(|| server.get_world_by_name("minecraft:overworld"))
            .or_else(|| server.get_all_worlds().into_iter().next())
            .unwrap_or_else(|| player.get_world());

        player.set_health(20.0);
        player.set_food_level(20);
        player.get_inventory().clear_all();

        player.teleport(
            (cfg.lobby_x, cfg.lobby_y, cfg.lobby_z),
            Some(cfg.lobby_yaw),
            Some(cfg.lobby_pitch),
            lobby_world,
        );
    }

    pub fn open_kit_selector(
        self: &Arc<Self>,
        player: &Player,
        target_name: Option<String>,
    ) -> Result<(), String> {
        let kits = self.storage.list_kits().map_err(|e| e.to_string())?;
        let mut builder = GuiBuilder::new(Screen::Generic9x3, "Select Kit");

        let border_item = ItemDescriptor::new("minecraft:gray_stained_glass_pane", 1).with_name(" ");
        builder = builder.fill_border(Button::new(border_item));

        for (idx, kit) in kits.into_iter().enumerate() {
            if idx >= 9 {
                break;
            }
            let display_name = crate::minimessage::minimessage_to_legacy(&kit.name);
            let item_desc = ItemDescriptor::new(&kit.display_item, 1)
                .with_name(&display_name)
                .with_lore("§7Click to select this kit!");

            let service = self.clone();
            let target_copy = target_name.clone();
            let kit_name = kit.name.clone();

            let button = Button::new(item_desc).on_click(move |ctx| {
                if let Some(target) = &target_copy {
                    if let Some(t_player) = ctx.server.get_player_by_name(target) {
                        let _ = service.send_challenge(&ctx.player, &t_player, &kit_name);
                    }
                } else {
                    let _ = service.enqueue(&ctx.player, &kit_name, false, &ctx.server);
                }
            });

            builder = builder.set_grid(1, (idx + 1) as u32, button);
        }

        builder.open(player, &self.gui_registry);
        Ok(())
    }

    pub fn find_kit(&self, query: &str) -> Result<Option<duels_storage::KitRecord>, String> {
        if let Ok(Some(k)) = self.storage.get_kit(query) {
            return Ok(Some(k));
        }
        let all = self.storage.list_kits().map_err(|e| e.to_string())?;
        let stripped_query = crate::minimessage::strip_minimessage(query).to_lowercase();
        for k in all {
            if k.name.eq_ignore_ascii_case(query) {
                return Ok(Some(k));
            }
            let stripped_name = crate::minimessage::strip_minimessage(&k.name).to_lowercase();
            if stripped_name == stripped_query {
                return Ok(Some(k));
            }
        }
        Ok(None)
    }

    pub fn surrender_match(&self, player: &Player, server: &Server) -> Result<(), String> {
        let p_uuid = player.get_id().to_string();
        let match_id = {
            let map = self.player_match_map.lock().unwrap();
            map.get(&p_uuid).cloned()
        }
        .ok_or_else(|| self.messages.get_config().not_in_match)?;

        let (winner_uuid, winner_name) = {
            let matches = self.active_matches.lock().unwrap();
            let session = matches
                .get(&match_id)
                .ok_or_else(|| self.messages.get_config().not_in_match)?;
            if session.player1_uuid == p_uuid {
                (session.player2_uuid.clone(), session.player2_name.clone())
            } else {
                (session.player1_uuid.clone(), session.player1_name.clone())
            }
        };

        let msg = self.messages.get_config().match_surrendered;
        let surrender_comp = self
            .messages
            .component(&msg, &[("{player}", &player.get_name())]);

        player.send_system_message(surrender_comp, false);
        if let Some(winner) = server.get_player_by_name(&winner_name) {
            winner.send_system_message(
                self.messages
                    .component(&msg, &[("{player}", &player.get_name())]),
                false,
            );
        }

        self.end_match(&match_id, &winner_uuid, server);
        Ok(())
    }
}

fn teleport_player(player: &Player, loc: &ArenaLocation, world: pumpkin_plugin_api::World) {
    let pos: Position = (loc.x, loc.y, loc.z);
    player.teleport(pos, Some(loc.yaw), Some(loc.pitch), world);
}
