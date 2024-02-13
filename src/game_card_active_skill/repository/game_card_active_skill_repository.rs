use async_trait::async_trait;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

#[async_trait]
pub trait GameCardActiveSkillRepository {
    unsafe fn call_active_skill_repository_handler(&self, unit_card_id: i32, skill_index: i32) -> SummaryActiveSkillEffect;
}
