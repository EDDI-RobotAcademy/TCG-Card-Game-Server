use serde_json::Value as JsonValue;
use crate::battle_wait_queue::service::request::battle_match_cancel_request::BattleMatchCancelRequest;

pub fn create_battle_match_cancel_request(data: &JsonValue) -> Option<BattleMatchCancelRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleMatchCancelRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

