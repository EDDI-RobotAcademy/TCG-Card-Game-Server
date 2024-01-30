use serde_json::Value as JsonValue;
use crate::battle_ready_account_hash::service::request::battle_ready_account_hash_request::BattleReadyAccountHashRequest;
use crate::battle_ready_monitor::service::request::battle_ready_request::BattleReadyRequest;

pub fn create_battle_ready_account_hash_request(data: &JsonValue) -> Option<BattleReadyAccountHashRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleReadyAccountHashRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

