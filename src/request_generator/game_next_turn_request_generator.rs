use serde_json::Value as JsonValue;
use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;

pub fn create_game_turn_request_form(data: &JsonValue) -> Option<TurnEndRequestForm> {
    if let (Some(sessionInfo),) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(TurnEndRequestForm::new(sessionInfo.to_string()))
    } else {
        None
    }
}