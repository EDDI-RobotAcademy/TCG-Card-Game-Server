use serde_json::Value as JsonValue;
use crate::game_card_unit::controller::request_form::attack_game_main_character_request_form::AttackGameMainCharacterRequestForm;

pub fn create_attack_game_main_character_request_form(data: &JsonValue) -> Option<AttackGameMainCharacterRequestForm> {
    if let (Some(sessionInfo), Some(attacker_unit_index), Some(target_game_main_character_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("attacker_unit_index").and_then(|v| v.as_str()),
        data.get("target_game_main_character_index").and_then(|v| v.as_str()),
    ) {
        Some(AttackGameMainCharacterRequestForm::new(
            sessionInfo.to_string(),
            attacker_unit_index.to_string(),
            target_game_main_character_index.to_string()))
    } else {
        None
    }
}
