use async_trait::async_trait;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::service::request::calculate_item_effect_request::CalculateItemEffectRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;

#[async_trait]
pub trait GameCardItemService {
    async fn use_item_card(&mut self, calculate_item_effect_request: CalculateItemEffectRequest) -> GameCardItemEffect;
    async fn summary_item_card(&mut self, summary_item_card_effect_request: SummaryItemCardEffectRequest) -> SummaryItemCardEffectResponse;
}