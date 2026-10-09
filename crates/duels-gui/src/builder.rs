use crate::button::{ActionFn, Button};
use crate::registry::GuiSessionRegistry;
use pumpkin_plugin_api::gui::Gui;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Screen;
use std::collections::HashMap;

pub struct GuiBuilder {
    screen: Screen,
    title: String,
    slots: HashMap<u32, Button>,
    allow_grab: bool,
    allow_put: bool,
}

impl GuiBuilder {
    pub fn new(screen: Screen, title: &str) -> Self {
        Self {
            screen,
            title: title.to_string(),
            slots: HashMap::new(),
            allow_grab: false,
            allow_put: false,
        }
    }

    pub fn size(&self) -> u32 {
        match self.screen {
            Screen::Generic9x1 => 9,
            Screen::Generic9x2 => 18,
            Screen::Generic9x3 => 27,
            Screen::Generic9x4 => 36,
            Screen::Generic9x5 => 45,
            Screen::Generic9x6 => 54,
            _ => 27,
        }
    }

    pub fn set_slot(mut self, slot: u32, button: Button) -> Self {
        if slot < self.size() {
            self.slots.insert(slot, button);
        }
        self
    }

    pub fn set_grid(self, row: u32, col: u32, button: Button) -> Self {
        if col < 9 {
            let slot = row * 9 + col;
            self.set_slot(slot, button)
        } else {
            self
        }
    }

    pub fn fill_border(mut self, button: Button) -> Self {
        let total = self.size();
        let rows = total / 9;
        if rows < 2 {
            return self;
        }

        for col in 0..9 {
            self.slots.entry(col).or_insert_with(|| button.clone());
            self.slots
                .entry((rows - 1) * 9 + col)
                .or_insert_with(|| button.clone());
        }

        for row in 1..(rows - 1) {
            self.slots.entry(row * 9).or_insert_with(|| button.clone());
            self.slots
                .entry(row * 9 + 8)
                .or_insert_with(|| button.clone());
        }

        self
    }

    pub fn fill(mut self, button: Button) -> Self {
        let total = self.size();
        for slot in 0..total {
            self.slots.entry(slot).or_insert_with(|| button.clone());
        }
        self
    }

    pub fn open(self, player: &Player, registry: &GuiSessionRegistry) {
        let gui = Gui::new(self.screen, TextComponent::text(&self.title));
        gui.set_allow_grab_items(self.allow_grab);
        gui.set_allow_put_items(self.allow_put);

        let mut actions: HashMap<u32, ActionFn> = HashMap::new();

        for (slot, button) in self.slots {
            let stack = button.item.build();
            gui.set_item(slot, stack);
            if let Some(action) = button.action {
                actions.insert(slot, action);
            }
        }

        let player_uuid = player.get_id().to_string();
        registry.register(player_uuid, actions);
        player.open_gui(gui);
    }
}
