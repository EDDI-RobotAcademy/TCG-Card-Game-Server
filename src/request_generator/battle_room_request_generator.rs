use serde_json::Value as JsonValue;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;

pub fn create_battle_match_request(data: &JsonValue) -> Option<BattleMatchRequest> {
    if let ((Some(sessionId),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleMatchRequest::new(sessionId.to_string()))
    } else {
        None
    }
}

