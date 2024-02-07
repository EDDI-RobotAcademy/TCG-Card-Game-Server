use async_trait::async_trait;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::request::calculate_effect_request::CalculateEffectRequest;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;

#[async_trait]
pub trait GameCardSupportService {
    async fn use_specific_support_card(&mut self, request: UseSupportCardRequest) -> UseSupportCardResponse;
    async fn use_support_card(&mut self, calculate_effect_request: CalculateEffectRequest) -> GameCardSupportEffect;
}