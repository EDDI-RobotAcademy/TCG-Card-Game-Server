use serde_json::Value as JsonValue;

use crate::game_hand::controller::request_form::use_game_hand_energy_card_request_form::UseGameHandEnergyCardRequestForm;

pub fn create_use_hand_energy_request_form(data: &JsonValue) -> Option<UseGameHandEnergyCardRequestForm> {
    if let (Some(unit_number), Some(session_info), Some(energy_card_id)) = (
        data.get("unitId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("energyCardId").and_then(|v| v.as_str()),
    ) {
        Some(UseGameHandEnergyCardRequestForm::new(session_info.to_string(), unit_number.to_string(), energy_card_id.to_string()))
    } else {
        None
    }
}
