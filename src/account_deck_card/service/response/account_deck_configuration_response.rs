use serde::{Deserialize, Serialize};

use crate::account_deck_card::controller::response_form::account_deck_configuration_response_form::AccountDeckConfigurationResponseForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckConfigurationResponse {
    is_success: bool,
    message: String
}

impl AccountDeckConfigurationResponse {
    pub fn new(is_success: bool, message: String) -> Self { AccountDeckConfigurationResponse { is_success, message } }
    pub fn to_account_deck_configuration_response_form(&self) -> AccountDeckConfigurationResponseForm {
        AccountDeckConfigurationResponseForm::new( self.is_success, self.message.clone() )
    }
    pub fn get_is_success(&self) -> bool { self.is_success }
}