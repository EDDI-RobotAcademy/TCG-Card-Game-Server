use async_trait::async_trait;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_request::SummaryPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_response::SummaryPassiveSkillEffectResponse;

#[async_trait]
pub trait GameCardPassiveSkillService {
    async fn summary_passive_skill(&self, summary_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse;
    async fn summary_deploy_passive_skill(&self, summary_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse;
    async fn summary_turn_start_passive_skill(&self, summary_deploy_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse;
    }
