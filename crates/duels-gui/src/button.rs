use pumpkin_plugin_api::common::ClickType;
use pumpkin_plugin_api::item_stack::ItemStack;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Server;
use std::sync::Arc;

pub struct ClickContext {
    pub player: Player,
    pub server: Server,
    pub slot: u32,
    pub click_type: ClickType,
}

pub type ActionFn = Arc<dyn Fn(ClickContext) + Send + Sync>;

#[derive(Clone, Debug)]
pub struct ItemDescriptor {
    pub id: String,
    pub count: u8,
    pub name: Option<String>,
    pub lore: Vec<String>,
}

impl ItemDescriptor {
    pub fn new(id: &str, count: u8) -> Self {
        Self {
            id: id.to_string(),
            count,
            name: None,
            lore: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_lore(mut self, line: &str) -> Self {
        self.lore.push(line.to_string());
        self
    }

    pub fn build(&self) -> ItemStack {
        let stack = ItemStack::new(&self.id, self.count);
        if let Some(ref name) = self.name {
            stack.set_custom_name(Some(TextComponent::text(name)));
        }
        for line in &self.lore {
            stack.add_lore(TextComponent::text(line));
        }
        stack
    }
}

pub struct ItemBuilder {
    descriptor: ItemDescriptor,
}

impl ItemBuilder {
    pub fn new(id: &str, count: u8) -> Self {
        Self {
            descriptor: ItemDescriptor::new(id, count),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.descriptor.name = Some(name.to_string());
        self
    }

    pub fn lore(mut self, line: &str) -> Self {
        self.descriptor.lore.push(line.to_string());
        self
    }

    pub fn build(self) -> ItemDescriptor {
        self.descriptor
    }
}

#[derive(Clone)]
pub struct Button {
    pub item: ItemDescriptor,
    pub action: Option<ActionFn>,
}

impl Button {
    pub fn new(item: ItemDescriptor) -> Self {
        Self { item, action: None }
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(ClickContext) + Send + Sync + 'static,
    {
        self.action = Some(Arc::new(handler));
        self
    }
}
