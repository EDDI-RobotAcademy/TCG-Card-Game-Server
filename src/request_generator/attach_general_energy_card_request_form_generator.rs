use serde_json::Value as JsonValue;
use crate::game_card_energy::controller::request_form::attach_general_energy_card_request_form::AttachGeneralEnergyCardRequestForm;

use crate::game_hand::controller::request_form::use_game_hand_energy_card_request_form::UseGameHandEnergyCardRequestForm;

pub fn create_attach_general_energy_card_request_form(data: &JsonValue) -> Option<AttachGeneralEnergyCardRequestForm> {
    if let (Some(unit_index), Some(session_info), Some(energy_card_id)) = (
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("energyCardId").and_then(|v| v.as_str()),
    ) {
        Some(AttachGeneralEnergyCardRequestForm::new(session_info.to_string(), unit_index.to_string(), energy_card_id.to_string()))
    } else {
        None
    }
}
