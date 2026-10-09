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
