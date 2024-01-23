use async_trait::async_trait;

use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;

#[async_trait]
pub trait AccountDeckService {
    async fn account_deck_register(&self, account_deck_register_request: AccountDeckRegisterRequest) -> AccountDeckRegisterResponse;
}