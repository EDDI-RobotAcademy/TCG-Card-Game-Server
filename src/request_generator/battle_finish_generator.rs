use serde_json::Value as JsonValue;
use crate::battle_finish::service::request::battle_finish_request::BattleFinishRequest;

pub fn create_battle_finish_request(data: &JsonValue) -> Option<BattleFinishRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleFinishRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}
