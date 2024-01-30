use serde_json::Value as JsonValue;

use crate::shop::service::request::free_card_request::FreeCardRequest;

pub fn create_free_card_request(data: &JsonValue) -> Option<FreeCardRequest> {
    if let Some(account_session_id) =
        data.get("sessionInfo").and_then(|v| v.as_str()) {
        Some(FreeCardRequest::new(account_session_id.to_string()))
    } else {
        None
    }
}