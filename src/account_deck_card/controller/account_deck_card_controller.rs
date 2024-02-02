use async_trait::async_trait;

use crate::account_deck_card::controller::request::account_deck_card_list_request::AccountDeckCardListRequest;
use crate::account_deck_card::controller::request::account_deck_configuration_request::AccountDeckConfigurationRequest;

use crate::account_deck_card::controller::response::account_deck_card_list_response::AccountDeckCardListResponse;
use crate::account_deck_card::controller::response::account_deck_configuration_response::AccountDeckConfigurationResponse;

#[async_trait]
pub trait AccountDeckCardController {
    async fn deck_configuration_register(
        &self, deck_configuration_request: AccountDeckConfigurationRequest) -> AccountDeckConfigurationResponse;
    async fn deck_card_list(
        &self, deck_card_list_request: AccountDeckCardListRequest) -> AccountDeckCardListResponse;
}