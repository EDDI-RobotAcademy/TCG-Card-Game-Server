use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub trait GameCardSupportHandler {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect;
}