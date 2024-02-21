use async_trait::async_trait;

use crate::account_card::service::request::account_card_list_request::AccountCardListRequest;
use crate::account_card::service::request::update_account_card_db_request::UpdateAccountCardDbRequest;
use crate::account_card::service::response::account_card_list_response::AccountCardListResponse;
use crate::account_card::service::response::update_account_card_db_response::UpdateAccountCardDbResponse;


#[async_trait]
pub trait AccountCardService {
    async fn account_card_list(&self, account_card_list_request: AccountCardListRequest) -> AccountCardListResponse;
    async fn update_account_card_db(&self, update_account_card_db_request: UpdateAccountCardDbRequest) -> UpdateAccountCardDbResponse;
}