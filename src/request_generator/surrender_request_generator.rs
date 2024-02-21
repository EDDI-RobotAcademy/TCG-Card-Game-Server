use serde_json::Value as JsonValue;
use crate::game_winner_check::service::request::surrender_request::SurrenderRequest;

pub fn create_surrender_request(data: &JsonValue) -> Option<SurrenderRequest> {
    if let (Some(sessionInfo),) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(SurrenderRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}