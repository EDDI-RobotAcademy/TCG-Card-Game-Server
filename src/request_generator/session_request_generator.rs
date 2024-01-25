use serde_json::Value as JsonValue;
use crate::account::service::request::account_session_login_request::AccountSessionLoginRequest;
use crate::account::service::request::account_session_logout_request::AccountSessionLogoutRequest;

pub fn create_session_login_request(data: &JsonValue) -> Option<AccountSessionLoginRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(AccountSessionLoginRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}

pub fn create_session_logout_request(data: &JsonValue) -> Option<AccountSessionLogoutRequest> {
    if let ((Some(sessionInfo),)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(AccountSessionLogoutRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}
