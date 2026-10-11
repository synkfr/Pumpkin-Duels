pub mod arena_world;
pub mod commands;
pub mod config;
pub mod listeners;
pub mod messages;
pub mod minimessage;
pub mod service;
pub mod ui;

use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, register_plugin,
};
use service::DuelsService;
use std::sync::{Arc, Mutex};

pub static DUELS: Mutex<Option<Arc<DuelsService>>> = Mutex::new(None);

pub fn get_service() -> Arc<DuelsService> {
    DUELS
        .lock()
        .unwrap()
        .as_ref()
        .expect("DuelsService must be initialized")
        .clone()
}

pub struct PumpkinDuelsPlugin;

impl Plugin for PumpkinDuelsPlugin {
    fn new() -> Self {
        PumpkinDuelsPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "pumpkin-duels".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Sayan".into()],
            description: "Practice duel plugin for Pumpkin Minecraft server.".into(),
            dependencies: vec![],
            permissions: vec![
                pumpkin_plugin_api::permissions::FS_READ_DATA.into(),
                pumpkin_plugin_api::permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> Result<(), String> {
        let version = env!("CARGO_PKG_VERSION");
        tracing::info!("==================================================");
        tracing::info!("  Pumpkin-Duels v{} | Practice Duel Engine", version);
        tracing::info!("  Target: Pumpkin 0.2.0 | Protocol 777");
        tracing::info!("==================================================");

        let data_folder = context.get_data_folder();
        let service = Arc::new(
            DuelsService::new(data_folder)
                .map_err(|e| format!("Failed to init DuelsService: {e}"))?,
        );

        *DUELS.lock().unwrap() = Some(service.clone());

        commands::register_commands(&context);
        listeners::register_listeners(&context)?;

        let service_for_tick = service.clone();
        context.schedule_repeating_task(1, 1, move |server| {
            service_for_tick.tick_matches(&server);
        });

        let kits = service.storage.list_kits().unwrap_or_default();
        let kit_names: Vec<String> = kits.iter().map(|k| k.name.clone()).collect();
        let kits_summary = if kit_names.is_empty() {
            "0 loaded (use /kit create <name> to add one)".to_string()
        } else {
            format!("{} loaded ({})", kits.len(), kit_names.join(", "))
        };

        let arenas = service.storage.list_arenas().unwrap_or_default();
        let arenas_summary = if arenas.is_empty() {
            "0 configured (use /arena create <name> to set up)".to_string()
        } else {
            let enabled = arenas.iter().filter(|a| a.enabled).count();
            let arena_names: Vec<String> = arenas.iter().map(|a| a.name.clone()).collect();
            format!(
                "{} configured, {} enabled ({})",
                arenas.len(),
                enabled,
                arena_names.join(", ")
            )
        };

        let cfg = service.config.get();
        let lobby_summary = format!(
            "world '{}' at ({:.1}, {:.1}, {:.1})",
            cfg.lobby_world, cfg.lobby_x, cfg.lobby_y, cfg.lobby_z
        );

        tracing::info!("Storage     : SQLite connected (pumpkin-duels.db)");
        tracing::info!("Configs     : Loaded config.toml & messages.toml");
        tracing::info!("Kits        : {}", kits_summary);
        tracing::info!("Arenas      : {}", arenas_summary);
        tracing::info!("Lobby Spawn : {}", lobby_summary);
        tracing::info!("Commands    : /duel, /queue, /leavequeue, /surrender, /ping, /pd, /arena, /kit");
        tracing::info!("Permissions : pumpkin-duels:duels.use, pumpkin-duels:duels.admin");
        tracing::info!("Matchmaker  : Online (20 TPS match & queue ticker)");
        tracing::info!("==================================================");
        tracing::info!("Pumpkin-Duels v{} loaded and ready for duels!", version);
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> Result<(), String> {
        let version = env!("CARGO_PKG_VERSION");
        tracing::info!("Stopping Pumpkin-Duels v{}...", version);
        let service = DUELS.lock().unwrap().take();
        if let Some(s) = service {
            let active = s.active_matches.lock().unwrap().len();
            if active > 0 {
                tracing::info!("Terminated {} active match(es) during shutdown.", active);
            }
        }
        tracing::info!("Pumpkin-Duels v{} disabled successfully.", version);
        Ok(())
    }
}

register_plugin!(PumpkinDuelsPlugin);
