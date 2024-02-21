use crate::account_card::service::request::update_account_card_db_request::UpdateAccountCardDbRequest;
use crate::account_deck::service::request::account_deck_list_request::AccountDeckListRequest;
use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck_card::service::request::account_deck_configuration_request::AccountDeckConfigurationRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct EventDistributeCardsRequestForm {
    account_session_id: String,
}
impl EventDistributeCardsRequestForm {
    pub fn new(account_session_id: String) -> Self {
        EventDistributeCardsRequestForm {
            account_session_id,
        }
    }

    pub fn account_session_id(&self) -> &str { &self.account_session_id }
    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.account_session_id.clone().as_str())
    }
    pub fn to_update_account_card_db_request(&self, account_unique_id: i32, update_card_list: Vec<i32> ) -> UpdateAccountCardDbRequest {
        UpdateAccountCardDbRequest::new(account_unique_id, update_card_list)
    }
    pub fn to_account_deck_register_request(&self, account_session_id: String, deck_name: String ) -> AccountDeckRegisterRequest {
        AccountDeckRegisterRequest::new(account_session_id, deck_name)
    }
    pub fn to_account_deck_list_request(&self, account_session_id: String) -> AccountDeckListRequest {
        AccountDeckListRequest::new(account_session_id)
    }
    pub fn to_account_deck_configuration_register_request(&self, deck_id: i32, card_list: Vec<i32>) -> AccountDeckConfigurationRequest {
        AccountDeckConfigurationRequest::new(deck_id, card_list)
    }
}