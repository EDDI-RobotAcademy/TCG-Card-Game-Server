use serde_json::Value as JsonValue;
use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;

pub fn create_register_request(data: &JsonValue) -> Option<AccountDeckRegisterRequest> {
    if let (Some(account_id), Some(deck_name)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("deck_name").and_then(|v| v.as_str()),
    ) {
        Some(AccountDeckRegisterRequest::new(account_id.to_string(), deck_name.to_string()))
    } else {
        None
    }
}