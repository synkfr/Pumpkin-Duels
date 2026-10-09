use duels_storage::{KitRecord, KitRulesDto};
use pumpkin_plugin_api::item_stack::ItemStack;
use pumpkin_plugin_api::player::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KitItem {
    pub slot: u32,
    pub id: String,
    pub count: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct KitRules {
    pub allow_build: bool,
    pub hunger: bool,
    pub natural_regen: bool,
    pub combo_mode: bool,
    pub boxing_mode: bool,
    pub sumo_mode: bool,
}

impl From<KitRulesDto> for KitRules {
    fn from(dto: KitRulesDto) -> Self {
        Self {
            allow_build: dto.allow_build,
            hunger: dto.hunger,
            natural_regen: dto.natural_regen,
            combo_mode: dto.combo_mode,
            boxing_mode: dto.boxing_mode,
            sumo_mode: dto.sumo_mode,
        }
    }
}

impl From<KitRules> for KitRulesDto {
    fn from(rules: KitRules) -> Self {
        Self {
            allow_build: rules.allow_build,
            hunger: rules.hunger,
            natural_regen: rules.natural_regen,
            combo_mode: rules.combo_mode,
            boxing_mode: rules.boxing_mode,
            sumo_mode: rules.sumo_mode,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Kit {
    pub name: String,
    pub display_item: String,
    pub items: Vec<KitItem>,
    pub helmet: Option<String>,
    pub chestplate: Option<String>,
    pub leggings: Option<String>,
    pub boots: Option<String>,
    pub offhand: Option<String>,
    pub rules: KitRules,
    pub enabled: bool,
}

impl Kit {
    pub fn sumo() -> Self {
        Self {
            name: "Sumo".to_string(),
            display_item: "minecraft:lead".to_string(),
            items: Vec::new(),
            helmet: None,
            chestplate: None,
            leggings: None,
            boots: None,
            offhand: None,
            rules: KitRules {
                allow_build: false,
                hunger: false,
                natural_regen: false,
                combo_mode: false,
                boxing_mode: false,
                sumo_mode: true,
            },
            enabled: true,
        }
    }

    pub fn classic() -> Self {
        let items = vec![
            KitItem {
                slot: 0,
                id: "minecraft:diamond_sword".to_string(),
                count: 1,
            },
            KitItem {
                slot: 1,
                id: "minecraft:bow".to_string(),
                count: 1,
            },
            KitItem {
                slot: 2,
                id: "minecraft:golden_apple".to_string(),
                count: 8,
            },
            KitItem {
                slot: 8,
                id: "minecraft:arrow".to_string(),
                count: 16,
            },
        ];

        Self {
            name: "Classic".to_string(),
            display_item: "minecraft:diamond_sword".to_string(),
            items,
            helmet: Some("minecraft:iron_helmet".to_string()),
            chestplate: Some("minecraft:iron_chestplate".to_string()),
            leggings: Some("minecraft:iron_leggings".to_string()),
            boots: Some("minecraft:iron_boots".to_string()),
            offhand: None,
            rules: KitRules {
                allow_build: false,
                hunger: false,
                natural_regen: true,
                combo_mode: false,
                boxing_mode: false,
                sumo_mode: false,
            },
            enabled: true,
        }
    }

    pub fn apply_to_player(&self, player: &Player) {
        let player_inv = player.get_inventory();
        player_inv.clear_all();

        let inv = player_inv.as_inventory();
        for item in &self.items {
            let stack = ItemStack::new(&item.id, item.count);
            inv.set_item(item.slot, Some(stack));
        }

        if let Some(ref helm_id) = self.helmet {
            player_inv.set_helmet(Some(ItemStack::new(helm_id, 1)));
        }
        if let Some(ref chest_id) = self.chestplate {
            player_inv.set_chestplate(Some(ItemStack::new(chest_id, 1)));
        }
        if let Some(ref leg_id) = self.leggings {
            player_inv.set_leggings(Some(ItemStack::new(leg_id, 1)));
        }
        if let Some(ref boot_id) = self.boots {
            player_inv.set_boots(Some(ItemStack::new(boot_id, 1)));
        }
        if let Some(ref off_id) = self.offhand {
            player_inv.set_off_hand(Some(ItemStack::new(off_id, 1)));
        }

        player.set_health(20.0);
        player.set_food_level(20);
    }

    pub fn to_record(&self) -> Result<KitRecord, String> {
        let inv_json = serde_json::to_string(&self.items).map_err(|e| e.to_string())?;

        let armor_struct = ArmorSlotData {
            helmet: self.helmet.clone(),
            chestplate: self.chestplate.clone(),
            leggings: self.leggings.clone(),
            boots: self.boots.clone(),
        };
        let armor_json = serde_json::to_string(&armor_struct).map_err(|e| e.to_string())?;
        let offhand_json = serde_json::to_string(&self.offhand).map_err(|e| e.to_string())?;

        Ok(KitRecord {
            name: self.name.clone(),
            display_item: self.display_item.clone(),
            inventory_json: inv_json,
            armor_json,
            offhand_json,
            rules: self.rules.clone().into(),
            enabled: self.enabled,
        })
    }

    pub fn from_record(record: KitRecord) -> Result<Self, String> {
        let items: Vec<KitItem> =
            serde_json::from_str(&record.inventory_json).map_err(|e| e.to_string())?;
        let armor: ArmorSlotData =
            serde_json::from_str(&record.armor_json).map_err(|e| e.to_string())?;
        let offhand: Option<String> =
            serde_json::from_str(&record.offhand_json).map_err(|e| e.to_string())?;

        Ok(Self {
            name: record.name,
            display_item: record.display_item,
            items,
            helmet: armor.helmet,
            chestplate: armor.chestplate,
            leggings: armor.leggings,
            boots: armor.boots,
            offhand,
            rules: record.rules.into(),
            enabled: record.enabled,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct ArmorSlotData {
    helmet: Option<String>,
    chestplate: Option<String>,
    leggings: Option<String>,
    boots: Option<String>,
}
