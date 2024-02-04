use serde_json::Value as JsonValue;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

pub fn create_use_hand_unit_request(data: &JsonValue) -> Option<UseGameHandUnitCardRequest> {
    if let (Some(deck_id), Some(session_info)) = (
        data.get("deckId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(UseGameHandUnitCardRequest::new(deck_id.to_string(), session_info.to_string()))
    } else {
        None
    }
}
