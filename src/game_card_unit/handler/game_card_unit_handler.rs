use crate::game_card_unit::entity::game_card_unit_effect::GameCardUnitEffect;

pub trait GameCardUnitHandler: Send {
    unsafe fn use_specific_unit_card(&self) -> GameCardUnitEffect;
}