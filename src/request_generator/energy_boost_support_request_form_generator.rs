use serde_json::Value as JsonValue;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;

pub fn create_energy_boost_support_request_form(data: &JsonValue) -> Option<EnergyBoostSupportRequestForm> {
    if let (Some(unit_index_number), Some(session_info), Some(support_card_id)) = (
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str()),
    ) {
        Some(EnergyBoostSupportRequestForm::new(
            session_info.to_string(),
            unit_index_number.to_string(),
            support_card_id.to_string()))
    } else {
        None
    }
}
