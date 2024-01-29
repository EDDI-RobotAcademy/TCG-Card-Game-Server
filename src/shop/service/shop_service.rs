use async_trait::async_trait;
use crate::shop::service::request::free_card_request::FreeCardRequest;
use crate::shop::service::response::free_card_response::FreeCardResponse;

#[async_trait]
pub trait ShopService {
    async fn free_card(&self, free_card_request: FreeCardRequest) -> FreeCardResponse;
}