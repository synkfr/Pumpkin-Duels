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

        tracing::info!("Pumpkin-Duels successfully loaded!");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> Result<(), String> {
        *DUELS.lock().unwrap() = None;
        tracing::info!("Pumpkin-Duels unloaded.");
        Ok(())
    }
}

register_plugin!(PumpkinDuelsPlugin);
