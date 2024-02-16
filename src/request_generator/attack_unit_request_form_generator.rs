use serde_json::Value as JsonValue;
use crate::game_card_unit::controller::request_form::attack_unit_request_form::AttackUnitRequestForm;

pub fn create_attack_unit_request_form(data: &JsonValue) -> Option<AttackUnitRequestForm> {
    if let (Some(sessionInfo), Some(attacker_unit_index), Some(target_unit_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("attacker_unit_index").and_then(|v| v.as_str()),
        data.get("target_unit_index").and_then(|v| v.as_str()),
    ) {
        Some(AttackUnitRequestForm::new(
            sessionInfo.to_string(),
            attacker_unit_index.to_string(),
            target_unit_index.to_string()))
    } else {
        None
    }
}
