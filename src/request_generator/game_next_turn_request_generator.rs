use serde_json::Value as JsonValue;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;

pub fn create_game_turn_request_form(data: &JsonValue) -> Option<NextTurnRequest> {
    if let (Some(account_id),) = (
        data.get("accountId").and_then(|v| v.as_i64()),
    ) {
        let account_id_i32 = account_id as i32;
        Some(NextTurnRequest::new(account_id_i32))
    } else {
        None
    }
}