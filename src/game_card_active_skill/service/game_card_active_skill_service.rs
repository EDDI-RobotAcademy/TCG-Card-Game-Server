use async_trait::async_trait;
use crate::game_card_active_skill::service::request::summary_active_skill_effect_request::SummaryActiveSkillEffectRequest;
use crate::game_card_active_skill::service::response::summary_active_skill_effect_response::SummaryActiveSkillEffectResponse;

#[async_trait]
pub trait GameCardActiveSkillService {
    async fn summary_active_skill(&mut self, summary_active_skill_effect_request: SummaryActiveSkillEffectRequest) -> SummaryActiveSkillEffectResponse;
}
