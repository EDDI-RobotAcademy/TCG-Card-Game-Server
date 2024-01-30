use async_trait::async_trait;

use crate::account_card::service::request::account_card_list_request::AccountCardListRequest;
use crate::account_card::service::response::account_card_list_response::AccountCardListResponse;



#[async_trait]
pub trait AccountCardService {
    async fn account_card_list(&self, account_card_list_request: AccountCardListRequest) -> AccountCardListResponse;
}