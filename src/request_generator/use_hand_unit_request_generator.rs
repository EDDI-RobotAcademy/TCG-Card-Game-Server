use serde_json::Value as JsonValue;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

pub fn create_use_hand_unit_request(data: &JsonValue) -> Option<UseGameHandUnitCardRequest> {
    if let (Some(unit_number), Some(session_info)) = (
        data.get("unitId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(UseGameHandUnitCardRequest::new(session_info.to_string(), unit_number.to_string()))
    } else {
        None
    }
}
