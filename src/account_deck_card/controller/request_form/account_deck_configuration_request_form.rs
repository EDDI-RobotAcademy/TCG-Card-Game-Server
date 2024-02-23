use crate::account_deck_card::service::request::account_deck_configuration_request::AccountDeckConfigurationRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct AccountDeckConfigurationRequestForm {
    deck_id_form: i32,
    card_id_list_form: Vec<i32>,
    account_session_id: String
}

impl AccountDeckConfigurationRequestForm {
    pub fn new(deck_id_form: i32, card_id_list_form: Vec<i32>, account_session_id: String) -> Self {
        AccountDeckConfigurationRequestForm {
            deck_id_form,
            card_id_list_form,
            account_session_id
        }
    }
    pub fn deck_id_form(&self) -> i32 { self.deck_id_form }
    pub fn card_id_list_form(&self) -> Vec<i32> { self.card_id_list_form.clone() }
    pub fn account_session_id(&self) -> &str { &self.account_session_id }
    pub fn to_account_deck_configuration_request(&self) -> AccountDeckConfigurationRequest {
        AccountDeckConfigurationRequest::new( self.deck_id_form, self.card_id_list_form.clone() )
    }
    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.account_session_id.clone().as_str())
    }
}