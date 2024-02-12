use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;

pub trait GameCardSupportHandler: Send {
    unsafe fn generate_support_card_effect_summary(&self) -> GameCardSupportEffect;
}