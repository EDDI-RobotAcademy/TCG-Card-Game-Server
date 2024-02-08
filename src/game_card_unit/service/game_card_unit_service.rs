use async_trait::async_trait;
use crate::game_card_unit::entity::game_card_unit_effect::GameCardUnitEffect;
use crate::game_card_unit::service::request::calculate_unit_effect_request::CalculateUnitEffectRequest;

#[async_trait]
pub trait GameCardUnitService {
    async fn use_unit_card(&mut self, calculate_unit_effect_request: CalculateUnitEffectRequest) -> GameCardUnitEffect;
}