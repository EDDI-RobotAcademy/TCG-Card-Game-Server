use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;
use crate::game_card_active_skill::entity::required_energy::RequiredEnergy;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;

pub struct SummaryActiveSkillEffectResponse {
    required_energy: RequiredEnergy,
    skill_type: ActiveSkillType,
    skill_damage: i32,
}

impl SummaryActiveSkillEffectResponse {
    pub fn new(
        required_energy: RequiredEnergy,
        skill_type: ActiveSkillType,
        skill_damage: i32,) -> Self {

        Self {
            required_energy,
            skill_type,
            skill_damage }
    }

    pub fn get_required_energy(&self) -> &RequiredEnergy { &self.required_energy }
    pub fn get_skill_type(&self) -> &ActiveSkillType {
        &self.skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }

    pub fn from_summary_active_skill_effect(summary_active_skill_effect: SummaryActiveSkillEffect) -> SummaryActiveSkillEffectResponse {

        let required_energy_count = summary_active_skill_effect.get_required_energy_for_active_skill().get_required_energy_count();
        let required_energy_race = *summary_active_skill_effect.get_required_energy_for_active_skill().get_required_energy_race();

        SummaryActiveSkillEffectResponse::new(
            RequiredEnergy::new(required_energy_race, required_energy_count),
            summary_active_skill_effect.get_skill_type().clone(),
            summary_active_skill_effect.get_skill_damage())
    }

}