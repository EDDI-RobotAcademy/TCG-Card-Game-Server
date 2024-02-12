use serde_json::Value as JsonValue;

use crate::game_card_support::controller::request_form::remove_opponent_field_energy_support_request_form::RemoveOpponentFieldEnergySupportRequestForm;

pub fn create_opponent_field_energy_remove_support_request_form(data: &JsonValue) -> Option<RemoveOpponentFieldEnergySupportRequestForm> {
    if let (Some(session_id), Some(support_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str()),
    ) {
        Some(RemoveOpponentFieldEnergySupportRequestForm::new(session_id, support_card_id))
    } else {
        None
    }
}