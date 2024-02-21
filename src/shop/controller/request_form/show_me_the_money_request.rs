use crate::account_point::service::request::gain_gold_request::GainGoldRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct ShowMeTheMoneyRequest {
    account_session_id: String,
}

impl ShowMeTheMoneyRequest {
    pub fn new(account_session_id: String) -> Self {
        ShowMeTheMoneyRequest { account_session_id: account_session_id.to_string() }
    }
    pub fn account_session_id(&self) -> &str { &self.account_session_id }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.account_session_id.clone().as_str())
    }
    pub fn to_gain_gold_request(&self, account_unique_id: i32, gold: i32) -> GainGoldRequest {
        GainGoldRequest::new(account_unique_id, gold)
    }
}