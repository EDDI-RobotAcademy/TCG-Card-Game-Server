use serde_json::Value as JsonValue;
use crate::battle_start::controller::request_form::battle_start_request_form::BattleStartRequestForm;

pub fn create_battle_start_request_form(data: &JsonValue) -> Option<BattleStartRequestForm> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(BattleStartRequestForm::new(sessionInfo.to_string()))
    } else {
        None
    }
}

