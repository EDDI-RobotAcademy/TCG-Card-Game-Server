use async_trait::async_trait;
use crate::account_deck::service::request::account_deck_delete_request::AccountDeckDeleteRequest;

use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::account_deck::service::request::account_deck_list_request::AccountDeckListRequest;
use crate::account_deck::service::request::account_deck_modify_request::AccountDeckModifyRequest;
use crate::account_deck::service::response::account_deck_delete_response::AccountDeckDeleteResponse;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;
use crate::account_deck::service::response::account_deck_modify_response::AccountDeckModifyResponse;


#[async_trait]
pub trait AccountDeckService {
    async fn account_deck_register(&self, account_deck_register_request: AccountDeckRegisterRequest) -> AccountDeckRegisterResponse;
    async fn account_deck_list(&self, account_deck_list_request: AccountDeckListRequest) -> AccountDeckListResponse;
    async fn account_deck_modify(&self, account_deck_modify_request: AccountDeckModifyRequest) -> AccountDeckModifyResponse;
    async fn account_deck_delete(&self, account_deck_delete_request: AccountDeckDeleteRequest) -> AccountDeckDeleteResponse;
}