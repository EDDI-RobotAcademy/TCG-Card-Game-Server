use async_trait::async_trait;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;

#[async_trait]
pub trait GameCardPassiveSkillRepository {
    unsafe fn call_passive_skill_repository_handler(&self, unit_card_id: i32, skill_index: i32) -> SummaryPassiveSkillEffect;
    unsafe fn call_deploy_passive_skill_repository_handler(&self, unit_card_id:i32, skill_index: i32) -> SummaryPassiveSkillEffect;
    unsafe fn call_turn_start_passive_skill_repository_handler(&self, unit_card_id:i32, skill_index: i32) -> SummaryPassiveSkillEffect ;
    }
