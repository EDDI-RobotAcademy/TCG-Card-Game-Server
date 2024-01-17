use serde_json::Value as JsonValue;
use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::account::service::request::account_register_request::AccountRegisterRequest;

pub fn create_register_request(data: &JsonValue) -> Option<AccountRegisterRequest> {
    if let (Some(username), Some(password)) = (
        data.get("userId").and_then(|v| v.as_str()),
        data.get("password").and_then(|v| v.as_str()),
    ) {
        Some(AccountRegisterRequest::new(username, password.to_string()))
    } else {
        None
    }
}

pub fn create_login_request(data: &JsonValue) -> Option<AccountLoginRequest> {
    if let (Some(username), Some(password)) = (
        data.get("userId").and_then(|v| v.as_str()),
        data.get("password").and_then(|v| v.as_str()),
    ) {
        Some(AccountLoginRequest::new(username, password.to_string()))
    } else {
        None
    }
}