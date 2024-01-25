use async_trait::async_trait;

use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::account_deck::service::request::account_deck_list_request::AccountDeckListRequest;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;


#[async_trait]
pub trait AccountDeckService {
    async fn account_deck_register(&self, account_deck_register_request: AccountDeckRegisterRequest) -> AccountDeckRegisterResponse;
    async fn account_deck_list(&self, account_deck_list_request: AccountDeckListRequest) -> AccountDeckListResponse;
}