use serde_json::Value as JsonValue;
use crate::game_card_item::controller::request_form::remove_opponent_field_energy_item_request_form::RemoveOpponentFieldEnergyItemRequestForm;


pub fn create_opponent_field_energy_remove_item_request_form(data: &JsonValue) -> Option<RemoveOpponentFieldEnergyItemRequestForm> {
    if let (Some(session_id), Some(item_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
    ) {
        Some(RemoveOpponentFieldEnergyItemRequestForm::new(session_id, item_card_id))
    } else {
        None
    }
}