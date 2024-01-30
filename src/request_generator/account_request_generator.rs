use serde_json::Value as JsonValue;
use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::request::account_logout_request::AccountLogoutRequest;
use crate::account::service::request::account_modify_request::AccountModifyRequest;

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

pub fn create_logout_request(data: &JsonValue) -> Option<AccountLogoutRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(AccountLogoutRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}
pub fn create_account_modify_request(data: &JsonValue) -> Option<AccountModifyRequest> {
    if let (Some(username), Some(password), Some(new_password)) = (
        data.get("userId").and_then(|v| v.as_str()),
        data.get("password").and_then(|v| v.as_str()),
        data.get("new_password").and_then(|v| v.as_str()),
    ) {
        Some(AccountModifyRequest::new(username, password.to_string(), new_password.to_string()))
    } else {
        None
    }
}