use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub trait GameCardSupportRepository {
    unsafe fn call_support_card_repository_table(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect;
    unsafe fn call_support_card_repository_handler(&self, support_card_id: i32) -> GameCardSupportEffect;
}