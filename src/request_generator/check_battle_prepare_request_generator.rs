use serde_json::Value as JsonValue;
use crate::battle_ready_account_hash::service::request::check_battle_prepare_request::CheckBattlePrepareRequest;


pub fn create_check_battle_prepare_request(data: &JsonValue) -> Option<CheckBattlePrepareRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(CheckBattlePrepareRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

