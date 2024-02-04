use serde_json::Value as JsonValue;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;

pub fn create_use_hand_energy_request(data: &JsonValue) -> Option<UseGameHandEnergyCardRequest> {
    if let (Some(unit_number), Some(session_info), Some(energy_card_id)) = (
        data.get("unitId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("energyCardId").and_then(|v| v.as_str()),
    ) {
        Some(UseGameHandEnergyCardRequest::new(session_info.to_string(), unit_number.to_string(), energy_card_id.to_string()))
    } else {
        None
    }
}
