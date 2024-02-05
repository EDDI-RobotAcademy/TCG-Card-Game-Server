use serde_json::Value as JsonValue;

use crate::shop::service::request::free_card_request::FreeCardRequest;
use crate::shop::service::request::get_card_default_request::GetCardDefaultRequest;

pub fn create_free_card_request(data: &JsonValue) -> Option<FreeCardRequest> {
    if let Some(account_session_id) =
        data.get("sessionInfo").and_then(|v| v.as_str()) {
        Some(FreeCardRequest::new(account_session_id.to_string()))
    } else {
        None
    }
}

pub fn create_get_card_default_request(data: &JsonValue) -> Option<GetCardDefaultRequest> {
    if let (Some(account_session_id), Some(race_name)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("race_name").and_then(|v| v.as_str()) )
    {
        Some(GetCardDefaultRequest::new(account_session_id.to_string(), race_name.to_string()))
    } else {
        None
    }
}