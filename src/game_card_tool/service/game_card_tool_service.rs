use async_trait::async_trait;
use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;
use crate::game_card_tool::service::request::summarize_tool_card_effect_request::SummarizeToolCardEffectRequest;

#[async_trait]
pub trait GameCardToolService {
    async fn summarize_tool_card_effect(&mut self, summarize_tool_card_effect_request: SummarizeToolCardEffectRequest) -> GameCardToolEffect;
}