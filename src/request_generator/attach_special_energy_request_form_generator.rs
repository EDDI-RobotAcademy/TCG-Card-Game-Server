use serde_json::Value as JsonValue;
use crate::game_card_energy::controller::request_form::attach_special_energy_card_request_form::AttachSpecialEnergyCardRequestForm;

pub fn create_attach_special_energy_card_request_form(data: &JsonValue) -> Option<AttachSpecialEnergyCardRequestForm> {
    if let (Some(session_info), Some(unit_index), Some(energy_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("energyCardId").and_then(|v| v.as_str()),
    ) {
        Some(AttachSpecialEnergyCardRequestForm::new(session_info.to_string(), unit_index.to_string(), energy_card_id.to_string()))
    } else {
        None
    }
}