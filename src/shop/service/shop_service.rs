use async_trait::async_trait;
use crate::shop::service::request::free_card_request::FreeCardRequest;
use crate::shop::service::request::get_card_default_request::GetCardDefaultRequest;
use crate::shop::service::response::free_card_response::FreeCardResponse;
use crate::shop::service::response::get_card_default_response::GetCardDefaultResponse;

#[async_trait]
pub trait ShopService {
    // async fn free_card(&self, free_card_request: FreeCardRequest) -> FreeCardResponse;
    async fn get_specific_card_default(&self, get_card_default_request: GetCardDefaultRequest) -> GetCardDefaultResponse ;

}