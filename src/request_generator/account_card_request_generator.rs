use serde_json::Value as JsonValue;

use crate::account_card::service::request::account_card_list_request::AccountCardListRequest;

pub fn create_account_card_list_request(data: &JsonValue) -> Option<AccountCardListRequest> {
    if let Some(account_session_id) =
        data.get("sessionInfo").and_then(|v| v.as_str()) {
        Some(AccountCardListRequest::new(account_session_id.to_string()))
    } else {
        None
    }
}