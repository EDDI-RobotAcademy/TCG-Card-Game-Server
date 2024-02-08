use async_trait::async_trait;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::service::request::calculate_item_effect_request::CalculateItemEffectRequest;

#[async_trait]
pub trait GameCardItemService {
    async fn use_item_card(&mut self, calculate_item_effect_request: CalculateItemEffectRequest) -> GameCardItemEffect;
}