use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;

pub trait GameCardItemHandler: Send {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect;
}
