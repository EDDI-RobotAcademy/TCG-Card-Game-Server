use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;

pub trait GameCardUnitHandler: Send {
    unsafe fn summary_unit_card(&self) -> GameCardUnitInfo;
    unsafe fn summary_unit_card_passive_default(&self) -> Vec<bool>;
}