use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;

pub trait GameCardSupportHandler: Send {
    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect;
}