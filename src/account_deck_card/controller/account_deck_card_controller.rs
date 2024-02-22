use async_trait::async_trait;

use crate::account_deck_card::controller::request_form::account_deck_configuration_request_form::AccountDeckConfigurationRequestForm;
use crate::account_deck_card::controller::response_form::account_deck_configuration_response_form::AccountDeckConfigurationResponseForm;

use crate::account_deck_card::controller::request_form::account_deck_card_list_request_form::AccountDeckCardListRequestFrom;
use crate::account_deck_card::controller::request_form::account_deck_card_modify_request_form::AccountDeckCardModifyRequestForm;
use crate::account_deck_card::controller::response_form::account_deck_card_list_response_form::AccountDeckCardListResponseForm;
use crate::account_deck_card::controller::response_form::account_deck_card_modify_response_form::AccountDeckCardModifyResponseForm;

#[async_trait]
pub trait AccountDeckCardController {
    async fn deck_configuration_register(
        &self, account_deck_configuration_request_form: AccountDeckConfigurationRequestForm) -> AccountDeckConfigurationResponseForm;
    async fn deck_card_list(
        &self, account_deck_card_list_request_from: AccountDeckCardListRequestFrom) -> AccountDeckCardListResponseForm;
    async fn deck_card_modify(
        &self, account_deck_card_modify_request_form: AccountDeckCardModifyRequestForm) -> AccountDeckCardModifyResponseForm;
}