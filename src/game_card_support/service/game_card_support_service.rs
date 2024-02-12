use async_trait::async_trait;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;

#[async_trait]
pub trait GameCardSupportService {
    async fn use_support_card(&mut self, summarize_support_card_effect_request: SummarizeSupportCardEffectRequest) -> GameCardSupportEffect;
}