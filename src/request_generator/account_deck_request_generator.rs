use serde_json::Value as JsonValue;

use crate::account_deck::service::request::account_deck_list_request::AccountDeckListRequest;
use crate::account_deck::service::request::account_deck_modify_request::AccountDeckModifyRequest;
use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;

pub fn create_deck_register_request(data: &JsonValue) -> Option<AccountDeckRegisterRequest> {
    if let (Some(account_id), Some(deck_name)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("deckName").and_then(|v| v.as_str()),
    ) {
        Some(AccountDeckRegisterRequest::new(account_id.to_string(), deck_name.to_string()))
    } else {
        None
    }
}

pub fn create_deck_list_request(data: &JsonValue) -> Option<AccountDeckListRequest> {
    if let Some(account_id) = data.get("sessionInfo").and_then(|v| v.as_str()) {
        Some(AccountDeckListRequest::new(account_id.to_string()))
    } else {
        None
    }
}

pub fn create_deck_modify_request(data: &JsonValue) -> Option<AccountDeckModifyRequest> {
    if let (Some(deck_id), Some(account_id), Some(deck_name)) = (
        data.get("deckId").and_then(|v| v.as_i64()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("deckName").and_then(|v| v.as_str())
    ) {
        let deck_id_opt_i32 = deck_id as i32;
        Some(AccountDeckModifyRequest::new(deck_id_opt_i32, account_id.to_string(), deck_name.to_string()))
    } else {
        None
    }
}