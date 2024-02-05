use serde_json::Value as JsonValue;

use crate::game_hand::controller::request_form::use_game_hand_unit_card_request_form::UseGameHandUnitCardRequestForm;

pub fn create_use_hand_unit_request_form(data: &JsonValue) -> Option<UseGameHandUnitCardRequestForm> {
    if let (Some(unit_number), Some(session_info)) = (
        data.get("unitId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(UseGameHandUnitCardRequestForm::new(session_info.to_string(), unit_number.to_string()))
    } else {
        None
    }
}
