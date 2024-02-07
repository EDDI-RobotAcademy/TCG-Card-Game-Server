use async_trait::async_trait;
use crate::game_card_unit::entity::game_card_unit_effect::GameCardUnitEffect;

#[async_trait]
pub trait GameCardUnitRepository {
    unsafe fn call_unit_card_repository_handler(&self, unit_card_id: i32) -> GameCardUnitEffect;
}