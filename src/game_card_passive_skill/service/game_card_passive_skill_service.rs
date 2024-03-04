use async_trait::async_trait;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_card_passive_skill::service::request::summary_deploy_passive_skill_effect_request::SummaryDeployPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_by_index_request::SummaryPassiveSkillEffectByIndexRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_request::SummaryPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::request::summary_turn_start_passive_skill_effect_request::SummaryTurnStartPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::response::summary_deploy_passive_skill_effect_response::SummaryDeployPassiveSkillEffectResponse;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_by_index_response::SummaryPassiveSkillEffectByIndexResponse;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_response::SummaryPassiveSkillEffectResponse;
use crate::game_card_passive_skill::service::response::summary_turn_start_passive_skill_effect_response::SummaryTurnStartPassiveSkillEffectResponse;

#[async_trait]
pub trait GameCardPassiveSkillService {
    async fn summary_passive_skill(&self, summary_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse;
    async fn summary_deploy_passive_skill(&self, summary_deploy_passive_skill_effect_request: SummaryDeployPassiveSkillEffectRequest) -> SummaryDeployPassiveSkillEffectResponse;
    async fn summary_turn_start_passive_skill(&self, summary_turn_start_passive_skill_effect_request: SummaryTurnStartPassiveSkillEffectRequest) -> SummaryTurnStartPassiveSkillEffectResponse;
    async fn summary_passive_skill_by_index(&self, summary_passive_skill_effect_by_index_request: SummaryPassiveSkillEffectByIndexRequest) -> SummaryPassiveSkillEffectByIndexResponse;

    }
