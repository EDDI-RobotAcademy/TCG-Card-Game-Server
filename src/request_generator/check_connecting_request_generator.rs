use serde_json::Value as JsonValue;
use crate::check_connecting::service::request::checked_response_request::CheckedResponseRequest;

pub fn create_check_connecting_request(data: &JsonValue) -> Option<CheckedResponseRequest> {
    if let (Some(address),) = (
        data.get("address").and_then(|v| v.as_str()),
    ) {
        Some(CheckedResponseRequest::new(address))
    } else {
        None
    }
}