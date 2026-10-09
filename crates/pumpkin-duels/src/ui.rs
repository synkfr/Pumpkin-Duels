use duels_storage::KitRecord;
use pumpkin_plugin_api::forms::{ModalFormBuilder, SimpleFormBuilder};
use pumpkin_plugin_api::java_dialog::{
    Action, ActionButton, AfterAction, CustomClickAction, Dialog, DialogBody, DialogType,
};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Player;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub enum PendingBedrockForm {
    Challenge {
        challenger_name: String,
    },
    Surrender,
    KitSelect {
        kits: Vec<String>,
        target_player: Option<String>,
    },
    MatchResult {
        kit_name: String,
    },
}

static PENDING_BEDROCK_FORMS: Mutex<Option<HashMap<(String, u32), PendingBedrockForm>>> =
    Mutex::new(None);

fn get_forms_map() -> &'static Mutex<Option<HashMap<(String, u32), PendingBedrockForm>>> {
    &PENDING_BEDROCK_FORMS
}

pub fn track_bedrock_form(player_uuid: &str, form_id: u32, form: PendingBedrockForm) {
    let mut lock = get_forms_map().lock().unwrap();
    let map = lock.get_or_insert_with(HashMap::new);
    map.insert((player_uuid.to_string(), form_id), form);
}

pub fn take_bedrock_form(player_uuid: &str, form_id: u32) -> Option<PendingBedrockForm> {
    let mut lock = get_forms_map().lock().unwrap();
    if let Some(map) = lock.as_mut() {
        map.remove(&(player_uuid.to_string(), form_id))
    } else {
        None
    }
}

pub fn show_challenge_ui(
    target: &Player,
    challenger: &Player,
    kit_name: &str,
    arena_name: &str,
) {
    let challenger_name = challenger.get_name();
    let target_uuid = target.get_id().to_string();

    if let Some(java) = target.as_java() {
        let dialog = Dialog {
            title: TextComponent::text("Duel Challenge"),
            type_: DialogType::Confirmation,
            body: vec![DialogBody::PlainMessage(TextComponent::text(&format!(
                "{} has challenged you to a duel with kit [{}] in [{}]!",
                challenger_name, kit_name, arena_name
            )))],
            inputs: vec![],
            buttons: vec![
                ActionButton {
                    text: TextComponent::text("Accept Duel"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: format!("duel:accept:{}", challenger_name),
                        payload: None,
                    }),
                },
                ActionButton {
                    text: TextComponent::text("Decline"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: format!("duel:decline:{}", challenger_name),
                        payload: None,
                    }),
                },
            ],
            links: vec![],
            after_action: Some(AfterAction::Pop),
            can_close_with_escape: true,
            external_title: None,
        };
        java.show_dialog(dialog);
    } else if let Some(bedrock) = target.as_bedrock() {
        let form = ModalFormBuilder::new(
            TextComponent::text("Duel Challenge"),
            TextComponent::text(&format!(
                "{} challenged you to a duel with kit [{}] in [{}]!\n\nDo you want to accept?",
                challenger_name, kit_name, arena_name
            )),
        )
        .button1(TextComponent::text("Accept"))
        .button2(TextComponent::text("Decline"))
        .build();

        let form_id = bedrock.open_form(form);
        track_bedrock_form(
            &target_uuid,
            form_id,
            PendingBedrockForm::Challenge {
                challenger_name: challenger_name.clone(),
            },
        );
    }
}

pub fn show_surrender_confirmation(player: &Player) {
    let player_uuid = player.get_id().to_string();

    if let Some(java) = player.as_java() {
        let dialog = Dialog {
            title: TextComponent::text("Confirm Surrender"),
            type_: DialogType::Confirmation,
            body: vec![DialogBody::PlainMessage(TextComponent::text(
                "Are you sure you want to forfeit this match? Your ELO will decrease.",
            ))],
            inputs: vec![],
            buttons: vec![
                ActionButton {
                    text: TextComponent::text("Forfeit Match"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: "duel:surrender:confirm".to_string(),
                        payload: None,
                    }),
                },
                ActionButton {
                    text: TextComponent::text("Stay in Match"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: "duel:surrender:cancel".to_string(),
                        payload: None,
                    }),
                },
            ],
            links: vec![],
            after_action: Some(AfterAction::Pop),
            can_close_with_escape: true,
            external_title: None,
        };
        java.show_dialog(dialog);
    } else if let Some(bedrock) = player.as_bedrock() {
        let form = ModalFormBuilder::new(
            TextComponent::text("Confirm Surrender"),
            TextComponent::text("Are you sure you want to forfeit this duel match?"),
        )
        .button1(TextComponent::text("Forfeit"))
        .button2(TextComponent::text("Stay"))
        .build();

        let form_id = bedrock.open_form(form);
        track_bedrock_form(&player_uuid, form_id, PendingBedrockForm::Surrender);
    }
}

pub fn show_match_results_ui(
    player: &Player,
    winner_name: &str,
    is_winner: bool,
    elo_delta: i32,
    kit_name: &str,
) {
    let player_uuid = player.get_id().to_string();
    let title_str = if is_winner { "VICTORY" } else { "DEFEAT" };
    let summary = format!(
        "Winner: {}\nELO Change: {:+}\nKit: {}",
        winner_name, elo_delta, kit_name
    );

    if let Some(java) = player.as_java() {
        let dialog = Dialog {
            title: TextComponent::text(title_str),
            type_: DialogType::MultiAction,
            body: vec![DialogBody::PlainMessage(TextComponent::text(&summary))],
            inputs: vec![],
            buttons: vec![
                ActionButton {
                    text: TextComponent::text("Queue Again"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: format!("duel:queue:{}", kit_name),
                        payload: None,
                    }),
                },
                ActionButton {
                    text: TextComponent::text("Return to Lobby"),
                    tooltip: None,
                    width: None,
                    action: Action::CustomClick(CustomClickAction {
                        id: "duel:lobby".to_string(),
                        payload: None,
                    }),
                },
            ],
            links: vec![],
            after_action: Some(AfterAction::Pop),
            can_close_with_escape: true,
            external_title: None,
        };
        java.show_dialog(dialog);
    } else if let Some(bedrock) = player.as_bedrock() {
        let form = SimpleFormBuilder::new(
            TextComponent::text(title_str),
            TextComponent::text(&summary),
        )
        .button(TextComponent::text("Queue Again"), None)
        .button(TextComponent::text("Return to Lobby"), None)
        .build();

        let form_id = bedrock.open_form(form);
        track_bedrock_form(
            &player_uuid,
            form_id,
            PendingBedrockForm::MatchResult {
                kit_name: kit_name.to_string(),
            },
        );
    }
}

pub fn open_bedrock_kit_selector(
    player: &Player,
    kits: &[KitRecord],
    target_player: Option<String>,
) -> bool {
    if let Some(bedrock) = player.as_bedrock() {
        let mut builder = SimpleFormBuilder::new(
            TextComponent::text("Select Duel Kit"),
            TextComponent::text("Choose a kit for matchmaking or challenge:"),
        );

        let mut kit_ids = Vec::new();
        for kit in kits {
            kit_ids.push(kit.name.clone());
            builder = builder.button(TextComponent::text(&kit.name), None);
        }

        let form_id = bedrock.open_form(builder.build());
        track_bedrock_form(
            &player.get_id().to_string(),
            form_id,
            PendingBedrockForm::KitSelect {
                kits: kit_ids,
                target_player,
            },
        );
        true
    } else {
        false
    }
}

use pumpkin_plugin_api::wit::pumpkin::plugin::scoreboard::{
    BedrockDisplaySlot, BedrockSortOrder, DisplaySlot, NumberFormat, RenderType,
};
use pumpkin_plugin_api::wit::pumpkin::plugin::status_effect::StatusEffectType;
use pumpkin_plugin_api::World;

pub const OBJECTIVE_HEALTH: &str = "duel_health";

pub fn get_combat_heart_color(player: &Player) -> &'static str {
    // 1. Freezing ticks (powder snow freeze)
    if player.get_freeze_ticks() >= 140 {
        return "#3BE3FF";
    }
    // 2. Wither effect
    if player.has_effect(StatusEffectType::Wither) {
        return "#303030";
    }
    // 3. Poison effect
    if player.has_effect(StatusEffectType::Poison) {
        return "#2DAD48";
    }
    // 4. Absorption (golden apple, absorption effect, or health > normal 20.0)
    if player.get_absorption() > 0.0
        || player.has_effect(StatusEffectType::Absorption)
        || player.get_health() > 20.0
    {
        return "#FFEE2E";
    }
    // 5. Default healthy/damaged red
    "#FF5555"
}

pub fn get_combat_heart_value(player: &Player, hp: f32) -> (String, i32) {
    let base_hp = hp.max(0.0);
    let absorption_hp = player.get_absorption().max(0.0);
    let total_hp = base_hp + absorption_hp;
    let hearts = total_hp / 2.0;
    let rounded_half_hearts = (hearts * 2.0).round() / 2.0;

    let heart_str = if rounded_half_hearts.fract().abs() < 1e-4 {
        format!("{:.0}", rounded_half_hearts)
    } else {
        format!("{:.1}", rounded_half_hearts)
    };

    let score_val = (rounded_half_hearts * 2.0).round() as i32;
    (heart_str, score_val)
}

fn format_nametag(template: &str, display_name: &str, heart_str: &str, color: &str) -> String {
    template
        .replace("{display_name}", display_name)
        .replace("{name}", display_name)
        .replace("{heart}", heart_str)
        .replace("{color}", color)
}

fn format_belowname(template: &str, heart_str: &str, color: &str, display_name: &str) -> String {
    template
        .replace("{heart}", heart_str)
        .replace("{color}", color)
        .replace("{display_name}", display_name)
        .replace("{name}", display_name)
}

pub fn init_combat_nametag_and_belowname(p1: &Player, p2: &Player, world: &World) {
    let p1_name = p1.get_name();
    let p2_name = p2.get_name();

    let service = crate::get_service();
    let msg_cfg = service.messages.get_config();

    let (p1_heart_str, p1_score) = get_combat_heart_value(p1, p1.get_health());
    let p1_color = get_combat_heart_color(p1);
    let p1_below = format_belowname(&msg_cfg.combat_belowname, &p1_heart_str, p1_color, &p1_name);

    let (p2_heart_str, p2_score) = get_combat_heart_value(p2, p2.get_health());
    let p2_color = get_combat_heart_color(p2);
    let p2_below = format_belowname(&msg_cfg.combat_belowname, &p2_heart_str, p2_color, &p2_name);

    let world_sb = world.get_scoreboard();
    world_sb.remove_objective(OBJECTIVE_HEALTH);
    world_sb.add_objective(
        OBJECTIVE_HEALTH,
        crate::minimessage::parse_minimessage("<color:#FF5555>❤</color>"),
        RenderType::Hearts,
        None,
    );
    world_sb.set_display_slot(DisplaySlot::BelowName, OBJECTIVE_HEALTH);
    world_sb.update_score(
        &p1_name,
        OBJECTIVE_HEALTH,
        p1_score,
        Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(&p1_below))),
    );
    world_sb.update_score(
        &p2_name,
        OBJECTIVE_HEALTH,
        p2_score,
        Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(&p2_below))),
    );

    setup_single_player_scoreboard(
        p1, &p1_name, p1_score, &p1_below, &p2_name, p2_score, &p2_below,
    );
    setup_single_player_scoreboard(
        p2, &p2_name, p2_score, &p2_below, &p1_name, p1_score, &p1_below,
    );

    update_player_nametag(p1, p1.get_health());
    update_player_nametag(p2, p2.get_health());
}

fn setup_single_player_scoreboard(
    player: &Player,
    p1_name: &str,
    p1_score: i32,
    p1_below: &str,
    p2_name: &str,
    p2_score: i32,
    p2_below: &str,
) {
    if let Some(java) = player.as_java() {
        let sb = java.get_scoreboard();
        sb.remove_objective(OBJECTIVE_HEALTH);
        sb.add_objective(
            OBJECTIVE_HEALTH,
            crate::minimessage::parse_minimessage("<color:#FF5555>❤</color>"),
            RenderType::Hearts,
            None,
        );
        sb.set_display_slot(DisplaySlot::BelowName, OBJECTIVE_HEALTH);
        sb.update_score(
            p1_name,
            OBJECTIVE_HEALTH,
            p1_score,
            Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(p1_below))),
        );
        sb.update_score(
            p2_name,
            OBJECTIVE_HEALTH,
            p2_score,
            Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(p2_below))),
        );
    }
    if let Some(bedrock) = player.as_bedrock() {
        let bsb = bedrock.get_scoreboard();
        bsb.remove_objective(OBJECTIVE_HEALTH);
        bsb.add_objective(OBJECTIVE_HEALTH, "§c❤", BedrockSortOrder::Descending);
        bsb.set_display_slot(BedrockDisplaySlot::BelowName, OBJECTIVE_HEALTH);
        bsb.update_score(p1_name, OBJECTIVE_HEALTH, p1_score);
        bsb.update_score(p2_name, OBJECTIVE_HEALTH, p2_score);
    }
}

pub fn update_combat_health(
    target: &Player,
    hp: f32,
    opponent: Option<&Player>,
    world: &World,
) {
    let name = target.get_name();
    let service = crate::get_service();
    let msg_cfg = service.messages.get_config();

    let (heart_str, score_val) = get_combat_heart_value(target, hp);
    let color = get_combat_heart_color(target);

    // Update target's overhead nametag
    let tag_text = format_nametag(&msg_cfg.combat_nametag, &name, &heart_str, color);
    let comp = crate::minimessage::parse_minimessage(&tag_text);
    target.as_entity().set_custom_name(comp);
    target.as_entity().set_custom_name_visible(true);

    // Format below-name string
    let below_text = format_belowname(&msg_cfg.combat_belowname, &heart_str, color, &name);

    // Update world scoreboard below-name value
    let world_sb = world.get_scoreboard();
    world_sb.update_score(
        &name,
        OBJECTIVE_HEALTH,
        score_val,
        Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(&below_text))),
    );

    // Update target's personal scoreboard
    if let Some(java) = target.as_java() {
        java.get_scoreboard().update_score(
            &name,
            OBJECTIVE_HEALTH,
            score_val,
            Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(&below_text))),
        );
    }
    if let Some(bedrock) = target.as_bedrock() {
        bedrock.get_scoreboard().update_score(&name, OBJECTIVE_HEALTH, score_val);
    }

    // Update opponent's personal scoreboard so below-name displays the new value
    if let Some(opp) = opponent {
        if let Some(java) = opp.as_java() {
            java.get_scoreboard().update_score(
                &name,
                OBJECTIVE_HEALTH,
                score_val,
                Some(NumberFormat::Fixed(crate::minimessage::parse_minimessage(&below_text))),
            );
        }
        if let Some(bedrock) = opp.as_bedrock() {
            bedrock.get_scoreboard().update_score(&name, OBJECTIVE_HEALTH, score_val);
        }
    }
}

pub fn update_player_nametag(player: &Player, hp: f32) {
    let name = player.get_name();
    let service = crate::get_service();
    let msg_cfg = service.messages.get_config();

    let (heart_str, _) = get_combat_heart_value(player, hp);
    let color = get_combat_heart_color(player);

    let tag_text = format_nametag(&msg_cfg.combat_nametag, &name, &heart_str, color);
    let comp = crate::minimessage::parse_minimessage(&tag_text);
    player.as_entity().set_custom_name(comp);
    player.as_entity().set_custom_name_visible(true);
}

pub fn reset_combat_nametag_and_belowname(player: &Player) {
    let name = player.get_name();
    player.as_entity().set_custom_name(TextComponent::text(&name));
    player.as_entity().set_custom_name_visible(false);

    if let Some(java) = player.as_java() {
        let sb = java.get_scoreboard();
        sb.clear_display_slot(DisplaySlot::BelowName);
        sb.remove_objective(OBJECTIVE_HEALTH);
        java.reset_scoreboard();
    }
    if let Some(bedrock) = player.as_bedrock() {
        let bsb = bedrock.get_scoreboard();
        bsb.clear_display_slot(BedrockDisplaySlot::BelowName);
        bsb.remove_objective(OBJECTIVE_HEALTH);
        bedrock.reset_scoreboard();
    }
}

