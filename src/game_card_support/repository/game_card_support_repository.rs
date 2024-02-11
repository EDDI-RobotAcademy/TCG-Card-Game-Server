use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;

pub trait GameCardSupportRepository {
    unsafe fn call_support_card_repository_handler(&self, support_card_id: i32) -> GameCardSupportEffect;
}