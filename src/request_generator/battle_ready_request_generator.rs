use serde_json::Value as JsonValue;
use crate::battle_ready_monitor::service::request::battle_ready_request::BattleReadyRequest;

pub fn create_battle_ready_request(data: &JsonValue) -> Option<BattleReadyRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleReadyRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

