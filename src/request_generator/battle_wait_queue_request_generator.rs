use serde_json::Value as JsonValue;
use crate::battle_wait_queue::service::request::battle_wait_queue_request::BattleWaitQueueRequest;

pub fn create_battle_wait_queue_request(data: &JsonValue) -> Option<BattleWaitQueueRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleWaitQueueRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

