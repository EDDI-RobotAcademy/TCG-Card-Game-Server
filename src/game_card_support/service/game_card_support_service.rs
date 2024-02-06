use async_trait::async_trait;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;

#[async_trait]
pub trait GameCardSupportService {
    async fn use_specific_support_card(&mut self, request: UseSupportCardRequest) -> UseSupportCardResponse;
}